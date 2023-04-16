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
    chains::sequential,
    output::Output,
    schema::{Document, EmptyMetadata},
    tokens::{PromptTokensError, TokenCount},
    Parameters,
};
use async_trait::async_trait;

/// Marker trait for errors in `Step` method. It is needed so the concrete Error can have a derived From<StepFormatError>
pub trait StepError {}
/// The `Step` trait represents a single step in a chain. It takes a set of parameters and returns a
/// formatted prompt that can be used by an executor.
pub trait Step {
    /// The output type produced by this step.
    type Output: Send;
    type Error: Send + Debug + Error + StepError;

    /// Formats the step given a set of parameters, returning a value that can be used by an
    /// executor.
    ///
    /// # Parameters
    ///
    /// * `parameters`: The parameters used to format the step.
    ///
    /// # Returns
    ///
    /// The formatted output of this step or an error if the step could not be formatted.
    fn format(&self, parameters: &Parameters) -> Result<Self::Output, Self::Error>;
}

impl<T: ?Sized> StepExt for T where T: Step {}

/// The `StepExt` trait extends the functionality of the `Step` trait, providing convenience
/// methods for working with steps.
#[async_trait]
pub trait StepExt: Step {
    /// Converts this step into a sequential chain with a single step.
    ///
    /// # Returns
    ///
    /// A sequential chain containing this step.
    fn to_chain(self) -> sequential::Chain<Self>
    where
        Self: Sized,
    {
        sequential::Chain::of_one(self)
    }
    async fn run<E: Executor<Step = Self> + Send + Sync>(
        &self,
        parameters: &Parameters,
        executor: &E,
    ) -> Result<E::Output, E::Error>
    where
        Self: Sized,
    {
        let output = self.format(parameters)?;
        executor.execute(output).await
    }
}

/// Marker trait for errors in `Executor` method. It is needed so the concrete Errors can have a derived From<ExecutorError>
pub trait ExecutorError {}

#[async_trait]
/// The `Executor` trait represents an executor that performs a single step in a chain. It takes a
/// step, executes it, and returns the output.
pub trait Executor {
    /// The step type that this executor works with.
    type Step: Step;

    /// The output type produced by this executor.
    type Output: Output;
    /// The error type produced by this executor.
    type Error: ExecutorError + Debug + Error + From<<Self::Step as Step>::Error>;

    /// The token type used by this executor.
    type Token;

    /// Executes the given input and returns the resulting output.
    ///
    /// # Parameters
    ///
    /// * `input`: The input value to execute, that is the output of the step.
    ///
    /// # Returns
    ///
    /// The output produced by the executor.
    async fn execute(
        &self,
        input: <<Self as Executor>::Step as Step>::Output,
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
        step: &Self::Step,
        parameters: &Parameters,
    ) -> Result<TokenCount, PromptTokensError>;

    /// Tokenizes a string based on the step.
    ///
    /// # Parameters
    ///
    /// * `step`: The step used for tokenization.
    /// * `doc`: The string to tokenize.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of tokens, or an error if there was a problem.
    fn tokenize_str(
        &self,
        step: &Self::Step,
        doc: &str,
    ) -> Result<Vec<Self::Token>, PromptTokensError>;

    /// Converts a slice of tokens into a string based on the step.
    ///
    /// # Parameters
    ///
    /// * `step`: The step used for conversion.
    /// * `tokens`: The slice of tokens to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing a string, or an error if there was a problem.
    fn to_string(
        &self,
        step: &Self::Step,
        tokens: &[Self::Token],
    ) -> Result<String, PromptTokensError>;
}

/// This marker trait is needed so the concrete VectorStore::Error can have a derived From<Embeddings::Error>
pub trait EmbeddingsError {}

#[async_trait]
pub trait Embeddings {
    type Error: Debug + Error + EmbeddingsError;
    async fn embed_texts(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, Self::Error>;
    async fn embed_query(&self, query: String) -> Result<Vec<f32>, Self::Error>;
}

#[async_trait]
pub trait VectorStore<E, M = EmptyMetadata>
where
    E: Embeddings,
{
    type Error: Debug + Error + From<<E as Embeddings>::Error>;
    async fn add_texts(&self, texts: Vec<String>) -> Result<Vec<String>, Self::Error>;
    async fn add_documents(&self, documents: Vec<Document<M>>) -> Result<Vec<String>, Self::Error>;
    async fn similarity_search(
        &self,
        query: String,
        limit: u32,
    ) -> Result<Vec<Document<M>>, Self::Error>;
}
