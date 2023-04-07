use crate::Parameters;
use dynfmt::{Format, SimpleCurlyFormat};

fn apply_formatting(template: &str, parameters: &Parameters) -> String {
    SimpleCurlyFormat {}
        .format(template, parameters)
        .unwrap()
        .to_string()
}

/// A template for a prompt. This is a string that can be formatted with a set of parameters.
///
/// # Examples
/// **Using the default key**
/// ```
/// use llm_chain::{PromptTemplate, Parameters};
/// let template: PromptTemplate = "Hello {}!".into();
/// let parameters: Parameters = "World".into();
/// assert_eq!(template.format(&parameters), "Hello World!".to_string());
/// ```
/// **Using a custom key**
/// ```
/// use llm_chain::{PromptTemplate, Parameters};
/// let template: PromptTemplate = "Hello {name}!".into();
/// let parameters: Parameters = vec![("name", "World")].into();
/// assert_eq!(template.format(&parameters), "Hello World!".to_string());
/// ```
#[derive(Clone)]
pub struct PromptTemplate {
    template: String,
}

impl PromptTemplate {
    /// Create a new prompt template from a string.
    pub fn new(template: String) -> PromptTemplate {
        PromptTemplate { template }
    }
    /// Format the template with the given parameters.
    pub fn format(&self, parameters: &Parameters) -> String {
        apply_formatting(&self.template, parameters)
    }
}

impl<T: Into<String>> From<T> for PromptTemplate {
    fn from(template: T) -> Self {
        Self::new(template.into())
    }
}
