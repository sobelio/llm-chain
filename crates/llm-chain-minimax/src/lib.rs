//! # llm-chain-minimax
//!
//! Use MiniMax's powerful LLM models with `llm-chain`! This crate provides a
//! seamless integration with MiniMax's OpenAI-compatible API, giving you access
//! to models like MiniMax-M2.7 and MiniMax-M2.5.
//!
//! MiniMax offers high-performance language models with large context windows
//! (up to 1M tokens for M2.7, 204K for M2.5-highspeed) through an
//! OpenAI-compatible chat completions endpoint at `https://api.minimax.io/v1`.
//!
//! # Getting Started
//!
//! Set the `MINIMAX_API_KEY` environment variable with your MiniMax API key:
//!
//! ```bash
//! export MINIMAX_API_KEY="your-api-key-here"
//! ```
//!
//! Then use the executor in your code:
//!
//! ```ignore
//! use llm_chain::executor;
//! use llm_chain::options;
//!
//! let exec = executor!(minimax)?;
//! // Or with a specific model:
//! let exec = executor!(minimax, options!(Model: "MiniMax-M2.7"))?;
//! ```

pub mod chatgpt;
