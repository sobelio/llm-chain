//! # llm-chain-openai
//!
//! Supercharge your applications with the power of OpenAI's API, and welcome to the world of `llm-chain-openai`! This crate brings the incredible ChatGPT model to `lang-chain`, making it a breeze to generate human-like text.
//!
//! Designed to work seamlessly with the [llm-chain](https://crates.io/crates/llm-chain) crate, `llm-chain-openai` allows you to take full advantage of the OpenAI API, unlocking the potential of Large Language Models for your projects.
//!
//! To get started, you'll need an OpenAI API key. Don't have one yet? No problem! You can grab one [here](https://beta.openai.com/docs/api-reference/authentication). 🔑
//!
//! # What's Inside? 🎁
//!
//! With `llm-chain-openai`, you'll be able to:
//!
//! - Crate your custom text summarization workflows using the llm-chain crate and the OpenAI API 📚
//! - Perform complex tasks by chaining together different prompts and models 🧠

//! # Examples 📚
//!
//! Dive into the examples folder to discover how to harness the power of this crate. You'll find detailed examples that showcase how to generate text using the ChatGPT model, as well as how to chain the prompts together to create more complex workflows.
//!
//! So gear up, and let llm-chain-openai elevate your applications to new heights! With the combined powers of Large Language Models and the OpenAI API, there's no limit to what you can achieve. 🌠🎊
//!
//! Happy coding, and enjoy the amazing world of LLMs with llm-chain-openai! 🥳🚀P

pub mod chatgpt;
pub mod embeddings;
pub use async_openai;
