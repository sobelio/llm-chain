//! This module implements chains for the ChatGPT model from OpenAI.
mod executor;
mod options;
mod prompt;

pub use executor::{Error, Executor};
pub use options::{Model, PerExecutor, PerInvocation};
