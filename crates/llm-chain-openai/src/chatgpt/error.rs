use async_openai::error::OpenAIError;
use llm_chain::prompt::StringTemplateError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum OpenAIInnerError {
    #[error(transparent)]
    OpenAIError(#[from] OpenAIError),
    #[error(transparent)]
    StringTemplateError(#[from] StringTemplateError),
}
