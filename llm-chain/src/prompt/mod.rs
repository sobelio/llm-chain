//! Module implementing prompts and prompt templates.
//! Contains the `prompt!` macro, Prompts and PromptTemplates.

pub mod chat;
mod model;
mod string_template;

pub use string_template::{StringTemplate, StringTemplateError};

pub use chat::{ChatMessage, ChatMessageCollection, ChatRole};
pub use model::Data;

pub type PromptTemplate = Data<StringTemplate>;
pub type Prompt = Data<String>;

/// Creates a `TextPrompt` or a `ChatPrompt` based on the number of arguments provided.
///
/// If there is only one argument, it creates a `TextPrompt` with the provided template.
/// If there are two arguments, it creates a `ChatPrompt` with the first message as the system
/// message and the second message as the user message. You may add a "conversation: your_conversation" to include a conversation.
/// If there are more than two arguments, a compile-time error is produced.
///
/// # Example
///
/// ```rust
/// use llm_chain::prompt;
/// let text_prompt = prompt!("Hello {{name}}!");
/// assert_eq!(format!("{}", text_prompt), "Hello {{name}}!");
///
/// let chat_prompt = prompt!("You are a helpful assistant.", "What is the meaning of life?");
/// assert_eq!(format!("{}", chat_prompt), "System: You are a helpful assistant.\nUser: What is the meaning of life?\n");
/// ```
#[macro_export]
macro_rules! prompt {
    ($single_arg:expr) => {
        $crate::prompt::Data::Text($crate::prompt::StringTemplate::tera($single_arg))
    };
    ($system_arg:expr, $user_arg:expr $(,)?) => {
        $crate::prompt::Data::Chat(
            $crate::prompt::ChatMessageCollection::<$crate::prompt::StringTemplate>::new()
                .with_system($crate::prompt::StringTemplate::tera($system_arg))
                .with_user($crate::prompt::StringTemplate::tera($user_arg)),
        )
    };
    ($($extra_tokens:expr),+ $(,)?) => {
        compile_error!("The 'prompt!' macro takes at most 2 arguments.")
    };
}
