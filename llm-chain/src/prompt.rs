//! # Prompts module
//! ```rust
//! use llm_chain::prompt::{ChatPromptBuilder, TextPrompt};
//!
//! let chat_prompt = ChatPromptBuilder::new()
//!     .system("You are a helpful assistant.")
//!     .user("What is the meaning of life?")
//!     .build()
//!     .unwrap();
//!
//! let simple_text = TextPrompt::new("Hello {{name}}!");
//!
//! println!("{}", chat_prompt);
//! println!("{}", simple_text);
//! ```
use crate::PromptTemplate;
use derive_builder::Builder;
use std::fmt;

pub trait Prompt {
    fn as_chat_prompt(&self) -> Vec<ChatMessage>;
    fn as_text_prompt(&self) -> Option<&PromptTemplate>;
    fn as_text_prompt_or_convert(&self) -> PromptTemplate {
        if let Some(template) = self.as_text_prompt() {
            template.clone()
        } else {
            // We need to interperse the chat messages with newlines and also include the Role in each line
            let mut templates = Vec::with_capacity(self.as_chat_prompt().len() * 3);
            for message in self.as_chat_prompt() {
                templates.push(PromptTemplate::static_string(format!("{}: ", message.role)));
                templates.push(message.content);
                templates.push(PromptTemplate::static_string("\n"));
            }
            PromptTemplate::combine(templates)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, Builder, Clone)]
#[builder(setter(into))]
pub struct ChatMessage {
    role: ChatRole,
    content: PromptTemplate,
}

impl ChatMessage {
    pub fn role(&self) -> ChatRole {
        self.role.clone()
    }
    pub fn content(&self) -> PromptTemplate {
        self.content.clone()
    }
}

#[derive(Debug, Builder, Clone)]
pub struct ChatPrompt {
    messages: Vec<ChatMessage>,
}

impl Prompt for ChatPrompt {
    fn as_chat_prompt(&self) -> Vec<ChatMessage> {
        self.messages.clone()
    }

    fn as_text_prompt(&self) -> Option<&PromptTemplate> {
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

#[derive(Debug)]
pub struct TextPrompt {
    content: PromptTemplate,
}

impl TextPrompt {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: PromptTemplate::tera(content.into()),
        }
    }
}

impl Prompt for TextPrompt {
    fn as_chat_prompt(&self) -> Vec<ChatMessage> {
        vec![ChatMessage {
            role: ChatRole::System,
            content: self.content.clone(),
        }]
    }

    fn as_text_prompt(&self) -> Option<&PromptTemplate> {
        Some(&self.content)
    }
}

impl fmt::Display for TextPrompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl ChatMessage {
    pub fn new<S: Into<String>>(role: ChatRole, content: S) -> Self {
        Self {
            role,
            content: PromptTemplate::tera(content.into()),
        }
    }
}

// Adding extension methods for ChatPromptBuilder to push user, agent, and system messages
impl ChatPromptBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_message(mut self, message: ChatMessage) -> Self {
        self.messages.get_or_insert_with(|| vec![]).push(message);
        self
    }

    pub fn user<S: Into<String>>(self, message: S) -> Self {
        self.add_message(ChatMessage::new(ChatRole::User, message))
    }

    pub fn assistant<S: Into<String>>(self, message: S) -> Self {
        self.add_message(ChatMessage::new(ChatRole::Assistant, message))
    }

    pub fn system<S: Into<String>>(self, message: S) -> Self {
        self.add_message(ChatMessage::new(ChatRole::System, message))
    }
}

/// Creates a `TextPrompt` or a `ChatPrompt` based on the number of arguments provided.
///
/// If there is only one argument, it creates a `TextPrompt` with the provided template.
/// If there are two arguments, it creates a `ChatPrompt` with the first message as the system
/// message and the second message as the user message.
/// If there are more than two arguments, a compile-time error is produced.
///
/// # Example
///
/// ```rust
/// use llm_chain::prompt::TextPrompt;
/// use llm_chain::prompt::{ChatPromptBuilder, ChatPrompt};
/// use llm_chain::prompt;
///
/// let text_prompt: TextPrompt = prompt!("Hello {{name}}!");
/// assert_eq!(format!("{}", text_prompt), "Hello {{name}}!");
///
/// let chat_prompt: ChatPrompt = prompt!("You are a helpful assistant.", "What is the meaning of life?");
/// assert_eq!(format!("{}", chat_prompt), "System: You are a helpful assistant.\nUser: What is the meaning of life?\n");
/// ```
#[macro_export]
macro_rules! prompt {
    ($single_arg:expr) => {
        TextPrompt::new($single_arg)
    };
    ($system_arg:expr, $user_arg:expr $(,)?) => {
        llm_chain::prompt::ChatPromptBuilder::new()
            .system($system_arg)
            .user($user_arg)
            .build()
            .unwrap()
    };
    ($($extra_tokens:expr),+ $(,)?) => {
        compile_error!("The 'prompt!' macro takes at most 2 arguments.")
    };
}
