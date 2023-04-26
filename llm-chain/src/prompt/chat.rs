use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt;

use crate::tokens::{Tokenizer, TokenizerError};

use super::StringTemplate;

/// The `ChatRole` enum represents the role of a chat message sender in a conversation.
///
/// It has four variants:
/// - `User`: Represents a message sent by a user.
/// - `Assistant`: Represents a message sent by an AI assistant.
/// - `System`: Represents a message sent by a system or service.
/// - `Other`: Represents a message sent by any other role, specified by a string.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ChatRole {
    User,
    Assistant,
    System,
    Other(String),
}

impl fmt::Display for ChatRole {
    /// Formats the `ChatRole` enum as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_chain::prompt::chat::ChatRole;
    ///
    /// let user_role = ChatRole::User;
    /// let assistant_role = ChatRole::Assistant;
    ///
    /// assert_eq!(format!("{}", user_role), "User");
    /// assert_eq!(format!("{}", assistant_role), "Assistant");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatRole::User => write!(f, "User"),
            ChatRole::Assistant => write!(f, "Assistant"),
            ChatRole::System => write!(f, "System"),
            ChatRole::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage<Body> {
    pub role: ChatRole,
    pub body: Body,
}

impl<Body> ChatMessage<Body> {
    /// Create a new chat message.
    ///
    /// # Arguments
    /// * `role` - The role of the message sender.
    /// * `body` - The body of the message.
    pub fn new(role: ChatRole, body: Body) -> Self {
        Self { role, body }
    }

    pub fn system(body: Body) -> Self {
        Self::new(ChatRole::System, body)
    }
    pub fn user(body: Body) -> Self {
        Self::new(ChatRole::User, body)
    }
    pub fn assistant(body: Body) -> Self {
        Self::new(ChatRole::Assistant, body)
    }

    /// Maps the body of the chat message using the provided function `f`.
    ///
    /// # Arguments
    /// * `f` - The function to apply to the message body.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_chain::prompt::{ChatMessage, ChatRole};
    /// let msg = ChatMessage::new(ChatRole::Assistant, "Hello!");
    /// let mapped_msg = msg.map(|body| body.to_uppercase());
    ///
    /// assert_eq!(mapped_msg.body, "HELLO!");
    /// ```
    pub fn map<U, F: FnOnce(&Body) -> U>(&self, f: F) -> ChatMessage<U> {
        let role = self.role.clone();
        ChatMessage {
            role,
            body: f(&self.body),
        }
    }

    pub fn try_map<U, E, F: Fn(&Body) -> Result<U, E>>(&self, f: F) -> Result<ChatMessage<U>, E> {
        let body = f(&self.body)?;
        let role = self.role.clone();
        Ok(ChatMessage { role, body })
    }

    pub fn role(&self) -> &ChatRole {
        &self.role
    }
    pub fn body(&self) -> &Body {
        &self.body
    }
}

impl<T: fmt::Display> fmt::Display for ChatMessage<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.role, self.body)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageCollection<Body> {
    messages: VecDeque<ChatMessage<Body>>,
}

impl<Body> ChatMessageCollection<Body> {
    /// Creates a new empty `ChatMessageList`.
    pub fn new() -> Self {
        ChatMessageCollection {
            messages: VecDeque::new(),
        }
    }

    pub fn for_vector(messages: Vec<ChatMessage<Body>>) -> Self {
        ChatMessageCollection {
            messages: messages.into(),
        }
    }

    pub fn with_system(mut self, body: Body) -> Self {
        self.add_message(ChatMessage::system(body));
        self
    }

    pub fn with_user(mut self, body: Body) -> Self {
        self.add_message(ChatMessage::user(body));
        self
    }

    pub fn with_assistant(mut self, body: Body) -> Self {
        self.add_message(ChatMessage::assistant(body));
        self
    }

    /// Appends a `ChatMessage` to the list.
    pub fn add_message(&mut self, message: ChatMessage<Body>) {
        self.messages.push_back(message);
    }

    /// Removes the first message from the list and returns it, or `None` if the list is empty.
    pub fn remove_first_message(&mut self) -> Option<ChatMessage<Body>> {
        self.messages.pop_front()
    }

    /// Returns the number of messages in the list.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Returns a reference to the message at the specified index, or `None` if the index is out of bounds.
    pub fn get_message(&self, index: usize) -> Option<&ChatMessage<Body>> {
        self.messages.get(index)
    }

    /// Returns an iterator over the messages in the list.
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, ChatMessage<Body>> {
        self.messages.iter()
    }

    /// Creates a new `ChatMessageList` with the results of applying a function to each `ChatMessage`.
    ///
    /// # Arguments
    ///
    /// * `f` - The function to apply to each `ChatMessage`.
    pub fn map<U, F>(&self, f: F) -> ChatMessageCollection<U>
    where
        F: FnMut(&ChatMessage<Body>) -> ChatMessage<U>,
    {
        let mapped_messages: VecDeque<ChatMessage<U>> = self.messages.iter().map(f).collect();
        ChatMessageCollection {
            messages: mapped_messages,
        }
    }

    pub fn try_map<U, E, F: Fn(&Body) -> Result<U, E>>(
        &self,
        f: F,
    ) -> Result<ChatMessageCollection<U>, E> {
        let mut mapped_messages = VecDeque::new();

        for msg in self.messages.iter() {
            let mapped_msg = msg.try_map(|body| f(body))?;

            mapped_messages.push_back(mapped_msg);
        }

        Ok(ChatMessageCollection {
            messages: mapped_messages,
        })
    }

    /// Trims the conversation to the number of messages by removing the oldest messages.
    ///
    /// # Arguments
    ///
    /// * `max_number_of_messages` - The desired number of messages to keep in the conversation.
    pub fn trim_to_max_messages(&mut self, max_number_of_messages: usize) {
        while self.len() > max_number_of_messages {
            self.messages.pop_front();
        }
    }
}

impl<Body> Default for ChatMessageCollection<Body> {
    fn default() -> Self {
        ChatMessageCollection::new()
    }
}

impl<T: fmt::Display> fmt::Display for ChatMessageCollection<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for message in self.messages.iter() {
            writeln!(f, "{}", message)?;
        }
        Ok(())
    }
}

impl ChatMessageCollection<String> {
    pub fn trim_context<Tok, TT: Clone>(
        &mut self,
        tokenizer: &Tok,
        max_tokens: usize,
    ) -> Result<(), TokenizerError>
    where
        Tok: Tokenizer<TT>,
    {
        let mut total_tokens = 0;

        while let Some(msg) = self.messages.back() {
            let tokens = tokenizer.tokenize_str(&msg.body)?;
            total_tokens += tokens.len();

            if total_tokens > max_tokens {
                self.messages.pop_back();
            } else {
                break;
            }
        }
        Ok(())
    }
}

impl ChatMessageCollection<StringTemplate> {
    pub fn with_user_template(self, body: &str) -> Self {
        self.with_user(StringTemplate::tera(body))
    }
    pub fn with_system_template(self, body: &str) -> Self {
        self.with_system(StringTemplate::tera(body))
    }
    pub fn with_assistant_template(self, body: &str) -> Self {
        self.with_assistant(StringTemplate::tera(body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let msg = ChatMessage::new(ChatRole::Assistant, "Hello!");
        let mapped_msg = msg.map(|body| body.to_uppercase());

        assert_eq!(mapped_msg.body, "HELLO!");
        assert_eq!(mapped_msg.role, ChatRole::Assistant);
    }

    #[test]
    fn test_chat_message_list() {
        let mut chat_message_list = ChatMessageCollection::new();

        assert_eq!(chat_message_list.len(), 0);

        chat_message_list.add_message(ChatMessage::new(ChatRole::User, "Hello!"));
        chat_message_list.add_message(ChatMessage::new(ChatRole::Assistant, "Hi there!"));

        assert_eq!(chat_message_list.len(), 2);

        assert_eq!(chat_message_list.get_message(0).unwrap().body, "Hello!");
        assert_eq!(chat_message_list.get_message(1).unwrap().body, "Hi there!");

        chat_message_list.remove_first_message();
        assert_eq!(chat_message_list.len(), 1);
    }

    #[test]
    fn test_chat_message_list_map() {
        let mut chat_message_list = ChatMessageCollection::new();

        chat_message_list.add_message(ChatMessage::new(ChatRole::User, "Hello!"));
        chat_message_list.add_message(ChatMessage::new(ChatRole::Assistant, "Hi there!"));

        let mapped_list = chat_message_list
            .map(|msg| ChatMessage::new(msg.role.clone(), format!("{} (mapped)", msg.body)));

        assert_eq!(mapped_list.get_message(0).unwrap().body, "Hello! (mapped)");
        assert_eq!(
            mapped_list.get_message(1).unwrap().body,
            "Hi there! (mapped)"
        );
    }
}
