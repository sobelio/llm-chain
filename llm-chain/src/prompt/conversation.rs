//! Conversation - a struct that represents a conversation with a chat-style model.
use super::chat::{ChatMessage, ChatRole};
use crate::{
    output::Output,
    tokens::{Tokenizer, TokenizerError},
};

/// A `Conversation` represents a conversation with a chat-style model.
/// This struct is useful for keeping context between messages and provides
/// utility methods for easily managing and updating the conversation.
#[derive(Clone, Debug)]
pub struct Conversation {
    v: Vec<(ChatRole, String)>,
}

impl Conversation {
    /// Creates a new, empty `Conversation`.
    pub fn new() -> Self {
        Self::new_from_vec(Vec::new())
    }

    fn new_from_vec(v: Vec<(ChatRole, String)>) -> Self {
        Conversation { v }
    }

    /// Returns the number of messages in the conversation.
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /// Get chatmessages
    pub fn get_chat_messages(&self) -> Vec<ChatMessage> {
        self.v
            .iter()
            .map(|(r, s)| ChatMessage::static_string(r.clone(), s))
            .collect()
    }

    /// Trims the conversation to the specified context window size by removing the oldest messages.
    ///
    /// # Arguments
    ///
    /// * `max_number_of_messages` - The desired number of messages to keep in the conversation.
    pub fn trim_to_max_messages(&mut self, max_number_of_messages: usize) {
        if self.v.len() > max_number_of_messages {
            let num_messages_to_remove = self.v.len() - max_number_of_messages;
            self.v.drain(0..num_messages_to_remove);
        }
    }

    pub fn trim_context<Tok, TT: Clone>(
        &mut self,
        tokenizer: &Tok,
        max_tokens: usize,
    ) -> Result<(), TokenizerError>
    where
        Tok: Tokenizer<TT>,
    {
        // Grab messages from most recent to oldest. (i.e. reverse order) While the total number of tokens is lower than max_tokens, keep adding messages.
        let mut total_tokens = 0;
        let mut messages_to_keep = Vec::new();
        for (role, message) in self.v.iter().rev() {
            let tokens = tokenizer.tokenize_str(message)?;
            total_tokens += tokens.len();
            if total_tokens <= max_tokens {
                messages_to_keep.push((role.clone(), message.clone()));
            } else {
                break;
            }
        }
        self.v = messages_to_keep.into_iter().rev().collect();
        Ok(())
    }

    /// Appends an `Output` as a `ChatMessage` to the conversation.
    ///
    /// # Arguments
    ///
    /// * `output` - The `Output` to be converted to a `ChatMessage` and added to the conversation.
    pub async fn add_output<O: Output>(&mut self, output: &O) {
        if let (Some(chat_role), Some(text_output)) = (
            output.get_chat_role().await,
            output.primary_textual_output().await,
        ) {
            self.v.push((chat_role, text_output));
        }
    }
    /// Adds a message to the conversation.
    ///
    /// # Arguments
    ///
    /// * `role` - The role of the message sender.
    /// * `message` - The message to be added to the conversation.
    pub fn add_message<S: Into<String>>(&mut self, role: ChatRole, message: S) {
        self.v.push((role, message.into()));
    }
}

impl Default for Conversation {
    fn default() -> Self {
        Conversation::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation() {
        let mut convo = Conversation::new();

        assert_eq!(convo.len(), 0);

        convo.add_message(ChatRole::Assistant, "hello world");
        convo.add_message(ChatRole::User, "hello world");

        assert_eq!(convo.len(), 2);

        convo.trim_to_max_messages(1);

        assert_eq!(convo.len(), 1);
    }
}
