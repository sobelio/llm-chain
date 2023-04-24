use super::chat::ChatMessage;
use super::string_template::StringTemplate;

pub trait Prompt {
    fn as_chat_prompt(&self) -> Vec<ChatMessage>;
    fn as_text_prompt(&self) -> Option<&StringTemplate>;
}
