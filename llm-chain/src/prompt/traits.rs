use super::chat::ChatMessage;
use crate::PromptTemplate;

pub trait Prompt {
    fn as_chat_prompt(&self) -> Vec<ChatMessage>;
    fn as_text_prompt(&self) -> Option<&PromptTemplate>;
}
