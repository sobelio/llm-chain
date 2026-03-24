//! This module implements the MiniMax chat model executor using the
//! OpenAI-compatible API at `https://api.minimax.io/v1`.
mod error;
mod executor;
mod model;
mod prompt;

pub use executor::{Error, Executor};
pub use model::Model;
