//! # 🚀 llm-chain 🚀
//!
//! `llm-chain` is the *ultimate* toolbox for developers looking to supercharge their applications with the power of Large Language Models (LLMs)! 🎉
//!
//! This versatile crate lets you chain together LLMs, making it incredibly useful for:
//! - Effortlessly summarizing lengthy documents 📚
//! - Allow your bots to interact with the environment using tools.
//! - Seamlessly chaining together multiple prompts to tackle complex tasks.
//! - ChatGPT model support
//! - LLaMA model support

//!
//! And that's not all! `llm-chain` is also your best friend when it comes to creating and managing prompts for LLMs. Say goodbye to hassle and bloated syntax! Quickly create and manage prompts with our intuitive templating system, and let `llm-chain` handle the rest! 🤩
//!
//! Keep in mind that this crate is a library, which means it doesn't include any LLMs out of the box. But fear not! We also offer the [llm-chain-openai](https://crates.io/crates/llm-chain-openai) crate, which brings the power of OpenAI's LLMs right to your fingertips! 🪄 We recommend starting with that crate to make the most of `llm-chain`. 😉
//!
//! So, buckle up and dive into the amazing world of LLMs! Unlock the full potential of your applications with `llm-chain` and watch them soar! 🌈💥
//!
//! To help you get started, we've prepared a comprehensive [tutorial](https://docs.llm-chain.xyz/docs/category/tutorial) that will guide you through the process of using `llm-chain`. The tutorial covers everything from installation to advanced usage, so you'll be well-equipped to make the most of this powerful toolbox.
//!
//! Happy coding, and may your LLM adventures be both exciting and productive! 🥳🚀
//!

// Core components
pub mod agents;
pub mod chains;
pub mod document_stores;
pub mod executor;
pub mod frame;
pub mod options;
pub mod output;
pub mod parameters;
pub mod parsing;
pub mod prompt;
pub mod schema;
pub mod serialization;
pub mod step;
pub mod tokens;
pub mod tools;
pub mod traits;

// Utilities and tools
pub mod summarization;

// Re-exports for convenient usage
pub use parameters::Parameters;
