use std::any::type_name;

use super::description::ToolDescription;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/// Marker trait for Tool errors. It is needed so the concrete Errors can have a derived `From<ToolError>`
pub trait ToolError {}

/// The `Tool` trait defines an interface for tools that can be added to a `ToolCollection`.
///
/// A `Tool` is a function that takes a YAML-formatted input and returns a YAML-formatted output.
/// It has a description that contains metadata about the tool, such as its name and usage.
#[async_trait]
pub trait Tool {
    // type Input: DeserializeOwned + Send + Sync;
    type Input: DeserializeOwned + Send + Sync + From<String>;
    type Output: Serialize;
    type Error: std::fmt::Debug + std::error::Error + ToolError + From<serde_yaml::Error>;

    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error>;

    /// Returns the `ToolDescription` containing metadata about the tool.
    fn description(&self) -> ToolDescription;


    /// Constructs input form incoming serde_value
    // fn construct_input(input: serde_yaml::Value) -> Result<Self::Input, Self::Error>;

    /// Invokes the tool with the given YAML-formatted input.
    ///
    /// # Errors
    ///
    /// Returns an `ToolUseError` if the input is not in the expected format or if the tool
    /// fails to produce a valid output.
    async fn invoke(&self, input_raw: serde_yaml::Value) -> Result<serde_yaml::Value, Self::Error> {
        let input: Self::Input;
        if let serde_yaml::Value::String(contents) = input_raw {
            input = Self::Input::from(contents);
        }
        else {
            input = serde_yaml::from_value(input_raw)?;
        }

        let output = self.invoke_typed(&input).await?;
        Ok(serde_yaml::to_value(output)?)
    }

    /// Checks whether the tool matches the given name.
    ///
    /// This function is used to find the appropriate tool in a `ToolCollection` based on its name.
    fn matches(&self, name: &str) -> bool {
        self.description().name == name
    }
}
