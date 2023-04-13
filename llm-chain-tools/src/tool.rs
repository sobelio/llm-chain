use crate::collection::ToolUseError;
use crate::description::ToolDescription;

macro_rules! gen_invoke_function {
    () => {
        fn invoke(
            &self,
            input: serde_yaml::Value,
        ) -> Result<serde_yaml::Value, crate::collection::ToolUseError> {
            let input = serde_yaml::from_value(input)?;
            let output = self.invoke_typed(&input)?;
            Ok(serde_yaml::to_value(output)?)
        }
    };
}
pub(crate) use gen_invoke_function;

/// The `Tool` trait defines an interface for tools that can be added to a `ToolCollection`.
///
/// A `Tool` is a function that takes a YAML-formatted input and returns a YAML-formatted output.
/// It has a description that contains metadata about the tool, such as its name and usage.
pub trait Tool {
    /// Returns the `ToolDescription` containing metadata about the tool.
    fn description(&self) -> ToolDescription;

    /// Invokes the tool with the given YAML-formatted input.
    ///
    /// # Errors
    ///
    /// Returns an `ToolUseError` if the input is not in the expected format or if the tool
    /// fails to produce a valid output.
    fn invoke(&self, input: serde_yaml::Value) -> Result<serde_yaml::Value, ToolUseError>;

    /// Checks whether the tool matches the given name.
    ///
    /// This function is used to find the appropriate tool in a `ToolCollection` based on its name.
    fn matches(&self, name: &str) -> bool {
        self.description().name == name
    }
}
