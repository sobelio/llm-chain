#[macro_export]
macro_rules! multitool {
    ($multitool:ident, $input:ident, $output:ident, $error:ident, $($tool:ident, $tool_input:ident, $tool_output:ident, $tool_error:ident),+) => {
        #[derive(Serialize, Deserialize)]
        enum $input {
            $($tool_input($tool_input)),+
        }

        $(
            impl From<$tool_input> for $input {
                fn from(tool: $tool_input) -> Self {
                    $input::$tool_input(tool)
                }
            }
        )+

        $(
            impl TryInto<$tool_input> for $input {
                type Error = $error;
                fn try_into(self) -> Result<$tool_input, Self::Error> {
                    if let $input::$tool_input(t) = self {
                        Ok(t)
                    } else {
                        Err($error::BadVariant)
                    }
                }
            }
        )+

        #[derive(Serialize, Deserialize)]
        enum $output {
            $($tool_output($tool_output)),+
        }

        $(
            impl From<$tool_output> for $output {
                fn from(tool: $tool_output) -> Self {
                    $output::$tool_output(tool)
                }
            }
        )+

        $(
            impl TryInto<$tool_output> for $output {
                type Error = $error;
                fn try_into(self) -> Result<$tool_output, Self::Error> {
                    if let $output::$tool_output(t) = self {
                        Ok(t)
                    } else {
                        Err($error::BadVariant)
                    }
                }
            }
        )+

        #[derive(Debug, Error)]
        enum $error {
            #[error("Could not convert")]
            BadVariant,
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

        #[async_trait]
        impl Tool for $multitool {
            type Input = $input;
            type Output = $output;
            type Error = $error;

            async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {

                match (self, input) {
                    $(($multitool::$tool(t), $input::$tool_input(i)) => {
                            t.invoke_typed(i).await.map(|o| <$tool_output as Into<Self::Output>>::into(o)).map_err(|e| e.into())
                        }
                    ),+
                    _ => Err($error::BadVariant)
                }
            }

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
            async fn invoke(&self, input: serde_yaml::Value) -> Result<serde_yaml::Value, Self::Error> {
                match self {
                    $($multitool::$tool(t) => t.invoke(input).await.map_err(|e| e.into())),+
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
