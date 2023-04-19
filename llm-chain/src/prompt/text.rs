//! This module defines the `TextPrompt` struct, which represents a text-only prompt without implying the existence of a chat.
//! This is useful for non-chat models. As an added benefit, `TextPrompt` can be used in chat models as well.

use super::chat::{ChatMessage, ChatRole};
use super::traits::Prompt;
use crate::PromptTemplate;
use std::fmt;

/// Represents a text-only prompt, without implying the existence of a chat. This is useful for non-chat models.
/// As an added benefit, `TextPrompt` can be used in chat models as well.
#[derive(Debug)]
pub struct TextPrompt {
    content: PromptTemplate,
}

impl TextPrompt {
    /// Creates a new `TextPrompt` with the provided template.
    ///
    /// # Arguments
    ///
    /// * `content` - A string that represents the template for the text prompt.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_chain::prompt::text::TextPrompt;
    ///
    /// let text_prompt = TextPrompt::new("Hello, {{name}}!");
    /// ```
    #[cfg(feature = "tera")]
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: PromptTemplate::tera(content.into()),
        }
    }
    #[cfg(not(feature = "tera"))]
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: PromptTemplate::legacy(content.into()),
        }
    }
}

/// Implement the `Prompt` trait for `TextPrompt`.
impl Prompt for TextPrompt {
    /// Converts a `TextPrompt` into a `Vec<ChatMessage>` where the `TextPrompt` is treated as a user message.
    fn as_chat_prompt(&self) -> Vec<ChatMessage> {
        vec![ChatMessage::from_template(
            ChatRole::User,
            self.content.clone(),
        )]
    }

    /// Returns a reference to the `PromptTemplate` for the `TextPrompt`.
    fn as_text_prompt(&self) -> Option<&PromptTemplate> {
        Some(&self.content)
    }
}

/// Implement `fmt::Display` for `TextPrompt` for pretty-printing.
impl fmt::Display for TextPrompt {
    /// Formats the `TextPrompt` for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
