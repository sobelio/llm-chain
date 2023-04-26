//! The output module provides representations of the output of a chain.
//!
//! This module provides an implementation of the `Output` trait for the
//! `CreateChatCompletionResponse` from ChatGPT's API
//!
//! The `Output` struct wraps the response, and provides human-readable
//! representation and easy access to the output choices.
//!
//! ## Example
//!
//! ```rust
//! use async_openai::types::CreateChatCompletionResponse;
//! use llm_chain::output::Output;
//! use llm_chain_openai::chatgpt::{Output as OpenAIOutput};
//!
//! fn handle_response(response: CreateChatCompletionResponse) {
//!     let output: OpenAIOutput = response.into();
//!     println!("Chat message: {}", output);
//! }
//! ```
//!
mod stream;

use async_openai::types::{ChatCompletionResponseStream, CreateChatCompletionResponse};
use async_trait::async_trait;
use llm_chain::output;
use std::fmt;
use stream::{ResponseStream, StreamWrapper};

/// Represents the output of a CreateChatCompletionResponse from OpenAI.
#[derive(Clone, Debug)]
pub enum OutputInner {
    Response(CreateChatCompletionResponse),
    Stream(StreamWrapper),
}

impl From<CreateChatCompletionResponse> for OutputInner {
    fn from(response: CreateChatCompletionResponse) -> Self {
        Self::Response(response)
    }
}

impl From<ChatCompletionResponseStream> for OutputInner {
    fn from(stream: ChatCompletionResponseStream) -> Self {
        Self::Stream(StreamWrapper::new(stream))
    }
}

/// Output wrapper for OpenAI API's response types.
#[derive(Clone, Debug)]
pub struct Output(OutputInner);

impl Output {
    pub fn as_stream(&self) -> Option<ResponseStream> {
        match &self.0 {
            OutputInner::Stream(wrapper) => Some(wrapper.inner()),
            _ => None,
        }
    }
}

/// Implement the Display trait to provide a human-readable representation of the Output.
impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            OutputInner::Response(response) => {
                write!(f, "{}", response.choices[0].message.content)
            }
            OutputInner::Stream(_) => {
                write!(
                    f,
                    "StreamWrapper: A wrapper around a ChatCompletionResponseStream"
                )
            }
        }
    }
}

/// Implement the Output trait required for LLM outputs
#[async_trait]
impl output::Output for Output {
    async fn primary_textual_output_choices(&self) -> Vec<String> {
        match &self.0 {
            OutputInner::Response(response) => response
                .choices
                .iter()
                .map(|choice| choice.message.content.clone())
                .collect(),
            OutputInner::Stream(stream) => stream.primary_textual_output_choices().await,
        }
    }
}

/// Implement From trait to allow conversion from OutputInner to Output.
impl From<OutputInner> for Output {
    fn from(response: OutputInner) -> Self {
        Self(response)
    }
}

/// Implement From trait to allow conversion from CreateChatCompletionResponse to Output.
impl From<CreateChatCompletionResponse> for Output {
    fn from(response: CreateChatCompletionResponse) -> Self {
        Self(response.into())
    }
}

/// Implement From trait to allow conversion from ChatCompletionResponseStream to Output.
impl From<ChatCompletionResponseStream> for Output {
    fn from(stream: ChatCompletionResponseStream) -> Self {
        Self(stream.into())
    }
}
