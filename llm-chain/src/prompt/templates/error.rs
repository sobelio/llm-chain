use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum PromptTemplateErrorImpl {
    #[cfg(feature = "tera")]
    #[error("Tera error: {0}")]
    Tera(String),
    #[error("Unable to load file: {0}")]
    UnableToLoadFile(String),
    #[error("Unable to parse template: {0}")]
    LegacyTemplateError(String),
}

impl From<std::io::Error> for PromptTemplateErrorImpl {
    fn from(error: std::io::Error) -> Self {
        PromptTemplateErrorImpl::UnableToLoadFile(error.to_string())
    }
}

#[cfg(feature = "tera")]
impl From<tera::Error> for PromptTemplateErrorImpl {
    fn from(error: tera::Error) -> Self {
        PromptTemplateErrorImpl::Tera(error.to_string())
    }
}

#[derive(Error, Debug, Clone)]
#[error(transparent)]
/// An error that can occur when formatting a prompt template.
/// This is a wrapper around the underlying error type, as
/// the underlying error type doesn't have a stable API.
pub struct PromptTemplateError(#[from] PromptTemplateErrorImpl);
