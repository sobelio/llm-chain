//! The `map_reduce` module contains the `Chain` struct, which represents a map-reduce chain.
//!
//! A map-reduce chain is a combination of two steps - a `map` step and a `reduce` step.
//! The `map` step processes each input document and the `reduce` step combines the results
//! of the `map` step into a single output.
//!
//! The `Chain` struct is generic over the type of the `Step` and provides a convenient way
//! to execute map-reduce operations using a provided `Executor`.

use crate::traits::ExecutorError;
use crate::{
    frame::Frame, output::Output, prompt::Data, serialization::StorableEntity, step::Step, tokens,
    tokens::PromptTokensError, traits::Executor, Parameters,
};
use futures::future::join_all;
use futures::future::FutureExt;
use serde::Deserialize;
use serde::Serialize;

use thiserror::Error;

/// The `MapReduceChainError` enum represents errors that can occur when executing a map-reduce chain.
#[derive(Error, Debug)]
pub enum MapReduceChainError {
    /// An error relating to the operation of the Executor.
    #[error("FormatAndExecuteError: {0}")]
    FormatAndExecuteError(#[from] crate::frame::FormatAndExecuteError),
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
#[derive(Serialize, Deserialize)]
pub struct Chain {
    map: Step,
    reduce: Step,
}

impl Chain {
    /// Constructs a new `Chain` with the given `map` and `reduce` steps.
    ///
    /// The `new` function takes two instances of `Step` and returns a new `Chain` instance.
    pub fn new(map: Step, reduce: Step) -> Chain {
        Chain { map, reduce }
    }

    /// Executes the map-reduce chain using the provided `Executor`.
    ///
    /// The `run` function takes a vector of input documents, a base set of parameters, and a reference
    /// to an `Executor`. It processes the input documents using the `map` step and the `reduce` step,
    /// and returns the result as an `Option<E::Output>`.
    ///
    /// The function is asynchronous and must be awaited.
    pub async fn run<E: Executor>(
        &self,
        documents: Vec<Parameters>,
        base_parameters: Parameters,
        executor: &E,
    ) -> Result<Output, MapReduceChainError> {
        if documents.is_empty() {
            return Err(MapReduceChainError::InputEmpty);
        }
        let map_frame = Frame::new(executor, &self.map);
        let reduce_frame = Frame::new(executor, &self.reduce);

        let chunked_docs = self.chunk_documents(
            documents.clone(),
            base_parameters.clone(),
            executor,
            &self.map,
        )?;

        // Execute the `map` step for each document, combining the base parameters with each document's parameters.
        let chunked_docs_with_base_parameters: Vec<_> = chunked_docs
            .iter()
            .map(|doc| base_parameters.combine(doc))
            .collect();
        let mapped_documents: Vec<_> = join_all(
            chunked_docs_with_base_parameters
                .iter()
                .map(|doc| map_frame.format_and_execute(doc))
                .collect::<Vec<_>>(),
        )
        .await;
        let mapped_documents = mapped_documents
            .into_iter()
            .collect::<Result<Vec<Output>, _>>()?;
        let mapped_documents: Vec<Result<Data<String>, ExecutorError>> = join_all(
            mapped_documents
                .into_iter()
                .map(|x| x.to_immediate().map(|x| x.map(|y| y.as_content())))
                .collect::<Vec<_>>(),
        )
        .await;
        let mapped_documents: Vec<Data<String>> = mapped_documents
            .into_iter()
            .collect::<Result<Vec<Data<String>>, ExecutorError>>()
            .map_err(|e| {
                MapReduceChainError::FormatAndExecuteError(
                    crate::frame::FormatAndExecuteError::Execute(e),
                )
            })?;

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
            let new_docs = join_all(
                new_docs
                    .into_iter()
                    .map(|x| x.to_immediate().map(|x| x.map(|y| y.as_content()))),
            )
            .await;
            let new_docs = new_docs
                .into_iter()
                .collect::<Result<Vec<Data<String>>, ExecutorError>>()
                .map_err(|e| {
                    MapReduceChainError::FormatAndExecuteError(
                        crate::frame::FormatAndExecuteError::Execute(e),
                    )
                })?;
            let n_new_docs = new_docs.len();
            if n_new_docs == 1 {
                return Ok(Output::new_immediate(new_docs[0].clone()));
            }
            documents = self
                .combine_documents_up_to(executor, new_docs, &base_parameters)
                .await?;
        }
    }

    async fn combine_documents_up_to<E: Executor>(
        &self,
        executor: &E,
        mut v: Vec<Data<String>>,
        parameters: &Parameters,
    ) -> Result<Vec<String>, MapReduceChainError> {
        let mut new_outputs = Vec::new();
        while let Some(current) = v.pop() {
            let mut current_doc = current.extract_last_body().cloned().unwrap_or_default();
            while let Some(next) = v.last() {
                let Some(next_doc_content) = next.extract_last_body() else {
                    continue;
                };
                let mut new_doc = current_doc.clone();
                new_doc.push('\n');
                new_doc.push_str(next_doc_content);

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

    fn chunk_documents<'a, E>(
        &self,
        v: Vec<Parameters>,
        base_parameters: Parameters,
        executor: &E,
        step: &Step,
    ) -> Result<Vec<Parameters>, PromptTokensError>
    where
        E: Executor + 'a,
    {
        let data: Result<Vec<_>, _> = v
            .iter()
            .map(|x| {
                <E as tokens::ExecutorTokenCountExt>::split_to_fit(
                    executor,
                    step,
                    x,
                    &base_parameters,
                    None,
                )
            })
            .collect();
        let data = data?.iter().flatten().cloned().collect();
        Ok(data)
    }
}

/// Implements the `StorableEntity` trait for the `Chain` struct.
///
/// This implementation provides a method for extracting metadata from a `Chain` instance, in order to identify it
impl StorableEntity for Chain {
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
