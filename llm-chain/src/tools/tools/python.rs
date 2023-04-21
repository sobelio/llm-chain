use crate::tools::description::{Describe, Format, ToolDescription};
use crate::tools::tool::{Tool, ToolError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::process::Command;
use thiserror::Error;

pub struct PythonTool {}

impl PythonTool {
    pub fn new() -> Self {
        PythonTool {}
    }
}

impl Default for PythonTool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct PythonToolInput {
    code: String,
}

#[derive(Serialize, Deserialize)]
pub struct PythonToolOutput {
    result: String,
    stderr: String,
}

impl Describe for PythonToolInput {
    fn describe() -> Format {
        vec![("code", "The Python code to execute.").into()].into()
    }
}

impl Describe for PythonToolOutput {
    fn describe() -> Format {
        vec![
            ("result", "The result of the executed Python code.").into(),
            ("stderr", "The stderr output of the Python code execution.").into(),
        ]
        .into()
    }
}

#[derive(Debug, Error)]
pub enum PythonToolError {
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl ToolError for PythonToolError {}

#[async_trait]
impl Tool for PythonTool {
    type Input = PythonToolInput;
    type Output = PythonToolOutput;
    type Error = PythonToolError;

    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let output = Command::new("python3")
            .arg("-c")
            .arg(&input.code)
            .output()?;
        Ok(PythonToolOutput {
            result: String::from_utf8(output.stdout).unwrap(),
            stderr: String::from_utf8(output.stderr).unwrap(),
        })
    }

    fn description(&self) -> ToolDescription {
        ToolDescription::new(
            "PythonTool",
            "A tool that executes Python code.",
            "Use this to execute Python code to solve your goals",
            PythonToolInput::describe(),
            PythonToolOutput::describe(),
        )
    }
}
