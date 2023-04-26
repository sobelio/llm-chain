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
    output::Output,
    serialization::StorableEntity,
    step::Step,
    tokens,
    tokens::PromptTokensError,
    traits,
    traits::{Executor, ExecutorError},
    Parameters,
};
use futures::future::join_all;
use serde::de::{Deserializer, MapAccess};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;

#[cfg(feature = "serialization")]
use crate::serialization::StorableEntity;

use thiserror::Error;

/// The `MapReduceChainError` enum represents errors that can occur when executing a map-reduce chain.
#[derive(Error, Debug)]
pub enum MapReduceChainError<Err: ExecutorError> {
    /// An error relating to the operation of the Executor.
    #[error("FormatAndExecuteError: {0}")]
    FormatAndExecuteError(#[from] crate::frame::FormatAndExecuteError<Err>),
    /// An error relating to tokenizing the inputs.
    #[error("TokenizeError: {0}")]
    TokenizeError(#[from] crate::tokens::PromptTokensError),
    #[error("The vector of input documents was empty")]
    InputEmpty,
    #[error("Error templating: {0}")]
    StringTemplate(#[from] crate::prompt::StringTemplateError),
}

/// The `Chain` struct represents a map-reduce chain, consisting of a `map` step and a `reduce` step.
///
/// The struct is generic over the type of the `Step` and provides methods for constructing and
/// executing the chain using a given `Executor`.
pub struct Chain<E: Executor> {
    map: Step<E>,
    reduce: Step<E>,
}

impl<E: Executor> Chain<E> {
    /// Constructs a new `Chain` with the given `map` and `reduce` steps.
    ///
    /// The `new` function takes two instances of `Step` and returns a new `Chain` instance.
    pub fn new(map: Step<E>, reduce: Step<E>) -> Chain<E> {
        Chain { map, reduce }
    }

    /// Executes the map-reduce chain using the provided `Executor`.
    ///
    /// The `run` function takes a vector of input documents, a base set of parameters, and a reference
    /// to an `Executor`. It processes the input documents using the `map` step and the `reduce` step,
    /// and returns the result as an `Option<E::Output>`.
    ///
    /// The function is asynchronous and must be awaited.
    pub async fn run(
        &self,
        documents: Vec<Parameters>,
        base_parameters: Parameters,
        executor: &E,
    ) -> Result<E::Output, MapReduceChainError<E::Error>> {
        if documents.is_empty() {
            return Err(MapReduceChainError::InputEmpty);
        }
        let map_frame = Frame::new(executor, &self.map);
        let reduce_frame = Frame::new(executor, &self.reduce);

        let chunked_docs = self.chunk_documents(documents.clone(), executor, &self.map)?;

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
        let mapped_documents = mapped_documents.into_iter().collect::<Result<_, _>>()?;

        let mut documents = self
            .combine_documents_up_to(executor, mapped_documents, &base_parameters)
            .await?;

        if documents.is_empty() {
            return Err(MapReduceChainError::InputEmpty);
        }

        loop {
            let tasks: Vec<_> = documents
                .iter()
                .map(|doc| base_parameters.with_text(doc))
                .collect();
            let futures = tasks.iter().map(|p| reduce_frame.format_and_execute(p));
            let new_docs = join_all(futures).await;
            let new_docs = new_docs.into_iter().collect::<Result<Vec<_>, _>>()?;
            let n_new_docs = new_docs.len();
            if n_new_docs == 1 {
                return Ok(new_docs[0].clone());
            }
            documents = self
                .combine_documents_up_to(executor, new_docs, &base_parameters)
                .await?;
        }
    }

    async fn combine_documents_up_to(
        &self,
        executor: &E,
        mut v: Vec<<E as Executor>::Output>,
        parameters: &Parameters,
    ) -> Result<Vec<String>, MapReduceChainError<E::Error>> {
        let mut new_outputs = Vec::new();
        while let Some(current) = v.pop() {
            let mut current_doc = current.primary_textual_output().await.unwrap_or_default();
            while let Some(next) = v.last() {
                let next_doc = next.primary_textual_output().await;
                if next_doc.is_none() {
                    continue;
                }
                let mut new_doc = current_doc.clone();
                new_doc.push('\n');
                new_doc.push_str(&next.primary_textual_output().await.unwrap_or_default());

                let params = parameters.with_text(new_doc.clone());
                let prompt = self.reduce.format(&params)?;
                let count = executor.tokens_used(self.reduce.options(), &prompt)?;
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

    fn chunk_documents<'a>(
        &self,
        v: Vec<Parameters>,
        executor: &E,
        step: &Step<E>,
    ) -> Result<Vec<Parameters>, PromptTokensError>
    where
        E: Executor + 'a,
    {
        let data: Result<Vec<_>, _> = v
            .iter()
            .map(|x| {
                <E as tokens::ExecutorTokenCountExt<
                    <E as traits::Executor>::Output,
                    <E as traits::Executor>::Token,
                    <E as traits::Executor>::StepTokenizer<'a>,
                >>::split_to_fit(executor, step, x, None)
            })
            .collect();
        let data = data?.iter().flatten().cloned().collect();
        Ok(data)
    }
}

// Your custom Serialize implementation for Chain
impl<E: Executor> Serialize for Chain<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Chain", 2)?;
        s.serialize_field("map", &self.map)?;
        s.serialize_field("reduce", &self.reduce)?;
        s.end()
    }
}

struct ChainVisitor<E: Executor>(std::marker::PhantomData<E>);

impl<'de, E: Executor> serde::de::Visitor<'de> for ChainVisitor<E> {
    type Value = Chain<E>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a struct containing map and reduce fields")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map_field: Option<Step<E>> = None;
        let mut reduce_field: Option<Step<E>> = None;

        while let Some(key) = map.next_key()? {
            match key {
                "map" => {
                    if map_field.is_some() {
                        return Err(serde::de::Error::duplicate_field("map"));
                    }
                    map_field = Some(map.next_value()?);
                }
                "reduce" => {
                    if reduce_field.is_some() {
                        return Err(serde::de::Error::duplicate_field("reduce"));
                    }
                    reduce_field = Some(map.next_value()?);
                }
                _ => return Err(serde::de::Error::unknown_field(key, FIELDS)),
            }
        }

        let map = map_field.ok_or_else(|| serde::de::Error::missing_field("map"))?;
        let reduce = reduce_field.ok_or_else(|| serde::de::Error::missing_field("reduce"))?;

        Ok(Chain { map, reduce })
    }
}

impl<'de, E: Executor> Deserialize<'de> for Chain<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Chain", FIELDS, ChainVisitor(std::marker::PhantomData))
    }
}

const FIELDS: &[&str] = &["map", "reduce"];

/// Implements the `StorableEntity` trait for the `Chain` struct.
///
/// This implementation provides a method for extracting metadata from a `Chain` instance, in order to identify it
impl<E: Executor> StorableEntity for Chain<E>
where
    E: Executor,
{
    /// Returns metadata about the Chain instance.
    ///
    /// The metadata is returned as a vector of tuples, where each tuple contains a key-value pair
    /// representing a metadata field and its value.
    ///
    /// This method also extracts metadata from the Step instances associated with the Chain.
    fn get_metadata() -> Vec<(String, String)> {
        let base = vec![(
            "chain-type".to_string(),
            "llm-chain::chains::map_reduce::Chain".to_string(),
        )];
        base
    }
}
