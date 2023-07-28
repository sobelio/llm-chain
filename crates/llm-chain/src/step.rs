//! Steps are indivudaul LLM invocations in a chain. They are a combination of a prompt and a configuration.
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

/// The types for before and after hooks. Parameters and Output are readonly currently.
pub type BeforeStepHook = fn(&Parameters) -> Result<(), String>;
pub type AfterStepHook = fn(&Output) -> Result<(), String>;

#[derive(derive_builder::Builder, Debug, Clone, Serialize, Deserialize)]
/// A step in a chain of LLM invocations. It is a combination of a prompt and a configuration.
pub struct Step {
    pub(crate) prompt: prompt::PromptTemplate,
    pub(crate) options: Options,
    #[serde(skip)]
    pub(crate) before: Option<BeforeStepHook>,
    #[serde(skip)]
    pub(crate) after: Option<AfterStepHook>,
}

impl Step {
    pub fn for_prompt_template(prompt: prompt::PromptTemplate) -> Self {
        Self {
            prompt,
            options: Options::empty().clone(),
            before: None,
            after: None,
        }
    }
    pub fn for_prompt_with_streaming(prompt: prompt::PromptTemplate) -> Self {
        let mut options = Options::builder();
        options.add_option(Opt::Stream(true));
        let options = options.build();
        Self {
            prompt,
            options,
            before: None,
            after: None,
        }
    }
    pub fn for_prompt_and_options(prompt: prompt::PromptTemplate, options: Options) -> Self {
        Self {
            prompt,
            options,
            before: None,
            after: None,
        }
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

    /// Add before and after hooks to the step.
    /// Before hook will be called before the parameters are fed to the prompt template.
    /// After hook will be called after the output for the step is generated.
    /// # Argument
    /// * before/after: the hook itself
    /// # Returns
    /// * Ok(()) on success and Err(String) on fail
    pub fn add_before_hook(&mut self, before: BeforeStepHook) {
        self.before = Some(before);
    }
    pub fn add_after_hook(&mut self, after: AfterStepHook) {
        self.after = Some(after);
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

#[cfg(test)]
mod tests {
    use super::*;
    // Tests for step
    #[test]
    fn test_add_step_hooks() {
        let mut step = Step::for_prompt_template(prompt!("Hello, world!"));
        assert_eq!(step.before, None);
        assert_eq!(step.after, None);

        fn dummy_fn(_: &Parameters) -> Result<(), String> {
            Ok(())
        }
        step.add_before_hook(dummy_fn);

        fn dummy_fn_with_error(_: &Output) -> Result<(), String> {
            Err("Exit with error".to_string())
        }
        step.add_after_hook(dummy_fn_with_error);
        assert_ne!(step.before, None);
        assert_ne!(step.after, None);
    }
}
