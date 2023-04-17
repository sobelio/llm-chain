use crate::{prompt, traits};
use derive_builder;

#[derive(Builder)]
/// A step in a chain of LLM invocations. It is a combination of a prompt and a configuration.
pub struct Step<Executor, Prompt>
where
    Executor: traits::Executor,
    Prompt: prompt::Prompt,
{
    prompt: Prompt,
}

impl <Executor, Prompt> Step<Executor, Prompt>
where
    Executor: traits::Executor,
    Prompt: prompt::Prompt {
    pub fn format(&self, parameters: &Parameters) ->
}
