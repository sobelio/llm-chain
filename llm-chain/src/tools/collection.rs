use super::tool::{Tool, ToolError};
use crate::parsing::{find_yaml, ExtractionError};
use crate::prompt::StringTemplate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct ToolCollection<T> {
    tools: Vec<T>,
}

#[derive(Error, Debug)]
pub enum ToolUseError<E: ToolError> {
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
    #[error(transparent)]
    ToolError(#[from] E),
}

impl<T> ToolCollection<T>
where
    T: Tool + Send + Sync,
{
    pub fn new() -> Self {
        Self { tools: vec![] }
    }

    pub fn add_tool(&mut self, tool: T) {
        self.tools.push(tool);
    }

    pub async fn invoke(
        &self,
        name: &str,
        input: &serde_yaml::Value,
    ) -> Result<serde_yaml::Value, ToolUseError<<T as Tool>::Error>> {
        let tool = self
            .tools
            .iter()
            .find(|t| t.matches(name))
            .ok_or(ToolUseError::ToolNotFound)?;
        tool.invoke(input.clone()).await.map_err(|e| e.into())
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
    pub async fn process_chat_input(
        &self,
        data: &str,
    ) -> Result<String, ToolUseError<<T as Tool>::Error>> {
        let tool_invocations: Vec<ToolInvocationInput> = find_yaml::<ToolInvocationInput>(data)?;
        if tool_invocations.len() != 1 {
            return Err(ToolUseError::InvalidInvocation);
        }
        let output = self
            .invoke(&tool_invocations[0].command, &tool_invocations[0].input)
            .await?;
        serde_yaml::to_string(&output).map_err(|e| e.into())
    }

    /// Generate a YAML-formatted string describing the available tools.
    pub fn describe(&self) -> Result<String, ToolUseError<<T as Tool>::Error>> {
        let des: Vec<_> = self.tools.iter().map(|t| t.description()).collect();
        serde_yaml::to_string(&des).map_err(|e| e.into())
    }

    /// Generate a prompt template for the tool collection. Combine it with a normal prompt template to perform your task.
    pub fn to_prompt_template(&self) -> Result<StringTemplate, ToolUseError<<T as Tool>::Error>> {
        Ok(StringTemplate::combine(vec![
            StringTemplate::static_string(include_str!("./tool_prompt_prefix.txt").to_string()),
            StringTemplate::static_string(self.describe()?),
            StringTemplate::static_string("\n\n"),
        ]))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolInvocationInput {
    command: String,
    input: serde_yaml::Value,
}
