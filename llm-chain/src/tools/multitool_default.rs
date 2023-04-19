use crate::toolbox;
use crate::tools::tools::{
    BashTool, BashToolError, ExitTool, ExitToolError, PythonTool, PythonToolError,
};
use crate::tools::{Tool, ToolDescription, ToolError};
use thiserror::Error;

toolbox!(
    DefaultToolbox,
    DefaultToolboxError,
    BashTool,
    BashToolError,
    ExitTool,
    ExitToolError,
    PythonTool,
    PythonToolError
);
