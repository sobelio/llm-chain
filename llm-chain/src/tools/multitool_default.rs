use crate::multitool;
use crate::tools::tools::{
    BashTool, BashToolError, BashToolInput, BashToolOutput, ExitTool, ExitToolError, ExitToolInput,
    ExitToolOutput, PythonTool, PythonToolError, PythonToolInput, PythonToolOutput,
};
use crate::tools::{Tool, ToolDescription, ToolError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

multitool!(
    DefaultToolbox,
    DefaultToolboxInput,
    DefaultToolboxOutput,
    DefaultToolboxError,
    BashTool,
    BashToolInput,
    BashToolOutput,
    BashToolError,
    ExitTool,
    ExitToolInput,
    ExitToolOutput,
    ExitToolError,
    PythonTool,
    PythonToolInput,
    PythonToolOutput,
    PythonToolError
);
