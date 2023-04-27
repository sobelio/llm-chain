//! # Traits Module
//!
//! Welcome to the `traits` module! This is where llm-chain houses its public traits, which define the essential behavior of steps and executors. These traits are the backbone of our library, and they provide the foundation for creating and working with different models in llm-chain.
//!
//! Here's a brief overview of the key concepts:
//! - **Steps**: These are the building blocks that make up the chains. Steps define the parameters, including the prompt that is sent to the LLM (Large Language Model).
//! - **Executors**: These are responsible for performing the steps. They take the output of a step, invoke the model with that input, and return the resulting output.
//!
//! By implementing these traits, you can set up a new model and use it in your application. Your step defines the input to the model, and your executor invokes the model and returns the output. The output of the executor is then passed to the next step in the chain, and so on.
//!

use std::{error::Error, fmt::Debug};

use crate::{
    output::Output,
    prompt::Prompt,
    schema::{Document, EmptyMetadata},
    tokens::{PromptTokensError, TokenCount, Tokenizer, TokenizerError},
    TextSplitter,
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[derive(thiserror::Error, Debug)]
#[error("unable to create executor")]
pub enum ExecutorCreationError {
    #[error("unable to create executor: {0}")]
    InnerError(#[from] Box<dyn Error + Send + Sync>),
    #[error("Field must be set: {0}")]
    FieldRequiredError(String),
}

/// Marker trait for errors in `Executor` method. It is needed so the concrete Errors can have a derived `From<ExecutorError>`
pub trait ExecutorError {}

/// The `Options` trait represents an options type that is used to customize the behavior of a step or executor.
pub trait Options: Clone + Send + Sync + Serialize + DeserializeOwned + Debug {}

#[async_trait]
/// The `Executor` trait represents an executor that performs a single step in a chain. It takes a
/// step, executes it, and returns the output.
pub trait Executor: Sized {
    /// The per-invocation options type used by this executor. These are the options you can send to each step.
    type PerInvocationOptions: Options;
    /// The per-executor options type used by this executor. These are the options you can send to the executor and can't be set per step.
    type PerExecutorOptions: Options;

    /// The output type produced by this executor.
    type Output: Output;
    /// The error type produced by this executor.
    type Error: ExecutorError + Debug + Error;

    /// The token type used by this executor.
    type Token: Clone;

    type StepTokenizer<'a>: Tokenizer<Self::Token>
    where
        Self: 'a;

    type TextSplitter<'a>: TextSplitter<Self::Token>
    where
        Self: 'a;

    /// Create a new executor with the given executor options and invocation options. If you don't need to set any options, you can use the `new` method instead.
    /// # Parameters
    /// * `executor_options`: The options to set for the executor.
    /// * `invocation_options`: The default options to set for each invocation.
    fn new_with_options(
        executor_options: Option<Self::PerExecutorOptions>,
        invocation_options: Option<Self::PerInvocationOptions>,
    ) -> Result<Self, ExecutorCreationError>;

    fn new() -> Result<Self, ExecutorCreationError> {
        Self::new_with_options(None, None)
    }

    async fn execute(
        &self,
        options: Option<&Self::PerInvocationOptions>,
        prompt: &Prompt,
    ) -> Result<Self::Output, Self::Error>;

    /// Calculates the number of tokens used by the step given a set of parameters.
    ///
    /// The step and the parameters together are used to form full prompt, which is then tokenized
    /// and the token count is returned.
    ///
    /// # Parameters
    ///
    /// * `step`: The step to calculate token usage for.
    /// * `parameters`: The parameters to plug into the step.
    ///
    /// # Returns
    ///
    /// A `Result` containing the token count, or an error if there was a problem.
    fn tokens_used(
        &self,
        options: Option<&Self::PerInvocationOptions>,
        prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError>;

    /// Returns the maximum number of input tokens allowed by the model used.
    ///
    /// # Parameters
    ///
    /// * `options`: The per-invocation options that affect the token allowance.
    ///
    /// # Returns
    /// The max token count for the step
    fn max_tokens_allowed(&self, options: Option<&Self::PerInvocationOptions>) -> i32;

    /// Creates a tokenizer, depending on the model used by `step`.
    ///
    /// # Parameters
    ///
    /// * `step`: The step to get an associated tokenizer for.
    ///
    /// # Returns
    ///

    /// A `Result` containing a tokenizer, or an error if there was a problem.
    fn get_tokenizer(
        &self,
        options: Option<&Self::PerInvocationOptions>,
    ) -> Result<Self::StepTokenizer<'_>, TokenizerError>;

    /// Creates a text splitter, depending on the model used by 'step'
    ///
    /// # Parameters
    ///
    /// * `step` The step to get an associated text splitter for.
    ///
    /// # Returns
    /// A `Result` containing a text splitter, or an error if there was a problem.
    fn get_text_splitter(
        &self,
        options: Option<&Self::PerInvocationOptions>,
    ) -> Result<Self::TextSplitter<'_>, Self::Error>;
}

/// This marker trait is needed so the concrete VectorStore::Error can have a derived From<Embeddings::Error>
pub trait EmbeddingsError {}

#[async_trait]
pub trait Embeddings {
    type Error: Debug + Error + EmbeddingsError;
    async fn embed_texts(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, Self::Error>;
    async fn embed_query(&self, query: String) -> Result<Vec<f32>, Self::Error>;
}

/// This marker trait is needed so users of VectorStore can derive From<VectorStore::Error>
pub trait VectorStoreError {}

#[async_trait]
pub trait VectorStore<E, M = EmptyMetadata>
where
    E: Embeddings,
{
    type Error: Debug + Error + VectorStoreError;
    async fn add_texts(&self, texts: Vec<String>) -> Result<Vec<String>, Self::Error>;
    async fn add_documents(&self, documents: Vec<Document<M>>) -> Result<Vec<String>, Self::Error>;
    async fn similarity_search(
        &self,
        query: String,
        limit: u32,
    ) -> Result<Vec<Document<M>>, Self::Error>;
}
