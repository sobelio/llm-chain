use crate::tools::description::{Describe, Format, ToolDescription};
use crate::tools::tool::{gen_invoke_function, Tool, ToolError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A tool that exits the program with the given status code.
pub struct ExitTool {}

impl ExitTool {
    /// Creates a new `ExitTool`.
    pub fn new() -> Self {
        ExitTool {}
    }
}

impl Default for ExitTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the input for `ExitTool`.
#[derive(Serialize, Deserialize)]
pub struct ExitToolInput {
    status_code: i32,
}

/// Represents the output for `ExitTool`.
#[derive(Serialize, Deserialize)]
pub struct ExitToolOutput {}

impl Describe for ExitToolInput {
    fn describe() -> Format {
        vec![("status_code", "<integer> UNIX status to exit with").into()].into()
    }
}

impl Describe for ExitToolOutput {
    fn describe() -> Format {
        vec![].into()
    }
}

#[derive(Debug, Error)]
pub enum ExitToolError {
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl ToolError for ExitToolError {}

impl ExitTool {
    /// Invokes the `ExitTool` with the provided input.
    fn invoke_typed(&self, input: &ExitToolInput) -> Result<ExitToolOutput, ExitToolError> {
        std::process::exit(input.status_code);
    }
}

impl Tool for ExitTool {
    type Error = ExitToolError;
    gen_invoke_function!();

    /// Returns a `ToolDescription` for `ExitTool`.
    fn description(&self) -> ToolDescription {
        ToolDescription::new(
            "ExitTool",
            "Exits the program with the given status code",
            "Use this when your task is complete and you want to exit the program.",
            ExitToolInput::describe(),
            ExitToolOutput::describe(),
        )
    }
}
