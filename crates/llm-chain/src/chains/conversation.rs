//! The `Chain` module models a conversation between an entity and an LLM.
//! It manages the conversation state and provides methods for sending messages and receiving responses.
//!
//! It relies on the `traits::Executor` trait to execute prompts and handle LLM interactions.

use crate::options::Options;
use crate::output::Output;
use crate::prompt::{ChatMessageCollection, Prompt, PromptTemplate, StringTemplateError};
use crate::step::Step;
use crate::tokens::{PromptTokensError, TokenizerError};
use crate::traits::{Executor, ExecutorError};
use crate::{parameters, Parameters};
use serde::{Deserialize, Serialize};

/// `Chain` represents a conversation between an entity and an LLM.
///
/// It holds the conversation state and provides methods for sending messages and receiving responses.
#[derive(Serialize, Deserialize, Default)]
pub struct Chain {
    state: ChatMessageCollection<String>,
}

impl Chain {
    /// Constructs a new `Chain` with the given conversation state.
    /// Self,
    /// # Arguments
    /// * `state` - The initial prompt state to use.
    pub fn new(state: PromptTemplate) -> Result<Chain, StringTemplateError> {
        state
            .format(&parameters!())
            .map(|state| state.to_chat())
            .map(|state| Self { state })
    }

    /// Constructs a new `Chain` with the given conversation state by passing a ChatMessageCollection<String> (clone).
    /// Self,
    /// # Arguments
    /// * `state` - The initial prompt state to use.
    pub fn new_with_message_collection(state: &ChatMessageCollection<String>) -> Chain {
        Self {
            state: state.clone(),
        }
    }

    /// Sends a message to the LLM and returns the response.
    ///
    /// This method sends a message to the LLM, adding it and the response to the internal state.
    ///
    /// # Arguments
    /// * `step` - The step to send.
    /// * `parameters` - The parameters to use when formatting the step.
    /// * `exec` - The executor to use.
    ///
    /// # Returns
    /// A `Result` containing the LLM's response as `E::Output` on success or an `Error` variant on failure.
    pub async fn send_message<E: Executor>(
        &mut self,
        step: Step,
        parameters: &Parameters,
        exec: &E,
    ) -> Result<Output, Error> {
        let fmt = step.format(parameters)?;
        self.send_message_raw(step.options(), &fmt, exec).await
    }

    /// Sends a message to the LLM and returns the response.
    ///
    /// This method takes a ready prompt and options and sends it to the LLM, adding it and the response to the internal state.
    ///
    /// # Arguments
    /// * `options` - The options to use when executing the prompt.
    /// * `prompt` - The prompt to send.
    /// * `exec` - The executor to use.
    ///
    /// # Returns
    /// A `Result` containing the LLM's response as `E::Output` on success or an `Error` variant on failure.
    pub async fn send_message_raw<E: Executor>(
        &mut self,
        options: &Options,
        prompt: &Prompt,
        exec: &E,
    ) -> Result<Output, Error> {
        let tok = exec.tokens_used(options, prompt)?;
        let tokens_remaining = tok.tokens_remaining();
        let tokenizer = exec.get_tokenizer(options)?;
        self.state.trim_context(&tokenizer, tokens_remaining)?;

        // Combine the conversation history with the new prompt.
        let prompt_with_history = Prompt::Chat(self.state.clone()).combine(prompt);

        // Execute the prompt and retrieve the LLM's response.
        let res = exec.execute(options, &prompt_with_history).await?;
        let content = res.to_immediate().await?.as_content().to_chat();

        self.state = prompt_with_history.to_chat();
        self.state.append(content.clone());

        Ok(Output::new_immediate(content.into()))
    }
}

/// An error type representing various errors that can occur while interacting with the `Chain`.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("PromptTokensError: {0}")]
    PromptTokens(#[from] PromptTokensError),
    #[error("TokenizerError: {0}")]
    Tokenizer(#[from] TokenizerError),
    #[error("ExecutorError: {0}")]
    Executor(#[from] ExecutorError),
    #[error("No model output")]
    NoModelOutput,
    #[error("StringTemplateError: {0}")]
    StringTemplate(#[from] crate::prompt::StringTemplateError),
}
