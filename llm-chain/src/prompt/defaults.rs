use super::chat::{ChatPrompt, ChatPromptBuilder};
use super::Prompt;

// Default Prompts can either be ChatPrompts or TextPrompts
pub trait DefaultPrompt {
    type Prompt : Prompt;
    fn as_prompt() -> Self::Prompt;
}

pub struct ExtractiveSummaryChat;

impl DefaultPrompt for ExtractiveSummaryChat {
    type Prompt = ChatPrompt;
    fn as_prompt() -> Self::Prompt {
        ChatPromptBuilder::new()
            .system("You are an extractive summarizer that follows the output pattern")
            .user("Please extract sentences as the summary. The summary should contain {{sentences}} sentences. Document: {{text}}")
            .build().unwrap()
    }
}
