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
//! use llm_chain_tools::{ToolCollection, create_tool_prompt_segment};
//! use llm_chain_tools::tools::BashTool;
//! use std::boxed::Box;
//!
//! // Create a ToolCollection with a tool.
//! let mut tc = ToolCollection::new();
//! tc.add_tool(BashTool::new());
//!
//! // Create a prompt indicating the LLM should use the provided tools.
//! let prompt = "Find information about Rust programming language.";
//! let tool_prompt = create_tool_prompt_segment(&tc, &prompt);
//! ```
//!
//! ## Modules
//!
//! - `tools`: A submodule that provides a variety of pre-defined tools.

mod collection;
mod description;
mod tool;
pub mod tools;
use llm_chain::PromptTemplate;

pub use crate::collection::ToolCollection;
pub use tool::Tool;

/// Creates a prompt that indicates the model should use the tools provided.
///
/// This function takes a reference to a `ToolCollection` and a `&str` prompt, then generates a `PromptTemplate` that includes a prefix and a description of the tools in the collection. This formatted prompt can be passed to the LLM to request its use of the provided tools.
///
/// # Arguments
///
/// * `tc`: A reference to a `ToolCollection` containing the tools to be used by the LLM.
/// * `prompt`: The base prompt to be used in the request.
///
/// # Returns
///
/// A `PromptTemplate` formatted with the provided tools and prompt.
pub fn create_tool_prompt_segment(tc: &ToolCollection, prompt: &str) -> PromptTemplate {
    let prefix = include_str!("./tool_prompt_prefix.txt").to_string();
    let desc = tc.describe();
    (prefix + &desc + "\n\n" + prompt).into()
}
