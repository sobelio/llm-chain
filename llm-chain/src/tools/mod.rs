//! # Tool Access Module
//!
//! This crate provides a collection of tools that can be used to grant the LLM (Large Language Model) access to various utilities, such as running Bash commands on your computer or performing web searches.
//!
//! The main components of this module are:
//!
//! - `Tool`: A struct that represents an individual tool that the LLM can use.
//! - `ToolCollection`: A collection of `Tool` instances.
//! - `create_tool_prompt_segment`: A function to create a prompt that indicates the model should use the provided tools.
//!
//! ## Example
//!
//! ```rust
//! use llm_chain::tools::{ToolCollection, tools::BashTool};
//! use llm_chain::prompt::StringTemplate;
//! use std::boxed::Box;
//!
//! // Create a ToolCollection with a tool.
//! let mut tc = ToolCollection::new();
//! tc.add_tool(BashTool::new());
//!
//! // Create a prompt indicating the LLM should use the provided tools.
//! let prompt = StringTemplate::static_string("Find information about Rust programming language.");
//! let tool_prompt = StringTemplate::combine(vec![tc.to_prompt_template().unwrap(), prompt]);
//! ```
//!
//! ## Modules
//!
//! - `tools`: A submodule that provides a variety of pre-defined tools.

mod collection;
mod description;
#[cfg(feature = "multitool_default")]
pub mod multitool_default;
pub use description::{Describe, Format, FormatPart, ToolDescription};
pub mod multitool;
mod tool;
pub mod tools;

pub use collection::{ToolCollection, ToolInvocationInput, ToolUseError};
pub use tool::{Tool, ToolError};
