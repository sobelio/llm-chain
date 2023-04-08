use std::fmt::{Display, Formatter};

/// Represents the output from the LLAMA model.
#[derive(Debug, Clone)]
pub struct Output {
    output: String,
}

impl Output {
    /// Combines two `Output` instances by concatenating their strings with a newline.
    ///
    /// # Arguments
    ///
    /// * `other` - The other `Output` instance to combine with the current instance.
    ///
    /// # Returns
    ///
    /// A new `Output` instance with the combined string.
    pub fn combine(&self, other: &Output) -> Output {
        Output {
            output: format!("{}\n{}", &self.output, &other.output),
        }
    }
}

/// Implements the `Display` trait for the `Output` struct.
impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output)
    }
}

/// Implements the `From<Output> for String` conversion trait.
impl From<Output> for String {
    fn from(output: Output) -> Self {
        output.output
    }
}

/// Implements the `From<String> for Output` conversion trait.
impl From<String> for Output {
    fn from(output: String) -> Self {
        Output { output }
    }
}

/// Implements the `From<&str> for Output` conversion trait.
impl From<&str> for Output {
    fn from(output: &str) -> Self {
        Output {
            output: output.to_string(),
        }
    }
}

/// Implements the `From<Output> for Box<str>` conversion trait.
///
/// Note: This implementation returns `Box<str>` instead of `&str` to avoid borrowing issues.
impl From<Output> for Box<str> {
    fn from(output: Output) -> Self {
        output.output.into_boxed_str()
    }
}
