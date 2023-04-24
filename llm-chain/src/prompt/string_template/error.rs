use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum StringTemplateErrorImpl {
    #[error("Tera error: {0}")]
    Tera(String),
    #[error("Unable to load file: {0}")]
    UnableToLoadFile(String),
    #[error("Unable to parse template: {0}")]
    LegacyTemplateError(String),
}

impl From<std::io::Error> for StringTemplateErrorImpl {
    fn from(error: std::io::Error) -> Self {
        StringTemplateErrorImpl::UnableToLoadFile(error.to_string())
    }
}

impl From<tera::Error> for StringTemplateErrorImpl {
    fn from(error: tera::Error) -> Self {
        StringTemplateErrorImpl::Tera(error.to_string())
    }
}

#[derive(Error, Debug, Clone)]
#[error(transparent)]
/// An error that can occur when formatting a prompt template.
/// This is a wrapper around the underlying error type, as
/// the underlying error type doesn't have a stable API.
pub struct StringTemplateError(#[from] StringTemplateErrorImpl);
