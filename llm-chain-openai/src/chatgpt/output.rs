//! # Output Module
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
//! use llm_chain_openai::chatgpt::Output as OpenAIOutput;
//!
//! fn handle_response(response: CreateChatCompletionResponse) {
//!     let output: OpenAIOutput = response.into();
//!     println!("Chat message: {}", output);
//! }
//! ```
//!
use async_trait::async_trait;
use std::{fmt, ops::Deref};

use async_openai::types::CreateChatCompletionResponse;
use llm_chain::output;

/// Represents the output of a CreateChatCompletionResponse from OpenAI.
#[derive(Clone, Debug)]
pub struct Output(CreateChatCompletionResponse);

/// Implement the Display trait to provide a human-readable representation of the Output.
impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.choices[0].message.content)
    }
}

/// Implement the Output trait required for LLM outputs
#[async_trait]
impl output::Output for Output {
    async fn primary_textual_output_choices(&self) -> Vec<String> {
        self.0
            .choices
            .iter()
            .map(|choice| choice.message.content.clone())
            .collect()
    }
}

/// Implement the Deref trait to provide access to the underlying CreateChatCompletionResponse.
impl Deref for Output {
    type Target = CreateChatCompletionResponse;
    fn deref(&self) -> &CreateChatCompletionResponse {
        &self.0
    }
}

/// Implement From trait to allow conversion from CreateChatCompletionResponse to Output.
impl From<CreateChatCompletionResponse> for Output {
    fn from(response: CreateChatCompletionResponse) -> Self {
        Self(response)
    }
}
