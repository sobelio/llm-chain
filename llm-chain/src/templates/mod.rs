mod legacy;

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

pub use legacy::PromptTemplate as LegacyPromptTemplate;

use crate::Parameters;

/// A template for a prompt. This is a string that can be formatted with a set of parameters.
///
/// # Examples
/// **Using the default key**
/// ```
/// use llm_chain::{PromptTemplate, Parameters};
/// let template: PromptTemplate = "Hello {}!".into();
/// let parameters: Parameters = "World".into();
/// assert_eq!(template.format(&parameters), "Hello World!");
/// ```
/// **Using a custom key**
/// ```
/// use llm_chain::{PromptTemplate, Parameters};
/// let template: PromptTemplate = "Hello {name}!".into();
/// let parameters: Parameters = vec![("name", "World")].into();
/// assert_eq!(template.format(&parameters), "Hello World!");
/// ```
#[derive(Clone)]
#[cfg_attr(
    feature = "serialization",
    derive(Serialize, Deserialize),
    serde(transparent)
)]
pub struct PromptTemplate(PromptTemplateImpl);

impl From<PromptTemplateImpl> for PromptTemplate {
    fn from(template: PromptTemplateImpl) -> Self {
        Self(template)
    }
}

impl PromptTemplate {
    /// Create a new prompt template from a string.
    pub fn new<K: Into<String>>(template: K) -> PromptTemplate {
        PromptTemplateImpl::new(template).into()
    }
    /// Format the template with the given parameters.
    pub fn format(&self, parameters: &Parameters) -> String {
        self.0.format(parameters)
    }
    /// Creates a non-dynmamic prompt template, useful for untrusted inputs.
    pub fn static_string<K: Into<String>>(template: K) -> PromptTemplate {
        PromptTemplateImpl::static_string(template.into()).into()
    }
}

impl<T: Into<String>> From<T> for PromptTemplate {
    fn from(template: T) -> Self {
        Self::new(template.into())
    }
}

/// The actual implementation of the prompt template. This hides the implementation details from the user.
#[derive(Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
enum PromptTemplateImpl {
    Static(String),
    Legacy(LegacyPromptTemplate),
}

impl PromptTemplateImpl {
    /// Create a new prompt template from a string.
    pub fn new<K: Into<String>>(template: K) -> PromptTemplateImpl {
        PromptTemplateImpl::Legacy(LegacyPromptTemplate::new(template))
    }

    pub fn format(&self, parameters: &Parameters) -> String {
        match self {
            PromptTemplateImpl::Static(template) => template.clone(),
            PromptTemplateImpl::Legacy(template) => template.format(parameters),
        }
    }

    pub fn static_string(template: String) -> PromptTemplateImpl {
        PromptTemplateImpl::Static(template)
    }
}
