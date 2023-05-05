//! Steps are indivudaul LLM invocations in a chain. They are a combination of a prompt and a configuration.
//!
//! Steps are used to set the per-invocation settings for a prompt. Useful when you want to change the settings for a specific prompt in a chain.
use crate::frame::{FormatAndExecuteError, Frame};
use crate::prompt::{Prompt, StringTemplateError};
use crate::traits::{Executor, Options};
use crate::{chains::sequential, prompt, traits, Parameters};
use derive_builder;

use serde::{Deserialize, Serialize};
#[derive(derive_builder::Builder, Debug, Clone, Serialize, Deserialize)]
/// A step in a chain of LLM invocations. It is a combination of a prompt and a configuration.
pub struct Step {
    pub(crate) prompt: prompt::PromptTemplate,
    pub(crate) options: Option<Box<dyn traits::Options>>,
    pub(crate) is_streaming: Option<bool>,
}

impl Step {
    pub fn for_prompt_template(prompt: prompt::PromptTemplate) -> Self {
        Self {
            prompt,
            options: None,
            is_streaming: None,
        }
    }
    pub fn for_prompt_with_streaming(prompt: prompt::PromptTemplate) -> Self {
        Self {
            prompt,
            options: None,
            is_streaming: Some(true),
        }
    }
    pub fn for_prompt_and_options<O: Options>(prompt: prompt::PromptTemplate, options: O) -> Self {
        Self {
            prompt,
            options: Some(Box::new(options)),
            is_streaming: None,
        }
    }
    pub fn prompt(&self) -> &prompt::PromptTemplate {
        &self.prompt
    }
    pub fn options(&self) -> Option<&dyn Options> {
        let opts = self.options.as_ref()?;
        Some(opts.as_ref())
    }
    pub fn options_downcast<O: Options>(&self) -> Option<&O> {
        // Should we throw on typemismatch?
        self.options().and_then(|x| x.downcast_ref())
    }

    pub fn is_streaming(&self) -> Option<bool> {
        self.is_streaming
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
    pub async fn run<E: Executor>(
        &self,
        parameters: &Parameters,
        executor: &E,
    ) -> Result<E::Output, FormatAndExecuteError<E::Error>>
    where
        Self: Sized,
    {
        Ok(Frame::new(executor, &self)
            .format_and_execute(parameters)
            .await?)
    }
}
