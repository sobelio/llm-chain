use serde::{ser::SerializeMap, Serialize, Serializer};

/// Represents a single parameter for a tool.
#[derive(Clone, Debug)]
pub struct FormatPart {
    pub key: String,
    pub purpose: String,
}

impl FormatPart {
    /// Creates a new `FormatPart` with the given key and purpose.
    pub fn new(key: &str, purpose: &str) -> Self {
        FormatPart {
            key: key.to_string(),
            purpose: purpose.to_string(),
        }
    }
}

impl<K: Into<String>, P: Into<String>> From<(K, P)> for FormatPart {
    fn from((k, p): (K, P)) -> Self {
        FormatPart::new(&k.into(), &p.into())
    }
}

/// Represents the format for a tool's input or output.
#[derive(Debug)]
pub struct Format {
    pub parts: Vec<FormatPart>,
}

impl Format {
    /// Creates a new `Format` with the given parts.
    pub fn new(parts: Vec<FormatPart>) -> Self {
        Format { parts }
    }
}

impl<T: AsRef<[FormatPart]>> From<T> for Format {
    fn from(parts: T) -> Self {
        Format::new(parts.as_ref().to_vec())
    }
}

impl Serialize for Format {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let n = self.parts.len();
        let mut map = serializer.serialize_map(Some(n))?;
        for part in &self.parts {
            map.serialize_entry(&part.key, &part.purpose)?;
        }
        map.end()
    }
}

/// A trait to provide a description format for a tool.
pub trait Describe {
    fn describe() -> Format;
}

/// Represents the description of a tool, including its name, usage, and input/output formats.
#[derive(Serialize, Debug)]
pub struct ToolDescription {
    pub name: String,
    pub description: String,
    pub description_context: String,
    pub input_format: Format,
    // #[serde(skip)]
    // #[allow(dead_code)]
    /// This will be used in the future.
    pub output_format: Format,
}

impl ToolDescription {
    /// Creates a new `ToolDescription` with the given name, description, context, and formats.
    pub fn new(
        name: &str,
        description: &str,
        description_context: &str,
        input_format: Format,
        output_format: Format,
    ) -> Self {
        ToolDescription {
            name: name.to_string(),
            description: description.to_string(),
            description_context: description_context.to_string(),
            input_format,
            output_format,
        }
    }
}
