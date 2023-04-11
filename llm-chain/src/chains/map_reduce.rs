//! The `map_reduce` module contains the `Chain` struct, which represents a map-reduce chain.
//!
//! A map-reduce chain is a combination of two steps - a `map` step and a `reduce` step.
//! The `map` step processes each input document and the `reduce` step combines the results
//! of the `map` step into a single output.
//!
//! The `Chain` struct is generic over the type of the `Step` and provides a convenient way
//! to execute map-reduce operations using a provided `Executor`.

use crate::{
    frame::Frame,
    serialization::StorableEntity,
    tokens::ExecutorTokenCountExt,
    tokens::PromptTokensError,
    traits::{Executor, Step},
    Parameters,
};
use futures::future::join_all;
#[cfg(feature = "serialization")]
use serde::{
    de::{MapAccess, Visitor},
    Deserialize,
};

use thiserror::Error;

/// The `MapReduceChainError` enum represents errors that can occur when executing a map-reduce chain.
#[derive(Error, Debug)]
pub enum MapReduceChainError {
    /// An error relating to tokenizing the inputs.
    #[error("TokenizeError: {0}")]
    TokenizeError(#[from] crate::tokens::PromptTokensError),
    #[error("The vector of input documents was empty")]
    InputEmpty,
}

/// The `Chain` struct represents a map-reduce chain, consisting of a `map` step and a `reduce` step.
///
/// The struct is generic over the type of the `Step` and provides methods for constructing and
/// executing the chain using a given `Executor`.
pub struct Chain<S: Step> {
    map: S,
    reduce: S,
}

impl<S: Step> Chain<S> {
    /// Constructs a new `Chain` with the given `map` and `reduce` steps.
    ///
    /// The `new` function takes two instances of `Step` and returns a new `Chain` instance.
    pub fn new(map: S, reduce: S) -> Chain<S> {
        Chain { map, reduce }
    }

    /// Executes the map-reduce chain using the provided `Executor`.
    ///
    /// The `run` function takes a vector of input documents, a base set of parameters, and a reference
    /// to an `Executor`. It processes the input documents using the `map` step and the `reduce` step,
    /// and returns the result as an `Option<E::Output>`.
    ///
    /// The function is asynchronous and must be awaited.
    pub async fn run<E>(
        &self,
        documents: Vec<Parameters>,
        base_parameters: Parameters,
        executor: &E,
    ) -> Result<E::Output, MapReduceChainError>
    where
        E: Executor<Step = S>,
    {
        if documents.is_empty() {
            return Err(MapReduceChainError::InputEmpty);
        }
        let map_frame = Frame::new(executor, &self.map);
        let reduce_frame = Frame::new(executor, &self.reduce);

        let chunked_docs =
            self.chunk_documents::<E, E::Token>(documents.clone(), executor, &self.map)?;

        // Execute the `map` step for each document, combining the base parameters with each document's parameters.
        let chunked_docs_with_base_parameters: Vec<_> = chunked_docs
            .iter()
            .map(|doc| base_parameters.combine(doc))
            .collect();
        let futures: Vec<_> = chunked_docs_with_base_parameters
            .iter()
            .map(|doc| map_frame.format_and_execute(doc))
            .collect();
        let mapped_documents = join_all(futures).await;

        let mut documents = self.combine_documents_up_to::<E, E::Token>(
            executor,
            mapped_documents,
            &base_parameters,
        )?;

        if documents.is_empty() {
            return Err(MapReduceChainError::InputEmpty);
        }

        loop {
            let tasks: Vec<_> = documents
                .iter()
                .map(|doc| E::apply_output_to_parameters(base_parameters.clone(), doc))
                .collect();
            let futures = tasks.iter().map(|p| reduce_frame.format_and_execute(p));
            let new_docs = join_all(futures).await;
            let n_new_docs = new_docs.len();
            documents =
                self.combine_documents_up_to::<E, E::Token>(executor, new_docs, &base_parameters)?;
            if n_new_docs == 1 {
                break;
            }
        }
        // At this point there is exactly one document.
        assert_eq!(documents.len(), 1);
        let output = documents.pop().unwrap();
        Ok(output)
    }

    fn combine_documents_up_to<E, T>(
        &self,
        executor: &E,
        mut v: Vec<<E as Executor>::Output>,
        parameters: &Parameters,
    ) -> Result<Vec<<E as Executor>::Output>, MapReduceChainError>
    where
        E: Executor<Step = S>,
    {
        let mut new_outputs = Vec::new();
        while let Some(current) = v.pop() {
            let mut current_doc = current;
            while let Some(next) = v.last() {
                let new_doc = E::combine_outputs(&current_doc, next);
                let params = E::apply_output_to_parameters(parameters.clone(), &new_doc);
                let count = executor.tokens_used(&self.reduce, &params)?;
                if count.has_tokens_remaining() {
                    current_doc = new_doc;
                    v.pop();
                } else {
                    break;
                }
            }
            new_outputs.push(current_doc);
        }
        Ok(new_outputs)
    }

    fn chunk_documents<E, T>(
        &self,
        v: Vec<Parameters>,
        executor: &E,
        step: &S,
    ) -> Result<Vec<Parameters>, PromptTokensError>
    where
        E: Executor<Step = S>,
    {
        let data: Result<Vec<_>, _> = v.iter().map(|x| executor.split_to_fit(step, x)).collect();
        let data = data?.iter().flatten().cloned().collect();
        Ok(data)
    }
}

#[cfg(feature = "serialization")]
impl<'de, S: Step + Deserialize<'de>> Deserialize<'de> for Chain<S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ChainVisitor<S>(std::marker::PhantomData<S>);

        impl<'de, S: Step + Deserialize<'de>> Visitor<'de> for ChainVisitor<S> {
            type Value = Chain<S>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an object with fields `map` and `reduce`")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut map_value: Option<S> = None;
                let mut reduce_value: Option<S> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "map" => {
                            if map_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("map"));
                            }
                            map_value = Some(map.next_value()?);
                        }
                        "reduce" => {
                            if reduce_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("reduce"));
                            }
                            reduce_value = Some(map.next_value()?);
                        }
                        _ => (),
                    }
                }

                let map = map_value.ok_or_else(|| serde::de::Error::missing_field("map"))?;
                let reduce =
                    reduce_value.ok_or_else(|| serde::de::Error::missing_field("reduce"))?;
                Ok(Chain { map, reduce })
            }
        }

        deserializer.deserialize_struct(
            "Chain",
            &["map", "reduce"],
            ChainVisitor(std::marker::PhantomData),
        )
    }
}

/// Implements the `StorableEntity` trait for the `Chain` struct.
///
/// This implementation provides a method for extracting metadata from a `Chain` instance, in order to identify it
impl<S> StorableEntity for Chain<S>
where
    S: Step + StorableEntity,
{
    /// Returns metadata about the Chain instance.
    ///
    /// The metadata is returned as a vector of tuples, where each tuple contains a key-value pair
    /// representing a metadata field and its value.
    ///
    /// This method also extracts metadata from the Step instances associated with the Chain.
    fn get_metadata() -> Vec<(String, String)> {
        let mut base = vec![(
            "chain-type".to_string(),
            "llm-chain::chains::map_reduce::Chain".to_string(),
        )];
        base.append(&mut S::get_metadata());
        base
    }
}
