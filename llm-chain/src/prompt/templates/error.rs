use thiserror::Error;

#[derive(Error, Debug)]
pub enum PromptTemplateErrorImpl {
    #[cfg(feature = "tera")]
    #[error("Tera error: {0}")]
    Tera(#[from] tera::Error),
    #[error("Unable to load file: {0}")]
    UnableToLoadFile(#[from] std::io::Error),
    #[error("Unable to parse template: {0}")]
    LegacyTemplateError(String),
}

#[derive(Error, Debug)]
#[error(transparent)]
/// An error that can occur when formatting a prompt template.
/// This is a wrapper around the underlying error type, as
/// the underlying error type doesn't have a stable API.
pub struct PromptTemplateError(#[from] PromptTemplateErrorImpl);
