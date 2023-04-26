use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An enum representing either a collection of chat messages or a single text.
pub enum Data<T> {
    /// A collection of chat messages.
    Chat(ChatMessageCollection<T>),
    /// A single text.
    Text(T),
}

impl<T> Data<T> {
    pub fn text(text: T) -> Self {
        Self::Text(text)
    }

    /// Maps the body of the chat messages or the text in the `MessageOrText` enum using the provided function.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes a reference to the body of a chat message or the text and returns a value of type `U`.
    ///
    /// # Returns
    ///
    /// A new `MessageOrText<U>` with the body of the chat messages or the text mapped by the provided function.
    pub fn map<U, F: Fn(&T) -> U>(&self, f: F) -> Data<U> {
        match self {
            Self::Chat(chat) => Data::Chat(chat.map(|msg| msg.map(|body| f(body)))),
            Self::Text(text) => Data::Text(f(text)),
        }
    }

    pub fn try_map<U, E, F: Fn(&T) -> Result<U, E>>(&self, f: F) -> Result<Data<U>, E> {
        match self {
            Self::Chat(chat) => {
                let result = chat.try_map(|msg| f(msg))?;
                Ok(Data::Chat(result))
            }
            Self::Text(text) => Ok(Data::Text(f(text)?)),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Data<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Chat(chat) => write!(f, "{}", chat),
            Self::Text(text) => write!(f, "{}", text),
        }
    }
}

impl Data<String> {
    pub fn to_chat(&self) -> ChatMessageCollection<String> {
        match self {
            Self::Chat(chat) => chat.clone(),
            Self::Text(text) => {
                let mut chat = ChatMessageCollection::new();
                chat.add_message(ChatMessage::new(ChatRole::User, text.clone()));
                chat
            }
        }
    }
    pub fn to_text(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Chat(chat) => chat.to_string(),
        }
    }
}

impl<T> From<T> for Data<T> {
    fn from(text: T) -> Self {
        Self::Text(text)
    }
}

impl<T> From<ChatMessageCollection<T>> for Data<T> {
    fn from(chat: ChatMessageCollection<T>) -> Self {
        Self::Chat(chat)
    }
}

impl<T> From<ChatMessage<T>> for Data<T> {
    fn from(chat: ChatMessage<T>) -> Self {
        Self::Chat(ChatMessageCollection::for_vector(vec![chat]))
    }
}

// move to another file
use crate::prompt::{StringTemplate, StringTemplateError};
use crate::step::Step;
use crate::traits::Executor;
use crate::Parameters;

use super::chat::ChatMessageCollection;
use super::{ChatMessage, ChatRole};

impl Data<StringTemplate> {
    pub async fn run<E: Executor>(
        &self,
        parameters: &Parameters,
        executor: &E,
    ) -> Result<E::Output, E::Error> {
        Step::for_prompt_template(self.clone())
            .run(parameters, executor)
            .await
    }

    pub fn format(&self, parameters: &Parameters) -> Result<Data<String>, StringTemplateError> {
        self.try_map(|x| x.format(parameters))
    }
}
