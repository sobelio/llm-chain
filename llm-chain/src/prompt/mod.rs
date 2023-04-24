//! # Prompts module
//! ```rust
//! use llm_chain::prompt::chat::{ChatPromptBuilder};
//! use llm_chain::prompt::text::TextPrompt;
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
pub mod chat;
mod string_template;

mod conversation;
pub mod text;
mod traits;

pub use conversation::Conversation;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// XXX: Check this?
pub use string_template::{StringTemplate, StringTemplateError};

use crate::{step::Step, traits::Executor, Parameters};

use self::{
    chat::{ChatPrompt, ChatRole},
    traits::Prompt as PromptTrait,
};

/// Creates a `TextPrompt` or a `ChatPrompt` based on the number of arguments provided.
///
/// If there is only one argument, it creates a `TextPrompt` with the provided template.
/// If there are two arguments, it creates a `ChatPrompt` with the first message as the system
/// message and the second message as the user message. You may add a "conversation: your_conversation" to include a conversation.
/// If there are more than two arguments, a compile-time error is produced.
///
/// # Example
///
/// ```rust
/// use llm_chain::prompt::text::TextPrompt;
/// use llm_chain::prompt::chat::{ChatPromptBuilder, ChatPrompt};
/// use llm_chain::prompt;
///
/// let text_prompt = prompt!("Hello {{name}}!");
/// assert_eq!(format!("{}", text_prompt), "Hello {{name}}!");
///
/// let chat_prompt = prompt!("You are a helpful assistant.", "What is the meaning of life?");
/// assert_eq!(format!("{}", chat_prompt), "System: You are a helpful assistant.\nUser: What is the meaning of life?\n");
/// ```
#[macro_export]
macro_rules! prompt {
    ($single_arg:expr) => {
        $crate::prompt::Prompt::new_from_text_prompt(
            llm_chain::prompt::text::TextPrompt::new($single_arg)
        )
    };
    ($system_arg:expr, $user_arg:expr $(,)?) => {
        $crate::prompt::Prompt::new_from_chat_prompt(
            $crate::prompt::chat::ChatPrompt::builder()
                .system($system_arg)
                .user($user_arg)
                .build()
                .unwrap(), // This unwrap is safe because we know that the builder will always succeed
        )
    };
    ($user_arg:expr, conversation: $conversation:expr $(,)?) => {
        {
            let mut chat_prompt = $crate::prompt::chat::ChatPrompt::builder()
                .conversation($conversation)
                .user($user_arg)
                .build()
                .unwrap(); // This unwrap is safe because we know that the builder will always succeed
            $crate::prompt::Prompt::new_from_chat_prompt(chat_prompt)
        }
    };
    ($system_arg:expr, $user_arg:expr, conversation: $conversation:expr $(,)?) => {
        {
            let mut chat_prompt = $crate::prompt::chat::ChatPrompt::builder()
                .conversation($conversation)
                .system($system_arg)
                .user($user_arg)
                .build()
                .unwrap(); // This unwrap is safe because we know that the builder will always succeed
            $crate::prompt::Prompt::new_from_chat_prompt(chat_prompt)
        }
    };
    ($($extra_tokens:expr),+ $(,)?) => {
        compile_error!("The 'prompt!' macro takes at most 3 arguments.")
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Prompt(PromptImpl);

impl Prompt {
    pub fn new_from_text_prompt(text_prompt: text::TextPrompt) -> Self {
        Self(PromptImpl::TextPrompt(text_prompt))
    }
    pub fn new_from_chat_prompt(chat_prompt: chat::ChatPrompt) -> Self {
        Self(PromptImpl::ChatPrompt(chat_prompt))
    }
    pub fn as_chat_prompt(&self) -> Vec<chat::ChatMessage> {
        self.0.as_chat_prompt()
    }

    pub fn as_text_prompt(&self) -> Option<&StringTemplate> {
        self.0.as_text_prompt()
    }

    pub fn as_text_prompt_or_convert(&self) -> StringTemplate {
        if let Some(template) = self.as_text_prompt() {
            template.clone()
        } else {
            // We need to interperse the chat messages with newlines and also include the Role in each line
            let mut templates = Vec::with_capacity(self.as_chat_prompt().len() * 3);
            for message in self.as_chat_prompt() {
                templates.push(StringTemplate::static_string(format!(
                    "{}: ",
                    message.role()
                )));
                templates.push(message.content());
                templates.push(StringTemplate::static_string("\n"));
            }
            StringTemplate::combine(templates)
        }
    }

    pub async fn run<E: Executor>(
        &self,
        parameters: &Parameters,
        executor: &E,
    ) -> Result<E::Output, E::Error> {
        Step::for_prompt(self.clone())
            .run(parameters, executor)
            .await
    }
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            PromptImpl::ChatPrompt(chat_prompt) => write!(f, "{}", chat_prompt),
            PromptImpl::TextPrompt(text_prompt) => write!(f, "{}", text_prompt),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptImpl {
    ChatPrompt(chat::ChatPrompt),
    TextPrompt(text::TextPrompt),
}

impl traits::Prompt for PromptImpl {
    fn as_chat_prompt(&self) -> Vec<chat::ChatMessage> {
        match &self {
            PromptImpl::ChatPrompt(chat_prompt) => chat_prompt.as_chat_prompt(),
            PromptImpl::TextPrompt(text_prompt) => text_prompt.as_chat_prompt(),
        }
    }

    fn as_text_prompt(&self) -> Option<&StringTemplate> {
        match &self {
            PromptImpl::TextPrompt(text_prompt) => text_prompt.as_text_prompt(),
            PromptImpl::ChatPrompt(chat_prompt) => chat_prompt.as_text_prompt(),
        }
    }
}

impl From<ChatPrompt> for Prompt {
    fn from(chat_prompt: ChatPrompt) -> Self {
        Self(PromptImpl::ChatPrompt(chat_prompt))
    }
}

impl From<text::TextPrompt> for Prompt {
    fn from(text_prompt: text::TextPrompt) -> Self {
        Self(PromptImpl::TextPrompt(text_prompt))
    }
}

pub struct ActualPrompt {
    prompt: Prompt,
    parameters: Parameters,
}

impl ActualPrompt {
    pub fn for_prompt_and_parameters(prompt: Prompt, parameters: Parameters) -> Self {
        Self { prompt, parameters }
    }
    pub fn for_prompt(prompt: Prompt) -> Self {
        Self {
            prompt,
            parameters: Parameters::new(),
        }
    }
    // -> Self::MyActualPrompt
    pub fn as_chat_prompt(&self) -> Result<Vec<ActualChatMessage>, StringTemplateError> {
        self.prompt
            .as_chat_prompt()
            .into_iter()
            .map(|c| -> Result<ActualChatMessage, StringTemplateError> {
                c.content()
                    .format(&self.parameters)
                    .map(|s| ActualChatMessage::new(c.role(), s))
            })
            .collect()
    }
    pub fn as_text_prompt(&self) -> Result<Option<String>, StringTemplateError> {
        self.prompt
            .as_text_prompt()
            .map(|text_prompt| -> Result<String, StringTemplateError> {
                text_prompt.format(&self.parameters)
            })
            .transpose()
    }
}

pub struct ActualChatMessage {
    role: ChatRole,
    content: String,
}

impl ActualChatMessage {
    pub fn new(role: ChatRole, content: String) -> Self {
        Self { role, content }
    }
}

pub struct ActualTextPrompt {
    text: String,
}

impl ActualTextPrompt {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
