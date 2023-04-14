use crate::Parameters;
use dynfmt::{Format, SimpleCurlyFormat};
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

fn apply_formatting<'l>(template: &'l str, parameters: &Parameters) -> Result<String, String> {
    SimpleCurlyFormat {}
        .format(template, parameters)
        .map_err(|e| e.to_string())
        .map(|s| s.to_string())
}

/// A template for a prompt. This is a string that can be formatted with a set of parameters.
///
/// # Examples
/// **Using the default key**
/// ```
/// use llm_chain::{PromptTemplate, Parameters};
/// let template: PromptTemplate = "Hello {}!".into();
/// let parameters: Parameters = "World".into();
/// assert_eq!(template.format(&parameters).unwrap(), "Hello World!");
/// ```
/// **Using a custom key**
/// ```
/// use llm_chain::{PromptTemplate, Parameters};
/// let template: PromptTemplate = "Hello {name}!".into();
/// let parameters: Parameters = vec![("name", "World")].into();
/// assert_eq!(template.format(&parameters).unwrap(), "Hello World!");
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct PromptTemplate {
    template: String,
}

impl PromptTemplate {
    /// Create a new prompt template from a string.
    pub fn new<K: Into<String>>(template: K) -> PromptTemplate {
        PromptTemplate {
            template: template.into(),
        }
    }
    /// Format the template with the given parameters.
    pub fn format(&self, parameters: &Parameters) -> Result<String, String> {
        apply_formatting(&self.template, parameters)
    }
}

impl<T: Into<String>> From<T> for PromptTemplate {
    fn from(template: T) -> Self {
        Self::new(template.into())
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_formatting, Parameters, PromptTemplate};
    #[test]
    fn test_apply_formatting() {
        let template = "Hello {name}!";
        let parameters = vec![("name", "World")].into();
        assert_eq!(
            apply_formatting(template, &parameters).unwrap(),
            "Hello World!"
        );
    }

    #[test]
    fn test_prompt_template_format() {
        let template: PromptTemplate = "Hello {name}!".into();
        let parameters = vec![("name", "World")].into();
        assert_eq!(template.format(&parameters).unwrap(), "Hello World!");
    }

    #[test]
    fn test_prompt_template_format_with_default_key() {
        let template: PromptTemplate = "Hello {}!".into();
        let parameters: Parameters = "World".into();
        assert_eq!(template.format(&parameters).unwrap(), "Hello World!");
    }

    #[test]
    fn test_prompt_template_format_with_multiple_keys() {
        let template: PromptTemplate = "Hello {name}, you are {age} years old.".into();
        let parameters: Parameters = vec![("name", "John"), ("age", "30")].into();
        assert_eq!(
            template.format(&parameters).unwrap(),
            "Hello John, you are 30 years old."
        );
    }
}
