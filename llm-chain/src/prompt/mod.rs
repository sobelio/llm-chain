//! Module implementing prompts and prompt templates.
//! Contains the `prompt!` macro, Prompts and PromptTemplates.

mod chat;
mod model;
mod string_template;

pub use string_template::{StringTemplate, StringTemplateError};

pub use chat::{ChatMessage, ChatMessageCollection, ChatRole};
pub use model::Data;

/// A prompt template.
///
/// A prompt template is a template that can be used to generate a prompt.
pub type PromptTemplate = Data<StringTemplate>;
/// A prompt.
///
/// A prompt input for an LLM model.
pub type Prompt = Data<String>;

/// A chat conversation.
///
/// A chat conversation is a collection of chat messages.
pub type Conversation = ChatMessageCollection<String>;

/// Creates a `TextPrompt` or a `ChatPrompt` based on the number of arguments provided.
///
/// If there is only one argument, it creates a `TextPrompt` with the provided template. However,
/// if you prefix it with system:, assistant: or user: it will create a `ChatPrompt` with the provided
/// template as the system, assistant or user message respectively.
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
///
/// let role_prompt = prompt!(system: "You are a helpful assistant.");
/// assert_eq!(format!("{}", role_prompt), "System: You are a helpful assistant.\n");
/// ```
#[macro_export]
macro_rules! prompt {
    (user: $user_arg:expr $(,)?) => {
        $crate::prompt::Data::Chat(
            $crate::prompt::ChatMessageCollection::<$crate::prompt::StringTemplate>::new()
                .with_user_template($user_arg),
        )
    };
    (assistant: $assistant_arg:expr $(,)?) => {
        $crate::prompt::Data::Chat(
            $crate::prompt::ChatMessageCollection::<$crate::prompt::StringTemplate>::new()
                .with_assistant_template($assistant_arg),
        )
    };
    (system: $system_arg:expr $(,)?) => {
        $crate::prompt::Data::Chat(
            $crate::prompt::ChatMessageCollection::<$crate::prompt::StringTemplate>::new()
                .with_system_template($system_arg),
        )
    };
    ($single_arg:expr) => {
        $crate::prompt::Data::Text($crate::prompt::StringTemplate::tera($single_arg))
    };
    ($system_arg:expr, $user_arg:expr $(,)?) => {
        $crate::prompt::Data::Chat(
            $crate::prompt::ChatMessageCollection::<$crate::prompt::StringTemplate>::new()
                .with_system_template($system_arg)
                .with_user_template($user_arg),
        )
    };
    ($($extra_tokens:expr),+ $(,)?) => {
        compile_error!("The 'prompt!' macro takes at most 2 arguments.")
    };
}
