//! This module implements chains for the ChatGPT model from OpenAI.
mod error;
mod executor;
mod model;
mod prompt;

pub use executor::{Error, Executor};
pub use model::Model;
