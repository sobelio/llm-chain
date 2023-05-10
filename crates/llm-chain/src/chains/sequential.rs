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
use serde::de::{Deserializer, MapAccess};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};

use crate::frame::FormatAndExecuteError;
use crate::{
    frame::Frame,
    serialization::StorableEntity,
    step::Step,
    traits::{Executor, ExecutorError},
    Parameters,
};

#[cfg(feature = "serialization")]
use crate::serialization::StorableEntity;

#[derive(thiserror::Error, Debug)]

/// The `SequentialChainError` enum represents errors that can occur when executing a sequential chain.
pub enum SequentialChainError<Err: ExecutorError> {
    #[error("ExecutorError: {0}")]
    FormatAndExecuteError(#[from] FormatAndExecuteError<Err>),
    #[error("The vector of steps was empty")]
    NoSteps,
}

/// A sequential chain is a chain where each step is executed in order, with the output of the previous step being available to the next step.
#[derive(Clone, Debug)]
pub struct Chain<E: Executor> {
    steps: Vec<Step<E>>,
}

impl<E: Executor> Chain<E> {
    /// Creates a new `Chain` instance with the given sequence of steps.
    ///
    /// # Arguments
    ///
    /// * `steps` - A vector of `Step<E>` objects that define the sequence of steps for the chain.
    pub fn new(steps: Vec<Step<E>>) -> Chain<E> {
        Chain { steps }
    }

    /// Creates a new `Chain` instance with a single step.
    ///
    /// # Arguments
    ///
    /// * `step` - A `Step<E>` object that defines the single step for the chain.
    pub fn of_one(step: Step<E>) -> Chain<E> {
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
    pub async fn run(
        &self,
        parameters: Parameters,
        executor: &E,
    ) -> Result<E::Output, SequentialChainError<E::Error>> {
        if self.steps.is_empty() {
            return Err(SequentialChainError::NoSteps);
        }
        let mut current_params = parameters;
        let mut output: Option<E::Output> = None;
        for (i, step) in self.steps.iter().enumerate() {
            let frame = Frame::new(executor, step);
            let res = frame.format_and_execute(&current_params).await?;
            let is_streaming_and_last_step =
                step.is_streaming() == Some(true) && i == self.steps.len() - 1;
            if !is_streaming_and_last_step {
                current_params = current_params.with_text_from_output(&res).await;
            }
            output = Some(res);
        }
        Ok(output.expect("No output from chain"))
    }
}

impl<E: Executor> StorableEntity for Chain<E> {
    fn get_metadata() -> Vec<(String, String)> {
        let base = vec![(
            "chain-type".to_string(),
            "llm-chain::chains::sequential::Chain".to_string(),
        )];
        base
    }
}

impl<E: Executor> Serialize for Chain<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("steps", &self.steps)?;
        map.end()
    }
}

struct ChainVisitor<E: Executor>(std::marker::PhantomData<E>);

impl<'de, E: Executor> serde::de::Visitor<'de> for ChainVisitor<E> {
    type Value = Chain<E>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map with a key named 'steps'")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut steps = None;
        while let Some(key) = map.next_key()? {
            match key {
                "steps" => {
                    if steps.is_some() {
                        return Err(serde::de::Error::duplicate_field("steps"));
                    }
                    steps = Some(map.next_value()?);
                }
                _ => return Err(serde::de::Error::unknown_field(key, &["steps"])),
            }
        }
        let steps = steps.ok_or_else(|| serde::de::Error::missing_field("steps"))?;
        Ok(Chain { steps })
    }
}

impl<'de, E: Executor> Deserialize<'de> for Chain<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ChainVisitor(std::marker::PhantomData))
    }
}
