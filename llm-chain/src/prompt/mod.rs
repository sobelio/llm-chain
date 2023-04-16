//! # Prompts module
//! ```rust
//! use llm_chain::prompt::chat::{ChatPromptBuilder};
//! use llm_chain::prompt::text::TextPrompt;
//! let chat_prompt = ChatPromptBuilder::new()
//!     .system("You are a helpful assistant.")
//!     .user("What is the meaning of life?")
//!     .build()
//!     .unwrap();
//!
//! let simple_text = TextPrompt::new("Hello {{name}}!");
//!
//! println!("{}", chat_prompt);
//! println!("{}", simple_text);
//! ```
pub mod chat;
mod templates;

pub mod text;
mod traits;

pub use templates::{PromptTemplate, PromptTemplateError};
pub use traits::Prompt;

/// Creates a `TextPrompt` or a `ChatPrompt` based on the number of arguments provided.
///
/// If there is only one argument, it creates a `TextPrompt` with the provided template.
/// If there are two arguments, it creates a `ChatPrompt` with the first message as the system
/// message and the second message as the user message.
/// If there are more than two arguments, a compile-time error is produced.
///
/// # Example
///
/// ```rust
/// use llm_chain::prompt::text::TextPrompt;
/// use llm_chain::prompt::chat::{ChatPromptBuilder, ChatPrompt};
/// use llm_chain::prompt;
///
/// let text_prompt: TextPrompt = prompt!("Hello {{name}}!");
/// assert_eq!(format!("{}", text_prompt), "Hello {{name}}!");
///
/// let chat_prompt: ChatPrompt = prompt!("You are a helpful assistant.", "What is the meaning of life?");
/// assert_eq!(format!("{}", chat_prompt), "System: You are a helpful assistant.\nUser: What is the meaning of life?\n");
/// ```
#[macro_export]
macro_rules! prompt {
    ($single_arg:expr) => {
        llm_chain::prompt::text::TextPrompt::new($single_arg)
    };
    ($system_arg:expr, $user_arg:expr $(,)?) => {
        llm_chain::prompt::chat::ChatPrompt::builder()
            .system($system_arg)
            .user($user_arg)
            .build()
            .unwrap()
    };
    ($($extra_tokens:expr),+ $(,)?) => {
        compile_error!("The 'prompt!' macro takes at most 2 arguments.")
    };
}
