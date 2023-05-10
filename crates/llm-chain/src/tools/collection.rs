use super::tool::{Tool, ToolError};
use crate::parsing::{find_yaml, ExtractionError};
use crate::prompt::StringTemplate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Default)]
pub struct ToolCollection<T> {
    tools: Vec<T>,
}

#[derive(Error, Debug)]
pub enum ToolUseError<E: ToolError> {
    #[error("Model is not trying to invoke tools")]
    NoToolInvocation,
    #[error("You must output at most one tool invocation")]
    MultipleInvocations,
    #[error("Tool not found")]
    ToolNotFound,
    #[error("You must output YAML: {0}")]
    InvalidYaml(#[from] ExtractionError),
    #[error("Invalid format: {0}")]
    InvalidFormat(#[from] serde_yaml::Error),
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

    pub fn get_tool_invocation(
        &self,
        data: &str,
    ) -> Result<ToolInvocationInput, ToolUseError<<T as Tool>::Error>> {
        let tool_invocations: Vec<ToolInvocationInput> = find_yaml::<ToolInvocationInput>(data)?;
        if tool_invocations.len() > 1 {
            return Err(ToolUseError::MultipleInvocations);
        }
        tool_invocations
            .first()
            .cloned()
            .ok_or(ToolUseError::NoToolInvocation)
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
        let tool_invocation = self.get_tool_invocation(data)?;
        let output = self
            .invoke(&tool_invocation.command, &tool_invocation.input)
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolInvocationInput {
    pub command: String,
    pub input: serde_yaml::Value,
}
