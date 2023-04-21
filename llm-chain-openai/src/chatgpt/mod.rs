//! This module implements chains for the ChatGPT model from OpenAI.
mod executor;
mod options;
mod output;
mod prompt;

mod text_splitter;

pub use async_openai::types::Role;
pub use executor::Executor;
pub use options::Model;
pub use output::Output;

#[deprecated(note = "Use llm_chain::step::Step instead", since = "0.7.0")]
pub use llm_chain::step::Step;
pub use text_splitter::OpenAITextSplitter;
