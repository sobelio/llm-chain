#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::{
    frame::Frame,
    serialization::StorableEntity,
    step::Step,
    traits::{Executor, ExecutorError},
    Parameters,
};

#[derive(thiserror::Error, Debug)]
pub enum SequentialChainError<Err: ExecutorError> {
    #[error("ExecutorError: {0}")]
    ExecutorError(#[from] Err),
    #[error("The vector of steps was empty")]
    NoSteps,
}
// A sequential chain is a chain where each step is executed in order, with the output of the previous being available to the next.
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Chain<E: Executor> {
    steps: Vec<Step<E>>,
}

impl<E: Executor> Chain<E> {
    pub fn new(steps: Vec<Step<E>>) -> Chain<E> {
        Chain { steps }
    }
    pub fn of_one(step: Step<E>) -> Chain<E> {
        Chain { steps: vec![step] }
    }

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
        for step in self.steps.iter() {
            let frame = Frame::new(executor, step);
            let res = frame.format_and_execute(&current_params).await?;

            current_params = current_params.with_text_from_output(&res).await;
            output = Some(res);
        }
        Ok(output.expect("No output from chain"))
    }
}

impl<E: Executor> StorableEntity for Step<E> {
    fn get_metadata() -> Vec<(String, String)> {
        let base = vec![(
            "chain-type".to_string(),
            "llm-chain::chains::sequential::Chain".to_string(),
        )];
        base
    }
}
