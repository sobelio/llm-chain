mod tera;

mod error;
pub use error::StringTemplateError;
use error::StringTemplateErrorImpl;
use std::fmt;
mod io;

use serde::{Deserialize, Serialize};

use crate::Parameters;

/// A template for a prompt. This is a string that can be formatted with a set of parameters.
///
/// # Examples
/// **Using the default key**
/// ```
/// use llm_chain::prompt::StringTemplate;
/// use llm_chain::Parameters;
/// let template: StringTemplate = "Hello {{ text }}!".into();
/// let parameters: Parameters = "World".into();
/// assert_eq!(template.format(&parameters).unwrap(), "Hello World!");
/// ```
/// **Using a custom key**
/// ```
/// use llm_chain::prompt::StringTemplate;
/// use llm_chain::Parameters;
/// let template: StringTemplate = "Hello {{ name }}!".into();
/// let parameters: Parameters = vec![("name", "World")].into();
/// assert_eq!(template.format(&parameters).unwrap(), "Hello World!");
/// ```
/// ## Tera
/// ```rust
/// use llm_chain::prompt::StringTemplate;
/// use llm_chain::Parameters;
/// let template: StringTemplate = StringTemplate::tera("Hello {{name}}!");
/// let parameters: Parameters = vec![("name", "World")].into();
/// assert_eq!(template.format(&parameters).unwrap(), "Hello World!");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StringTemplate(StringTemplateImpl);

impl From<StringTemplateImpl> for StringTemplate {
    fn from(template: StringTemplateImpl) -> Self {
        Self(template)
    }
}

impl StringTemplate {
    /// Format the template with the given parameters.
    pub fn format(&self, parameters: &Parameters) -> Result<String, error::StringTemplateError> {
        self.0.format(parameters).map_err(|e| e.into())
    }
    /// Creates a non-dynmamic prompt template, useful for untrusted inputs.
    pub fn static_string<K: Into<String>>(template: K) -> StringTemplate {
        StringTemplateImpl::static_string(template.into()).into()
    }

    /// Creates a prompt template that uses the Tera templating engine.
    /// This is only available if the `tera` feature is enabled, which it is by default.
    /// # Examples
    ///
    /// ```rust
    /// use llm_chain::prompt::StringTemplate;
    /// use llm_chain::Parameters;
    /// let template = StringTemplate::tera("Hello {{name}}!");
    /// let parameters: Parameters = vec![("name", "World")].into();
    /// assert_eq!(template.format(&parameters).unwrap(), "Hello World!");
    /// ```
    pub fn tera<K: Into<String>>(template: K) -> StringTemplate {
        StringTemplateImpl::tera(template.into()).into()
    }

    /// Creates a prompt template from a file. The file should be a text file containing the template as a tera template.
    /// # Examples
    /// ```no_run
    /// use llm_chain::prompt::StringTemplate;
    /// let template = StringTemplate::from_file("template.txt").unwrap();
    /// ```
    pub fn from_file<K: AsRef<std::path::Path>>(path: K) -> Result<StringTemplate, std::io::Error> {
        io::read_prompt_template_file(path)
    }

    /// Combines two prompt templates into one.
    /// This is useful for creating a prompt template from multiple sources.
    /// # Examples
    /// ```
    /// use llm_chain::prompt::StringTemplate;
    /// use llm_chain::Parameters;
    /// let template1 = StringTemplate::tera("Hello {{name}}");
    /// let template2 = StringTemplate::tera("!");
    /// let template3 = StringTemplate::combine(vec![template1, template2]);
    /// let parameters: Parameters = vec![("name", "World")].into();
    /// assert_eq!(template3.format(&parameters).unwrap(), "Hello World!");
    /// ```
    pub fn combine(parts: Vec<StringTemplate>) -> StringTemplate {
        let res: Vec<StringTemplateImpl> = parts.into_iter().map(|p| p.0).collect();
        StringTemplateImpl::combine(res).into()
    }
}

impl fmt::Display for StringTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The actual implementation of the prompt template. This hides the implementation details from the user.
#[derive(Clone, Debug, Serialize, Deserialize)]
enum StringTemplateImpl {
    Static(String),
    Tera(String),
    Combined(Vec<StringTemplateImpl>),
}

impl StringTemplateImpl {
    pub fn format(&self, parameters: &Parameters) -> Result<String, StringTemplateErrorImpl> {
        match self {
            Self::Static(template) => Ok(template.clone()),
            Self::Tera(template) => tera::render(template, parameters).map_err(|e| e.into()),
            Self::Combined(templates) => {
                let mut result = String::new();
                for template in templates {
                    let formatted = template.format(parameters)?;
                    result.push_str(&formatted);
                }
                Ok(result)
            }
        }
    }

    pub fn static_string(template: String) -> Self {
        Self::Static(template)
    }

    pub fn tera(template: String) -> Self {
        Self::Tera(template)
    }

    pub fn combine(templates: Vec<Self>) -> Self {
        Self::Combined(templates)
    }
}

impl fmt::Display for StringTemplateImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static(s) => write!(f, "{}", s),
            Self::Tera(template) => write!(f, "{}", template),
            Self::Combined(templates) => {
                for template in templates {
                    write!(f, "{}", template)?;
                }
                Ok(())
            }
        }
    }
}

impl From<&str> for StringTemplate {
    fn from(template: &str) -> Self {
        Self::tera(template.to_string())
    }
}
