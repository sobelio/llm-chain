//! Welcome to the `traits` module! This is where llm-chain houses its public traits, which define the essential behavior of steps and executors. These traits are the backbone of our library and are used to implement a new model.
//!
//! Let's break it down:
//! - **Steps**: These are the building blocks that make up the chains. Steps define the parameters, including the prompt that is sent to the LLM.
//! - **Executors**: These are the workhorses that perform the steps. They take the output of a step and invokes the model on it to get the output.
//!
//! By implementing these traits, you can set up a new model and use it in your application. Your step, defines the input to the model, and your executor invokes the model and returns the output. The output of the executor is then passed to the next step in the chain, and so on.

use crate::{chains::sequential, Parameters};
use async_trait::async_trait;
use thiserror::Error;

// A step is a single step in a chain. It takes a set of parameters and returns a formatted prompt that can be used by an executor.
pub trait Step {
    type Output: Send;
    fn format(&self, parameters: &Parameters) -> Self::Output;
}

impl<T: ?Sized> StepExt for T where T: Step {}
pub trait StepExt: Step {
    fn to_chain(self) -> sequential::Chain<Self>
    where
        Self: Sized,
    {
        sequential::Chain::of_one(self)
    }
}

#[async_trait]
// An executor performs a single step in a chain. It takes a step, executes it, and returns the output.
pub trait Executor {
    type Step: Step;
    type Output: Send + Clone;
    async fn execute(&self, input: <<Self as Executor>::Step as Step>::Output) -> Self::Output;
    fn apply_output_to_parameters(parameters: Parameters, output: &Self::Output) -> Parameters;
    fn combine_outputs(output: &Self::Output, other: &Self::Output) -> Self::Output;
    fn combine_outputs_many(outputs: &[Self::Output]) -> Option<Self::Output> {
        if outputs.is_empty() {
            return None;
        }
        let (cur, rest) = outputs.split_first().unwrap();
        let mut cur = cur.clone();
        for output in rest {
            cur = Self::combine_outputs(&cur, output);
        }
        Some(cur)
    }
}

#[derive(Clone, Debug, Error)]
pub enum PromptTokensError {
    #[error("The prompt tokens are accessible for this type of step.")]
    NotAvailable,
    #[error("The prompt tokens could not be computed.")]
    UnableToCompute,
}

// A trait for executors that can count the number of prompt tokens in a step. Useful if the Step itself cannot count the number of prompt tokens.
pub trait ExecutorPromptTokens<Step>: Executor<Step = Step> {
    fn count_tokens_for_output(
        &self,
        step: &Step,
        output: &Self::Output,
    ) -> Result<usize, PromptTokensError>;
    fn count_tokens_for_doc(&self, step: &Step, doc: &str) -> Result<usize, PromptTokensError>;
    fn count_prompt_tokens(&self, step: &Step) -> Result<usize, PromptTokensError>;
    fn max_tokens(&self, step: &Step) -> Result<usize, PromptTokensError>;
    fn split_at_tokens(
        &self,
        step: &Step,
        doc: &str,
        tokens: usize,
    ) -> Result<(String, String), PromptTokensError>;
}
