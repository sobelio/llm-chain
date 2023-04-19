use crate::multitool;
use crate::tools::tools::{
    BashTool, BashToolError, ExitTool, ExitToolError, PythonTool, PythonToolError,
};
use crate::tools::{Tool, ToolDescription, ToolError};
use thiserror::Error;

multitool!(
    DefaultToolbox,
    DefaultToolboxError,
    BashTool,
    BashToolError,
    ExitTool,
    ExitToolError,
    PythonTool,
    PythonToolError
);
