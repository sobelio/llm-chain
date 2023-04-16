use super::chat::ChatMessage;
use crate::PromptTemplate;

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
                templates.push(PromptTemplate::static_string(format!(
                    "{}: ",
                    message.role()
                )));
                templates.push(message.content());
                templates.push(PromptTemplate::static_string("\n"));
            }
            PromptTemplate::combine(templates)
        }
    }
}
