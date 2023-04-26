//! A frame is the combination of a `Step` and an `Executor`. It wraps common behavior used by different chain types.
//!
//! This module provides the `Frame` struct, which is a core component in the process of creating customized chains.
//! The `Frame` struct is designed to simplify the process of interacting with `Step` and `Executor` traits, allowing
//! developers to focus on implementing the desired functionality without worrying about the boilerplate code.
//!
//! The `Frame` struct is generic over the `Step` and `Executor` types, ensuring that it can work with any
//! combination of types that implement the required traits.

use crate::step::Step;
use crate::traits;
use crate::traits::ExecutorError;
use crate::Parameters;

/// The `Frame` struct represents a combination of a `Step` and an `Executor`.
///
/// It is designed to provide a simple interface for working with different chain types and handling common
/// behavior for formatting and executing steps.
pub struct Frame<'l, E>
where
    E: traits::Executor,
{
    executor: &'l E,
    step: &'l Step<E>,
}

impl<'l, E> Frame<'l, E>
where
    E: traits::Executor,
{
    /// Constructs a new `Frame` with the given `Executor` and `Step`.
    ///
    /// The `new` function takes two references to an `Executor` and a `Step`, respectively, and returns
    /// a new `Frame` instance.
    pub fn new(executor: &'l E, step: &'l Step<E>) -> Self {
        Self { executor, step }
    }

    /// Formats the step with the provided parameters and executes it using the associated executor.
    ///
    /// This function takes a reference to a `Parameters` struct, formats the step with the provided parameters,
    /// and executes it using the associated executor. The result of the execution is returned as `E::Output`.
    pub async fn format_and_execute(
        &self,
        parameters: &Parameters,
    ) -> Result<E::Output, FormatAndExecuteError<E::Error>> {
        let prompt = self.step.format(parameters)?;
        Ok(self.executor.execute(self.step.options(), &prompt).await?)
    }
}

#[derive(Debug, thiserror::Error)]
/// An error that occurs when formatting and prompt template for an LLM
pub enum FormatAndExecuteError<E: ExecutorError> {
    #[error("Error formatting: {0}")]
    Format(#[from] crate::prompt::StringTemplateError),
    #[error("Error executing: {0}")]
    Execute(#[from] E),
}
