//! Steps are indivudaul LLM invocations in a chain. They are a combination of a prompt and a configuration.
//!
//! Steps are used to set the per-invocation settings for a prompt. Useful when you want to change the settings for a specific prompt in a chain.
use crate::frame::{FormatAndExecuteError, Frame};
use crate::prompt::{Prompt, StringTemplateError};
use crate::{chains::sequential, prompt, traits, Parameters};
use derive_builder;
use serde::de::{Deserialize, Deserializer, MapAccess};
use serde::ser::{Serialize, SerializeMap, Serializer};
#[derive(derive_builder::Builder, Debug, Clone)]
/// A step in a chain of LLM invocations. It is a combination of a prompt and a configuration.
pub struct Step<Executor>
where
    Executor: traits::Executor,
{
    pub(crate) prompt: prompt::PromptTemplate,
    pub(crate) options: Option<Executor::PerInvocationOptions>,
    pub(crate) is_streaming: Option<bool>,
}

impl<Executor> Step<Executor>
where
    Executor: traits::Executor,
{
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
    pub fn for_prompt_and_options(
        prompt: prompt::PromptTemplate,
        options: Executor::PerInvocationOptions,
    ) -> Self {
        Self {
            prompt,
            options: Some(options),
            is_streaming: None,
        }
    }
    pub fn prompt(&self) -> &prompt::PromptTemplate {
        &self.prompt
    }
    pub fn options(&self) -> Option<&Executor::PerInvocationOptions> {
        self.options.as_ref()
    }
    pub fn is_streaming(&self) -> Option<bool> {
        self.is_streaming
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
    pub async fn run(
        &self,
        parameters: &Parameters,
        executor: &Executor,
    ) -> Result<Executor::Output, FormatAndExecuteError<Executor::Error>>
    where
        Self: Sized,
    {
        Ok(Frame::new(executor, &self)
            .format_and_execute(parameters)
            .await?)
    }
}

// Your custom Serialize implementation for Step
impl<E: traits::Executor> Serialize for Step<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("prompt", &self.prompt)?;
        map.serialize_entry("options", &self.options)?;
        map.end()
    }
}

// Your custom Deserialize implementation for Step
struct StepVisitor<E: traits::Executor>(std::marker::PhantomData<E>);

impl<'de, E: traits::Executor> serde::de::Visitor<'de> for StepVisitor<E> {
    type Value = Step<E>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map with keys named 'prompt' and 'options'")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut prompt = None;
        let mut options = None;
        let mut is_streaming = None;
        while let Some(key) = map.next_key()? {
            match key {
                "prompt" => {
                    if prompt.is_some() {
                        return Err(serde::de::Error::duplicate_field("prompt"));
                    }
                    prompt = Some(map.next_value()?);
                }
                "options" => {
                    if options.is_some() {
                        return Err(serde::de::Error::duplicate_field("options"));
                    }
                    options = Some(map.next_value()?);
                }
                "is_streaming" => {
                    if is_streaming.is_some() {
                        return Err(serde::de::Error::duplicate_field("is_streaming"));
                    }
                    is_streaming = Some(map.next_value()?);
                }
                _ => return Err(serde::de::Error::unknown_field(key, &["prompt", "options"])),
            }
        }
        let prompt = prompt.ok_or_else(|| serde::de::Error::missing_field("prompt"))?;
        let options = options.ok_or_else(|| serde::de::Error::missing_field("options"))?;
        Ok(Step {
            prompt,
            options,
            is_streaming,
        })
    }
}

impl<'de, E: traits::Executor> Deserialize<'de> for Step<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(StepVisitor(std::marker::PhantomData))
    }
}
