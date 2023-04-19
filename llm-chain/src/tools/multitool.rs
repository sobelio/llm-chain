#[macro_export]
macro_rules! multitool {
    ($multitool:ident, $error:ident, $($tool:ident, $tool_error:ident),+) => {
        #[derive(Debug, Error)]
        enum $error {
            #[error(transparent)]
            YamlError(#[from] serde_yaml::Error),
            $(#[error(transparent)]
            $tool_error(#[from] $tool_error)),+
        }

        impl ToolError for $error {}

        enum $multitool {
            $($tool($tool)),+
        }

        $(
            impl From<$tool> for $multitool {
                fn from(tool: $tool) -> Self {
                    $multitool::$tool(tool)
                }
            }
        )+

        impl Tool for $multitool {
            type Error = $error;

            /// Returns the `ToolDescription` containing metadata about the tool.
            fn description(&self) -> ToolDescription {
                match self {
                    $($multitool::$tool(t) => t.description()),+
                }
            }

            /// Invokes the tool with the given YAML-formatted input.
            ///
            /// # Errors
            ///
            /// Returns an `ToolUseError` if the input is not in the expected format or if the tool
            /// fails to produce a valid output.
            fn invoke(&self, input: serde_yaml::Value) -> Result<serde_yaml::Value, Self::Error> {
                match self {
                    $($multitool::$tool(t) => t.invoke(input).map_err(|e| e.into())),+
                }
            }

            /// Checks whether the tool matches the given name.
            ///
            /// This function is used to find the appropriate tool in a `ToolCollection` based on its name.
            fn matches(&self, name: &str) -> bool {
                match self {
                    $($multitool::$tool(t) => t.description().name == name),+
                }
            }
        }
    };
}
