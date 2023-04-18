use crate::{chains::sequential, prompt, traits, Parameters};
use derive_builder;

#[derive(derive_builder::Builder, serde::Serialize, serde::Deserialize, Debug, Clone)]
/// A step in a chain of LLM invocations. It is a combination of a prompt and a configuration.
pub struct Step<Executor>
where
    Executor: traits::Executor,
{
    pub(crate) prompt: prompt::Prompt,
    pub(crate) options: Option<Executor::PerInvocationOptions>,
}

impl<Executor> Step<Executor>
where
    Executor: traits::Executor,
{
    pub fn for_prompt(prompt: prompt::Prompt) -> Self {
        Self {
            prompt,
            options: None,
        }
    }
    pub fn for_prompt_and_options(
        prompt: prompt::Prompt,
        options: Executor::PerInvocationOptions,
    ) -> Self {
        Self {
            prompt,
            options: Some(options),
        }
    }
    pub fn prompt(&self) -> &prompt::Prompt {
        &self.prompt
    }
    pub fn options(&self) -> Option<&Executor::PerInvocationOptions> {
        self.options.as_ref()
    }

    /// Converts this step into a sequential chain with a single step.
    ///
    /// # Returns
    ///
    /// A sequential chain containing this step.
    pub fn to_chain(self) -> sequential::Chain<Executor>
    where
        Self: Sized,
    {
        crate::chains::sequential::Chain::of_one(self)
    }
    pub async fn run(
        &self,
        parameters: &Parameters,
        executor: &Executor,
    ) -> Result<Executor::Output, Executor::Error>
    where
        Self: Sized,
    {
        executor.execute(self, parameters).await
    }
}

#[derive(thiserror::Error, Debug)]
#[error("StepError")]
pub struct StepError;
