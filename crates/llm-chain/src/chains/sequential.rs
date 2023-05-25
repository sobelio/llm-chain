//! A module for implementing a sequential chain of LLM steps.
//!
//! This module provides the `Chain` struct, which represents a sequential chain of steps for Large Language Models (LLMs). Each step in the chain is executed in order, and the output of the previous step is available as input to the next step.
//!
//! The `Chain` struct allows you to:
//! - Create a new chain with a vector of `Step` instances
//! - Execute the chain with a given set of `Parameters` and an `Executor`
//!
//! The `Chain` struct is designed to work with any executor that implements the `Executor` trait, providing flexibility and extensibility.
//!
//! # Example
//!
//! ```ignore
//!
//! // Assuming an executor `executor` that implements the `Executor` trait.
//! let step1 = Step::new(prompt!("Write a summary for this text: {{text}}"));
//! let step2 = Step::new(prompt!("{{text}}\n\nWrite a tweet thread for the above summary");
//!
//! let chain = Chain::new(vec![step1, step2]);
//!
//! let parameters = parameters!("your input text here")
//!
//! // Execute the chain with the provided parameters and executor.
//! let result = chain.run(parameters, &executor).await;
//! ```
//!
//! This module also provides serialization and deserialization support for the `Chain` struct, allowing you to store and load chains using formats like JSON, YAML, or others.

use serde::{Deserialize, Serialize};

use crate::frame::FormatAndExecuteError;
use crate::output::Output;
use crate::{
    frame::Frame, serialization::StorableEntity, step::Step, traits::Executor, Parameters,
};

#[derive(thiserror::Error, Debug)]

/// The `SequentialChainError` enum represents errors that can occur when executing a sequential chain.
pub enum SequentialChainError {
    #[error("ExecutorError: {0}")]
    FormatAndExecuteError(#[from] FormatAndExecuteError),
    #[error("The vector of steps was empty")]
    NoSteps,
}

/// A sequential chain is a chain where each step is executed in order, with the output of the previous step being available to the next step.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chain {
    steps: Vec<Step>,
}

impl Chain {
    /// Creates a new `Chain` instance with the given sequence of steps.
    ///
    /// # Arguments
    ///
    /// * `steps` - A vector of `Step<E>` objects that define the sequence of steps for the chain.
    pub fn new(steps: Vec<Step>) -> Chain {
        Chain { steps }
    }

    /// Creates a new `Chain` instance with a single step.
    ///
    /// # Arguments
    ///
    /// * `step` - A `Step<E>` object that defines the single step for the chain.
    pub fn of_one(step: Step) -> Chain {
        Chain { steps: vec![step] }
    }

    /// Executes the chain with the given parameters and executor.
    ///
    /// This method runs each step in the chain in sequence, passing the output of the previous step to the next step.
    /// If the chain is empty, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `parameters` - A `Parameters` object containing the input parameters for the chain.
    /// * `executor` - A reference to an executor that implements the `Executor` trait.
    ///
    /// # Returns
    ///
    /// * `Ok(E::Output)` - If the chain executes successfully, the output of the last step is returned.
    /// * `Err(SequentialChainError<E::Error>)` - If an error occurs during the execution of the chain, the error is returned.
    pub async fn run<E>(
        &self,
        parameters: Parameters,
        executor: &E,
    ) -> Result<Output, SequentialChainError>
    where
        E: Executor,
    {
        if self.steps.is_empty() {
            return Err(SequentialChainError::NoSteps);
        }
        let mut current_params = parameters;

        for step in &self.steps[..self.steps.len() - 1] {
            let body = Frame::new(executor, step)
                .format_and_execute(&current_params)
                .await?
                .to_immediate()
                .await
                .map_err(|err| {
                    SequentialChainError::FormatAndExecuteError(FormatAndExecuteError::Execute(err))
                })?
                .as_content()
                .extract_last_body()
                .cloned()
                .unwrap_or_default();
            current_params = current_params.with_text(body);
        }
        let last_step = self.steps.last().unwrap();
        Ok(Frame::new(executor, last_step)
            .format_and_execute(&current_params)
            .await?)
    }
}

impl StorableEntity for Chain {
    fn get_metadata() -> Vec<(String, String)> {
        let base = vec![(
            "chain-type".to_string(),
            "llm-chain::chains::sequential::Chain".to_string(),
        )];
        base
    }
}
