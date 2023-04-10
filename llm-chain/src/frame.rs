//! A frame is the combination of a `Step` and an `Executor`. It wraps common behavior used by different chain types.
//!
//! This module provides the `Frame` struct, which is a core component in the process of creating customized chains.
//! The `Frame` struct is designed to simplify the process of interacting with `Step` and `Executor` traits, allowing
//! developers to focus on implementing the desired functionality without worrying about the boilerplate code.
//!
//! The `Frame` struct is generic over the `Step` and `Executor` types, ensuring that it can work with any
//! combination of types that implement the required traits.

use crate::traits;
use crate::Parameters;

/// The `Frame` struct represents a combination of a `Step` and an `Executor`.
///
/// It is designed to provide a simple interface for working with different chain types and handling common
/// behavior for formatting and executing steps.
pub struct Frame<'l, E, S>
where
    E: traits::Executor<Step = S>,
    S: traits::Step,
{
    executor: &'l E,
    step: &'l S,
}

impl<'l, E, S> Frame<'l, E, S>
where
    E: traits::Executor<Step = S>,
    S: traits::Step,
{
    /// Constructs a new `Frame` with the given `Executor` and `Step`.
    ///
    /// The `new` function takes two references to an `Executor` and a `Step`, respectively, and returns
    /// a new `Frame` instance.
    pub fn new(executor: &'l E, step: &'l S) -> Self {
        Self { executor, step }
    }

    /// Formats the step with the provided parameters and executes it using the associated executor.
    ///
    /// This function takes a reference to a `Parameters` struct, formats the step with the provided parameters,
    /// and executes it using the associated executor. The result of the execution is returned as `E::Output`.
    pub async fn format_and_execute(&self, parameters: &Parameters) -> E::Output {
        let output = self.step.format(parameters);
        self.executor.execute(output).await
    }
}
