use super::description::ToolDescription;

macro_rules! gen_invoke_function {
    () => {
        fn invoke(&self, input: serde_yaml::Value) -> Result<serde_yaml::Value, Self::Error> {
            let input = serde_yaml::from_value(input)
                .map_err(|e| <serde_yaml::Error as Into<Self::Error>>::into(e))?;
            let output = self.invoke_typed(&input)?;
            Ok(serde_yaml::to_value(output)?)
        }
    };
}
pub(crate) use gen_invoke_function;

/// Marker trait is required so we do not use ToolUseError in our trait; this will allow users to easily define their own tools with their own errors
pub trait ToolError {}

/// The `Tool` trait defines an interface for tools that can be added to a `ToolCollection`.
///
/// A `Tool` is a function that takes a YAML-formatted input and returns a YAML-formatted output.
/// It has a description that contains metadata about the tool, such as its name and usage.
pub trait Tool {
    type Error: std::fmt::Debug + std::error::Error + ToolError + From<serde_yaml::Error>;

    /// Returns the `ToolDescription` containing metadata about the tool.
    fn description(&self) -> ToolDescription;

    /// Invokes the tool with the given YAML-formatted input.
    ///
    /// # Errors
    ///
    /// Returns an `ToolUseError` if the input is not in the expected format or if the tool
    /// fails to produce a valid output.
    fn invoke(&self, input: serde_yaml::Value) -> Result<serde_yaml::Value, Self::Error>;

    /// Checks whether the tool matches the given name.
    ///
    /// This function is used to find the appropriate tool in a `ToolCollection` based on its name.
    fn matches(&self, name: &str) -> bool {
        self.description().name == name
    }
}
