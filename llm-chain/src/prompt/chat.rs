use super::string_template::StringTemplate;
use super::{conversation::Conversation, traits::Prompt};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ChatRole {
    User,
    Assistant,
    System,
    Other(String),
}

impl fmt::Display for ChatRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatRole::User => write!(f, "User"),
            ChatRole::Assistant => write!(f, "Assistant"),
            ChatRole::System => write!(f, "System"),
            ChatRole::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct ChatMessage {
    role: ChatRole,
    content: StringTemplate,
}

impl ChatMessage {
    /// Creates a new `ChatMessage` from a role and a string.
    pub fn new<S: Into<String>>(role: ChatRole, content: S) -> Self {
        Self {
            role,
            content: StringTemplate::tera(content.into()),
        }
    }
    pub fn legacy<S: Into<String>>(role: ChatRole, content: S) -> Self {
        Self {
            role,
            content: StringTemplate::legacy(content.into()),
        }
    }

    /// Creates a chat-message prompt from a static string useful for untrusted inputs
    pub fn static_string<S: Into<String>>(role: ChatRole, content: S) -> Self {
        Self {
            role,
            content: StringTemplate::static_string(content),
        }
    }

    /// Creates a new `ChatMessage` from a role and a prompt template.
    pub fn from_template(role: ChatRole, content: StringTemplate) -> Self {
        Self { role, content }
    }
    pub fn role(&self) -> ChatRole {
        self.role.clone()
    }
    pub fn content(&self) -> StringTemplate {
        self.content.clone()
    }
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
pub struct ChatPrompt {
    messages: Vec<ChatMessage>,
}

impl ChatPrompt {
    /// Returns a new `ChatPromptBuilder` for building a `ChatPrompt`.
    pub fn builder() -> ChatPromptBuilder {
        ChatPromptBuilder::default()
    }
    pub fn to_builder(&self) -> ChatPromptBuilder {
        let mut cpb = ChatPromptBuilder::default();
        cpb.messages(self.messages.clone());
        cpb
    }
}

impl Prompt for ChatPrompt {
    fn as_chat_prompt(&self) -> Vec<ChatMessage> {
        self.messages.clone()
    }

    fn as_text_prompt(&self) -> Option<&StringTemplate> {
        None
    }
}

impl fmt::Display for ChatPrompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for message in &self.messages {
            writeln!(f, "{}: {}", message.role, message.content)?;
        }
        Ok(())
    }
}

// Adding extension methods for ChatPromptBuilder to push user, agent, and system messages
impl ChatPromptBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a chat message to the prompt
    pub fn add_message(mut self, message: ChatMessage) -> Self {
        self.messages
            .get_or_insert_with(std::vec::Vec::new)
            .push(message);
        self
    }

    /// Adds a user message to the prompt
    pub fn user<S: Into<String>>(self, message: S) -> Self {
        self.add_message(ChatMessage::new(ChatRole::User, message))
    }

    /// Adds an agent message to the prompt
    pub fn assistant<S: Into<String>>(self, message: S) -> Self {
        self.add_message(ChatMessage::new(ChatRole::Assistant, message))
    }

    /// Adds a system message to the prompt
    pub fn system<S: Into<String>>(self, message: S) -> Self {
        self.add_message(ChatMessage::new(ChatRole::System, message))
    }

    /// Adds a conversation to the start of the prompt. Useful for adding a conversation history.
    ///
    /// # Parameters
    /// * `conversation` - The conversation to add to the prompt
    pub fn conversation(mut self, conversation: Conversation) -> Self {
        // Get the messages from the input conversation
        let mut convo_messages = conversation.get_chat_messages();

        // Append the current messages to the end of the input conversation's messages
        convo_messages.extend(self.messages.unwrap_or_default().iter().cloned());

        // Replace the current conversation's messages with the updated list
        self.messages = Some(convo_messages);
        self
    }
}
