use serde::{ser::SerializeMap, Serialize, Serializer};

#[derive(Clone)]
/// A description of a parameter for a tool.
pub struct FormatPart {
    key: String,
    purpose: String,
}

impl FormatPart {
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

pub struct Format {
    parts: Vec<FormatPart>,
}

impl Format {
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

pub trait Describe {
    fn describe() -> Format;
}

#[derive(Serialize)]
/// A description of a tool, used to prompt the model
pub struct ToolDescription {
    pub(crate) name: String,
    description: String,
    description_context: String,
    input_format: Format,
    #[serde(skip)]
    #[allow(dead_code)]
    output_format: Format,
}

impl ToolDescription {
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
