//! This module implements chains for the ChatGPT model from OpenAI.
mod executor;
mod options;
mod output;
mod prompt;

pub use async_openai::types::Role;
pub use executor::Executor;
pub use options::Model;
pub use output::Output;
