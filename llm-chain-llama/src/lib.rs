//! # llm-chain-llama
//!
//! Welcome to the world of `llm-chain-llama`! This crate supercharges your applications with the power of LLaMA (Large Language Model Applications), providing a robust framework for creating chains of LLaMA models to generate human-like text.
//!
//! Designed to work seamlessly with LLaMA models, `llm-chain-llama` makes it a breeze to build and execute complex text generation workflows, unlocking the potential of Large Language Models for your projects.
//!
//! # What's Inside? 🎁
//!
//! With `llm-chain-llama`, you'll be able to:
//!
//! - Generate text using LLaMA models
//! - Create custom text summarization workflows
//! - Perform complex tasks by chaining together different prompts and models 🧠
//!
//!
//! # Examples 📚
//!
//! Dive into the examples folder to discover how to harness the power of this crate. You'll find detailed examples that showcase how to generate text using LLaMA models, as well as how to chain the prompts together to create more complex workflows.
//!
//! So gear up, and let llm-chain-llama elevate your applications to new heights! With the combined powers of Large Language Models and the LLaMA framework, there's no limit to what you can achieve. 🌠🎊
//!
//! Happy coding, and enjoy the amazing world of LLMs with llm-chain-llama! 🥳🚀

mod context;
mod executor;
mod output;
mod step;
mod tokenizer;
mod instruct;

pub use executor::Executor;
pub use output::Output;
pub use step::{Step, LlamaConfig};
pub use context::LlamaContextParams;
pub use instruct::new_instruct_template;