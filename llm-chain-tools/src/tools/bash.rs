use crate::collection::ToolUseError;
use crate::description::{Describe, Format, ToolDescription};
use crate::tool::{gen_invoke_function, Tool};
use serde::{Deserialize, Serialize};
use std::process::Command;

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

impl BashTool {
    fn invoke_typed(&self, input: &BashToolInput) -> Result<BashToolOutput, ToolUseError> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(&input.cmd)
            .output()
            .map_err(|e| ToolUseError::ToolInvocationFailed(e.to_string()))?;

        Ok(BashToolOutput {
            status: output.status.code().unwrap() as isize,
            stderr: String::from_utf8(output.stderr).unwrap(),
            stdout: String::from_utf8(output.stdout).unwrap(),
        })
    }
}

impl Tool for BashTool {
    gen_invoke_function!();
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
