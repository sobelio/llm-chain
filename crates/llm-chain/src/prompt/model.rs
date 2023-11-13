use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An enum representing either a collection of chat messages or a single text.
pub enum Data<T> {
    /// A collection of chat messages.
    Chat(ChatMessageCollection<T>),
    /// A text prompt.
    Text(T),
}

impl<T> Data<T> {
    pub fn text(text: T) -> Self {
        Self::Text(text)
    }

    /// Maps the body of the chat messages or the text in the `Data` enum using the provided function.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes a reference to the body of a chat message or the text and returns a value of type `U`.
    ///
    /// # Returns
    ///
    /// A new `Data<U>` with the body of the chat messages or the text mapped by the provided function.
    pub fn map<U, F: Fn(&T) -> U>(&self, f: F) -> Data<U> {
        match self {
            Self::Chat(chat) => Data::Chat(chat.map(|msg| msg.map(|body| f(body)))),
            Self::Text(text) => Data::Text(f(text)),
        }
    }

    /// Maps the body of the chat messages or the text in the `Data` enum using the provided function that might fail.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes a reference to the body of a chat message or the text and returns a `Result<U, E>` value.
    ///
    /// # Returns
    ///
    /// A `Result<Data<U>, E>` with the body of the chat messages or the text mapped by the provided function.
    /// If the provided function returns an error, the error will be propagated in the result.
    pub fn try_map<U, E, F: Fn(&T) -> Result<U, E>>(&self, f: F) -> Result<Data<U>, E> {
        match self {
            Self::Chat(chat) => {
                let result = chat.try_map(|msg| f(msg))?;
                Ok(Data::Chat(result))
            }
            Self::Text(text) => Ok(Data::Text(f(text)?)),
        }
    }

    /// Extracts the body of the last message in the Data, or simply returns the Text if it is a text prompt
    pub fn extract_last_body(&self) -> Option<&T> {
        match self {
            Self::Chat(c) => c.extract_last_body(),
            Self::Text(t) => Some(t),
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

    /// Combines two `Data` values into one.
    ///
    /// If both values are `Chat`, the two chat collections will be combined.
    /// If one value is `Chat` and the other is `Text`, the text will be added as a message to the chat collection.
    ///
    /// # Arguments
    /// - `other` - The other `Data` value to combine with.
    pub fn combine(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Chat(chat1), Self::Chat(chat2)) => {
                let mut chat = chat1.clone();
                chat.append(chat2.clone());
                Self::Chat(chat)
            }
            (Self::Chat(chat), Self::Text(text)) => {
                let mut chat = chat.clone();
                chat.add_message(ChatMessage::new(ChatRole::User, text.clone()));
                Self::Chat(chat)
            }
            (Self::Text(text), Self::Chat(chat)) => {
                let mut chat = chat.clone();
                chat.add_message(ChatMessage::new(ChatRole::User, text.clone()));
                Self::Chat(chat)
            }
            (Self::Text(text1), Self::Text(text2)) => {
                let combined_text = format!("{}\n\n{}", text1, text2);
                Self::Text(combined_text)
            }
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

use crate::frame::FormatAndExecuteError;
use crate::output::Output;
use crate::prompt::{StringTemplate, StringTemplateError};
use crate::step::Step;
use crate::traits::Executor;
use crate::Parameters;

use super::chat::ChatMessageCollection;
use super::{ChatMessage, ChatRole};

impl Data<StringTemplate> {
    /// Helper function to run a prompt template.
    ///
    /// # Arguments
    /// parameters: &Parameters - The parameters to use for the prompt template.
    /// executor: &E - The executor to use for the prompt template.
    ///
    /// # Returns
    /// The output of applying the prompt template to the model.
    pub async fn run<E: Executor>(
        &self,
        parameters: &Parameters,
        executor: &E,
    ) -> Result<Output, FormatAndExecuteError> {
        Step::for_prompt_template(self.clone())
            .run(parameters, executor)
            .await
    }

    pub fn format(&self, parameters: &Parameters) -> Result<Data<String>, StringTemplateError> {
        self.try_map(|x| x.format(parameters))
    }
}
