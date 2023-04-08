use crate::description::{Describe, Format, ToolDescription};
use crate::tool::{gen_invoke_function, Tool};
use serde::{Deserialize, Serialize};
use std::process::Command;

pub struct PythonTool {}

impl PythonTool {
    pub fn new() -> Self {
        PythonTool {}
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

impl PythonTool {
    fn invoke_typed(&self, input: &PythonToolInput) -> Result<PythonToolOutput, String> {
        let output = Command::new("python3")
            .arg("-c")
            .arg(&input.code)
            .output()
            .map_err(|_e| "failed to execute process")?;
        Ok(PythonToolOutput {
            result: String::from_utf8(output.stdout).unwrap(),
            stderr: String::from_utf8(output.stderr).unwrap(),
        })
    }
}

impl Tool for PythonTool {
    gen_invoke_function!();
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
