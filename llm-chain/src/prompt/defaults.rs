use super::chat::{ChatPrompt, ChatPromptBuilder};
use super::Prompt;

// Default Prompts can either be ChatPrompts or TextPrompts
pub trait DefaultPrompt<P: Prompt> {
    fn as_prompt() -> P;
}

pub struct ExtractiveSummaryChat;

impl DefaultPrompt<ChatPrompt> for ExtractiveSummaryChat {
    fn as_prompt() -> ChatPrompt {
        ChatPromptBuilder::new()
            .system("You are an extractive summarizer that follows the output pattern")
            .user("Please extract sentences as the summary. The summary should contain {{sentences}} sentences. Document: {{text}}")
            .build().unwrap()
    }
}
