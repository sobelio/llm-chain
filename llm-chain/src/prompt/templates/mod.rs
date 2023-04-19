mod legacy;
#[cfg(feature = "tera")]
mod tera;

mod error;
pub use error::PromptTemplateError;
use error::PromptTemplateErrorImpl;
use std::fmt;
#[cfg(feature = "serialization")]
mod io;

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::Parameters;

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
#[derive(Clone, Debug)]
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
    #[deprecated(note = "Use PromptTemplate::tera or llm_chain::prompt! instead")]
    /// Create a new prompt template from a string.
    pub fn new<K: Into<String>>(template: K) -> PromptTemplate {
        Self::legacy(template)
    }
    pub fn legacy<K: Into<String>>(template: K) -> PromptTemplate {
        PromptTemplateImpl::legacy(template).into()
    }

    /// Format the template with the given parameters.
    pub fn format(&self, parameters: &Parameters) -> Result<String, PromptTemplateError> {
        self.0.format(parameters).map_err(|e| e.into())
    }
    /// Creates a non-dynmamic prompt template, useful for untrusted inputs.
    pub fn static_string<K: Into<String>>(template: K) -> PromptTemplate {
        PromptTemplateImpl::static_string(template.into()).into()
    }

    #[cfg(feature = "tera")]
    /// Creates a prompt template that uses the Tera templating engine.
    /// This is only available if the `tera` feature is enabled, which it is by default.
    /// # Examples
    ///
    /// ```rust
    /// use llm_chain::{PromptTemplate, Parameters};
    /// let template = PromptTemplate::tera("Hello {{name}}!");
    /// let parameters: Parameters = vec![("name", "World")].into();
    /// assert_eq!(template.format(&parameters).unwrap(), "Hello World!");
    /// ```
    pub fn tera<K: Into<String>>(template: K) -> PromptTemplate {
        PromptTemplateImpl::tera(template.into()).into()
    }

    #[cfg(feature = "tera")]
    /// Creates a prompt template from a file. The file should be a text file containing the template as a tera template.
    /// # Examples
    /// ```no_run
    /// use llm_chain::PromptTemplate;
    /// let template = PromptTemplate::from_file("template.txt").unwrap();
    /// ```
    pub fn from_file<K: AsRef<std::path::Path>>(path: K) -> Result<PromptTemplate, std::io::Error> {
        io::read_prompt_template_file(path)
    }

    /// Combines two prompt templates into one.
    /// This is useful for creating a prompt template from multiple sources.
    /// # Examples
    /// ```
    /// use llm_chain::{PromptTemplate, Parameters};
    /// let template1 = PromptTemplate::tera("Hello {{name}}");
    /// let template2 = PromptTemplate::new("!");
    /// let template3 = PromptTemplate::combine(vec![template1, template2]);
    /// let parameters: Parameters = vec![("name", "World")].into();
    /// assert_eq!(template3.format(&parameters).unwrap(), "Hello World!");
    /// ```
    pub fn combine(parts: Vec<PromptTemplate>) -> PromptTemplate {
        let res = parts.into_iter().map(|p| p.0).collect();
        PromptTemplateImpl::combine(res).into()
    }
}

impl<T: Into<String>> From<T> for PromptTemplate {
    fn from(template: T) -> Self {
        Self::legacy(template.into())
    }
}

impl fmt::Display for PromptTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The actual implementation of the prompt template. This hides the implementation details from the user.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
enum PromptTemplateImpl {
    Static(String),
    Legacy(legacy::PromptTemplate),
    #[cfg(feature = "tera")]
    Tera(String),
    Combined(Vec<PromptTemplateImpl>),
}

impl PromptTemplateImpl {
    /// Create a new prompt template from a string.
    pub fn legacy<K: Into<String>>(template: K) -> PromptTemplateImpl {
        PromptTemplateImpl::Legacy(legacy::PromptTemplate::new(template))
    }

    pub fn format(&self, parameters: &Parameters) -> Result<String, PromptTemplateErrorImpl> {
        match self {
            PromptTemplateImpl::Static(template) => Ok(template.clone()),
            PromptTemplateImpl::Legacy(template) => template
                .format(parameters)
                .map_err(PromptTemplateErrorImpl::LegacyTemplateError),
            #[cfg(feature = "tera")]
            PromptTemplateImpl::Tera(template) => {
                tera::render(template, parameters).map_err(|e| e.into())
            }
            PromptTemplateImpl::Combined(templates) => {
                let mut result = String::new();
                for template in templates {
                    result.push_str(&template.format(parameters)?);
                }
                Ok(result)
            }
        }
    }

    pub fn static_string(template: String) -> PromptTemplateImpl {
        PromptTemplateImpl::Static(template)
    }

    #[cfg(feature = "tera")]
    pub fn tera(template: String) -> PromptTemplateImpl {
        PromptTemplateImpl::Tera(template)
    }

    pub fn combine(templates: Vec<PromptTemplateImpl>) -> PromptTemplateImpl {
        PromptTemplateImpl::Combined(templates)
    }
}

impl fmt::Display for PromptTemplateImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PromptTemplateImpl::Static(s) => write!(f, "{}", s),
            PromptTemplateImpl::Legacy(template) => write!(f, "{}", template),
            #[cfg(feature = "tera")]
            PromptTemplateImpl::Tera(template) => write!(f, "{}", template),
            PromptTemplateImpl::Combined(templates) => {
                for template in templates {
                    write!(f, "{}", template)?;
                }
                Ok(())
            }
        }
    }
}
