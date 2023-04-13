use super::tool::Tool;
use crate::parsing::{find_yaml, ExtractionError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct ToolCollection {
    tools: Vec<Box<dyn Tool>>,
}

#[derive(Error, Debug)]
pub enum ToolUseError {
    #[error("Tool not found")]
    ToolNotFound,
    #[error("You must output YAML: {0}")]
    InvalidYaml(#[from] ExtractionError),
    #[error("Invalid format: {0}")]
    InvalidFormat(#[from] serde_yaml::Error),
    #[error("You must output exactly one tool invocation")]
    InvalidInvocation,
    #[error("Tool invocation failed: {0}")]
    ToolInvocationFailed(String),
}

impl ToolCollection {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn add_tool<T: Tool + 'static>(&mut self, tool: T) {
        self.tools.push(Box::new(tool));
    }

    pub fn invoke(
        &self,
        name: &str,
        input: &serde_yaml::Value,
    ) -> Result<serde_yaml::Value, ToolUseError> {
        let tool = self
            .tools
            .iter()
            .find(|t| t.matches(name))
            .ok_or(ToolUseError::ToolNotFound)?;
        tool.invoke(input.clone())
    }

    /// Process chat input and execute the appropriate tool.
    ///
    /// The input string should contain a YAML block describing the tool invocation.
    /// The YAML block should have a `command` field and an `input` field.
    ///
    /// # Errors
    ///
    /// Returns an `OpaqueError` variant if the input is not a valid YAML or
    /// if the specified tool is not found.
    pub fn process_chat_input(&self, data: &str) -> Result<String, ToolUseError> {
        let tool_invocations: Vec<ToolInvocationInput> = find_yaml::<ToolInvocationInput>(data)?;
        if tool_invocations.len() != 1 {
            return Err(ToolUseError::InvalidInvocation);
        }
        let output = self.invoke(&tool_invocations[0].command, &tool_invocations[0].input)?;
        Ok(serde_yaml::to_string(&output).unwrap())
    }

    /// Generate a YAML-formatted string describing the available tools.
    pub fn describe(&self) -> String {
        let des: Vec<_> = self.tools.iter().map(|t| t.description()).collect();
        serde_yaml::to_string(&des).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ToolInvocationInput {
    command: String,
    input: serde_yaml::Value,
}
