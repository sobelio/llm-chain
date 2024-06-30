use erniebot_rs::chat::{Message, Role};
use llm_chain::prompt::Prompt;

/// Creates a chat message from a prompt.
pub fn create_message(prompt: &Prompt) -> Vec<Message> {
    match prompt {
        Prompt::Text(text) => vec![Message {
            role: Role::User,
            content: text.clone(),
            ..Default::default()
        }],
        Prompt::Chat(chat) => {
            let mut messages = Vec::new();
            for message in chat.iter() {
                let role = match message.role() {
                    llm_chain::prompt::ChatRole::User => Role::User,
                    llm_chain::prompt::ChatRole::Assistant => Role::Assistant,
                    llm_chain::prompt::ChatRole::System => Role::Assistant, // ernie doesn't have a system role
                    llm_chain::prompt::ChatRole::Other(_) => todo!(),
                };
                let content = message.body();
                messages.push(Message {
                    role,
                    content: content.clone(),
                    ..Default::default()
                });
            }
            messages
        }
    }
}
