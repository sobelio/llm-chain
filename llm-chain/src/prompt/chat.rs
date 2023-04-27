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
    /// use llm_chain::prompt::ChatRole;
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
/// The `ChatMessage` struct represents a chat message.
/// It has two fields:
/// - `role`: The role of the message sender.
/// - `body`: The body of the message.
pub struct ChatMessage<Body> {
    role: ChatRole,
    body: Body,
}

impl<Body> ChatMessage<Body> {
    /// Creates a new chat message.
    ///
    /// # Arguments
    /// * `role` - The role of the message sender.
    /// * `body` - The body of the message.
    pub fn new(role: ChatRole, body: Body) -> Self {
        Self { role, body }
    }

    /// Creates a new chat message with the role of `Assistant`.
    ///
    /// # Arguments
    /// * `body` - The body of the message.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_chain::prompt::{ChatMessage, ChatRole};
    /// let msg = ChatMessage::assistant("Hello, how can I help you?");
    ///
    /// assert_eq!(msg.role(), &ChatRole::Assistant);
    /// ```
    pub fn assistant(body: Body) -> Self {
        Self::new(ChatRole::Assistant, body)
    }

    /// Creates a new chat message with the role of `User`.
    ///
    /// # Arguments
    /// * `body` - The body of the message.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_chain::prompt::{ChatMessage, ChatRole};
    /// let msg = ChatMessage::user("What's the weather like today?");
    ///
    /// assert_eq!(msg.role(), &ChatRole::User);
    /// ```
    pub fn user(body: Body) -> Self {
        Self::new(ChatRole::User, body)
    }

    /// Creates a new chat message with the role of `System`.
    ///
    /// # Arguments
    /// * `body` - The body of the message.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_chain::prompt::{ChatMessage, ChatRole};
    /// let msg = ChatMessage::system("Session started.");
    ///
    /// assert_eq!(msg.role(), &ChatRole::System);
    /// ```
    pub fn system(body: Body) -> Self {
        Self::new(ChatRole::System, body)
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
    /// assert_eq!(mapped_msg.body(), "HELLO!");
    /// ```
    pub fn map<U, F: FnOnce(&Body) -> U>(&self, f: F) -> ChatMessage<U> {
        let role = self.role.clone();
        ChatMessage {
            role,
            body: f(&self.body),
        }
    }

    /// Applies a fallible function `f` to the body of the chat message and returns a new chat message
    /// with the mapped body or an error if the function fails.
    ///
    /// # Arguments
    /// * `f` - The fallible function to apply to the message body.
    pub fn try_map<U, E, F: Fn(&Body) -> Result<U, E>>(&self, f: F) -> Result<ChatMessage<U>, E> {
        let body = f(&self.body)?;
        let role = self.role.clone();
        Ok(ChatMessage { role, body })
    }

    /// Returns a reference to the role of the message sender.
    pub fn role(&self) -> &ChatRole {
        &self.role
    }

    /// Returns a reference to the body of the message.
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
/// A collection of chat messages with various roles (e.g., user, assistant, system).
pub struct ChatMessageCollection<Body> {
    messages: VecDeque<ChatMessage<Body>>,
}

impl<Body> ChatMessageCollection<Body> {
    /// Creates a new empty `ChatMessageCollection`.
    pub fn new() -> Self {
        ChatMessageCollection {
            messages: VecDeque::new(),
        }
    }

    /// Creates a `ChatMessageCollection` from a given vector of `ChatMessage`.
    ///
    /// # Arguments
    ///
    /// * `messages` - A vector of `ChatMessage` instances to be included in the collection.
    pub fn for_vector(messages: Vec<ChatMessage<Body>>) -> Self {
        ChatMessageCollection {
            messages: messages.into(),
        }
    }

    /// Adds a system message to the collection with the given body.
    ///
    /// # Arguments
    ///
    /// * `body` - The message body to be added as a system message.
    pub fn with_system(mut self, body: Body) -> Self {
        self.add_message(ChatMessage::system(body));
        self
    }

    /// Adds a user message to the collection with the given body.
    ///
    /// # Arguments
    ///
    /// * `body` - The message body to be added as a user message.
    pub fn with_user(mut self, body: Body) -> Self {
        self.add_message(ChatMessage::user(body));
        self
    }

    /// Adds an assistant message to the collection with the given body.
    ///
    /// # Arguments
    ///
    /// * `body` - The message body to be added as an assistant message.
    pub fn with_assistant(mut self, body: Body) -> Self {
        self.add_message(ChatMessage::assistant(body));
        self
    }

    /// Appends another ChatMessageCollection to this one
    ///
    /// # Arguments
    /// - `other` - The other ChatMessageCollection to append to this one
    pub fn append(&mut self, other: ChatMessageCollection<Body>) {
        self.messages.extend(other.messages);
    }

    /// Appends a `ChatMessage` to the collection.
    ///
    /// # Arguments
    ///
    /// * `message` - The `ChatMessage` instance to be added to the collection.
    pub fn add_message(&mut self, message: ChatMessage<Body>) {
        self.messages.push_back(message);
    }

    /// Removes the first message from the collection and returns it, or `None` if the collection is empty.
    pub fn remove_first_message(&mut self) -> Option<ChatMessage<Body>> {
        self.messages.pop_front()
    }

    /// Returns the number of messages in the collection.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Returns a reference to the message at the specified index, or `None` if the index is out of bounds.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the desired message in the collection.
    pub fn get_message(&self, index: usize) -> Option<&ChatMessage<Body>> {
        self.messages.get(index)
    }

    /// Returns an iterator over the messages in the collection.
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, ChatMessage<Body>> {
        self.messages.iter()
    }

    /// Creates a new `ChatMessageCollection` with the results of applying a function to each `ChatMessage`.
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

    /// Creates a new `ChatMessageCollection` by applying a fallible function to each message body
    /// in the current collection. Returns an error if the function fails for any message.
    ///
    /// # Arguments
    ///
    /// * `f` - The fallible function to apply to each message body.
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

    /// Trims the conversation to the specified number of messages by removing the oldest messages.
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

/// Implementation of `ChatMessageCollection` for `String`.
impl ChatMessageCollection<String> {
    /// Trims the conversation context by removing the oldest messages in the collection
    /// until the total number of tokens in the remaining messages is less than or equal
    /// to the specified `max_tokens` limit.
    ///
    /// # Arguments
    ///
    /// * `tokenizer` - An instance of a `Tokenizer` that is used to tokenize the chat message bodies.
    /// * `max_tokens` - The maximum number of tokens allowed in the trimmed conversation context.
    ///
    /// # Returns
    ///
    /// A `Result<(), TokenizerError>` indicating success or failure.
    pub fn trim_context<Tok, TT: Clone>(
        &mut self,
        tokenizer: &Tok,
        max_tokens: i32,
    ) -> Result<(), TokenizerError>
    where
        Tok: Tokenizer<TT>,
    {
        let mut total_tokens: i32 = 0;

        // Remove the oldest messages from the collection
        // until the total tokens are within the limit.
        while let Some(msg) = self.messages.back() {
            let tokens = tokenizer.tokenize_str(&msg.body)?;
            total_tokens += tokens.len() as i32;
            if total_tokens > max_tokens {
                self.messages.pop_back();
            } else {
                break;
            }
        }
        Ok(())
    }
}

/// Implementation of `ChatMessageCollection` for `StringTemplate`.
impl ChatMessageCollection<StringTemplate> {
    /// Adds a user message to the conversation using the specified template string.
    ///
    /// # Arguments
    ///
    /// * `body` - A template string representing the message body.
    ///
    /// # Returns
    ///
    /// A modified `ChatMessageCollection` with the new user message added.
    pub fn with_user_template(self, body: &str) -> Self {
        self.with_user(StringTemplate::tera(body))
    }

    /// Adds a system message to the conversation using the specified template string.
    ///
    /// # Arguments
    ///
    /// * `body` - A template string representing the message body.
    ///
    /// # Returns
    ///
    /// A modified `ChatMessageCollection` with the new system message added.
    pub fn with_system_template(self, body: &str) -> Self {
        self.with_system(StringTemplate::tera(body))
    }

    /// Adds an assistant message to the conversation using the specified template string.
    ///
    /// # Arguments
    ///
    /// * `body` - A template string representing the message body.
    ///
    /// # Returns
    ///
    /// A modified `ChatMessageCollection` with the new assistant message added.
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
