//! Steps are individual LLM invocations in a chain. They are a combination of a prompt and a configuration.
//!
//! Steps are used to set the per-invocation settings for a prompt. Useful when you want to change the settings for a specific prompt in a chain.
use crate::frame::{FormatAndExecuteError, Frame};
use crate::options::Opt;
use crate::options::Options;
use crate::output::Output;
use crate::prompt::{Prompt, StringTemplateError};
use crate::traits::Executor;
use crate::{chains::sequential, prompt, Parameters};

use serde::Deserialize;
use serde::Serialize;
#[derive(derive_builder::Builder, Debug, Clone, Serialize, Deserialize)]
/// A step in a chain of LLM invocations. It is a combination of a prompt and a configuration.
pub struct Step {
    pub(crate) prompt: prompt::PromptTemplate,
    pub(crate) options: Options,
}

impl Step {
    pub fn for_prompt_template(prompt: prompt::PromptTemplate) -> Self {
        Self {
            prompt,
            options: Options::empty().clone(),
        }
    }
    pub fn for_prompt_with_streaming(prompt: prompt::PromptTemplate) -> Self {
        let mut options = Options::builder();
        options.add_option(Opt::Stream(true));
        let options = options.build();
        Self { prompt, options }
    }
    pub fn for_prompt_and_options(prompt: prompt::PromptTemplate, options: Options) -> Self {
        Self { prompt, options }
    }
    pub fn prompt(&self) -> &prompt::PromptTemplate {
        &self.prompt
    }
    pub fn options(&self) -> &Options {
        &self.options
    }

    /// Converts this step into a sequential chain with a single step.
    ///
    /// # Returns
    ///
    /// A sequential chain containing this step.
    pub fn to_chain(self) -> sequential::Chain
    where
        Self: Sized,
    {
        crate::chains::sequential::Chain::of_one(self)
    }

    /// Formats the prompt for this step with the given parameters.
    pub fn format(&self, parameters: &Parameters) -> Result<Prompt, StringTemplateError> {
        self.prompt.format(parameters)
    }

    /// Executes the step with the given parameters and executor.
    /// # Arguments
    /// * `parameters` - A `Parameters` object containing the input parameters for the step.
    /// * `executor` - An executor to use to execute the step.
    /// # Returns
    /// The output of the executor.
    pub async fn run<E>(
        &self,
        parameters: &Parameters,
        executor: &E,
    ) -> Result<Output, FormatAndExecuteError>
    where
        Self: Sized,
        E: Executor,
    {
        Frame::new(executor, self)
            .format_and_execute(parameters)
            .await
    }
}
