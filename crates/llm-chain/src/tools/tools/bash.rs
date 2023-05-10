use crate::tools::description::{Describe, Format, ToolDescription};
use crate::tools::tool::{Tool, ToolError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::num::TryFromIntError;
use std::process::Command;
use std::string::FromUtf8Error;
use thiserror::Error;

/// A tool that executes a bash command.
pub struct BashTool {}

impl BashTool {
    pub fn new() -> Self {
        BashTool {}
    }
}

impl Default for BashTool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct BashToolInput {
    cmd: String,
}

#[derive(Serialize, Deserialize)]
pub struct BashToolOutput {
    stderr: String,
    stdout: String,
    status: isize,
}

impl Describe for BashToolInput {
    fn describe() -> Format {
        vec![("cmd", "The command to execute in the bash shell.").into()].into()
    }
}

impl Describe for BashToolOutput {
    fn describe() -> Format {
        vec![
            ("result", "Exit code 0 == success").into(),
            ("stderr", "The stderr output of the command").into(),
            ("stdout", "The stdout output of the command").into(),
        ]
        .into()
    }
}

#[derive(Debug, Error)]
pub enum BashToolError {
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Type `isize` overflowed when reading output status code {0}")]
    OutputStatusCodeOverflow(#[from] TryFromIntError),
    #[error("Received a None status code, which means the program was exited by signal")]
    ProcessTerminatedBySignal,
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
}

impl ToolError for BashToolError {}

#[async_trait]
impl Tool for BashTool {
    type Input = BashToolInput;
    type Output = BashToolOutput;
    type Error = BashToolError;
    async fn invoke_typed(&self, input: &BashToolInput) -> Result<BashToolOutput, BashToolError> {
        let output = Command::new("bash").arg("-c").arg(&input.cmd).output()?;

        Ok(BashToolOutput {
            status: output
                .status
                .code()
                .ok_or(BashToolError::ProcessTerminatedBySignal)?
                .try_into()?,
            stderr: String::from_utf8(output.stderr)?,
            stdout: String::from_utf8(output.stdout)?,
        })
    }

    fn description(&self) -> ToolDescription {
        ToolDescription::new(
            "BashTool",
            "A tool that executes a bash command.",
            "Use this to execute local commands to solve your goals",
            BashToolInput::describe(),
            BashToolOutput::describe(),
        )
    }
}
