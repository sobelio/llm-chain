use super::options::PerInvocation;
use super::output::Output;
use super::prompt::create_chat_completion_request;
use super::prompt::format_chat_messages;
use super::Model;
use super::OpenAITextSplitter;
use async_openai::error::OpenAIError;
use llm_chain::prompt::Prompt;

use llm_chain::tokens::PromptTokensError;
use llm_chain::tokens::{Tokenizer, TokenizerError};
use llm_chain::traits;
use llm_chain::traits::{ExecutorCreationError, ExecutorError};

use super::options::PerExecutor;
use async_trait::async_trait;
use llm_chain::tokens::TokenCount;

use tiktoken_rs::async_openai::num_tokens_from_messages;

use std::sync::Arc;

/// The `Executor` struct for the ChatGPT model. This executor uses the `async_openai` crate to communicate with the OpenAI API.
#[derive(Clone, Default)]
pub struct Executor {
    /// The client used to communicate with the OpenAI API.
    client: Arc<async_openai::Client>,
    /// The per-invocation options for this executor.
    per_invocation_options: Option<PerInvocation>,
}

impl Executor {
    /// Creates a new `Executor` with the given client.
    pub fn for_client(
        client: async_openai::Client,
        per_invocation_options: Option<PerInvocation>,
    ) -> Self {
        let client = Arc::new(client);
        Self {
            client,
            per_invocation_options,
        }
    }

    fn get_model_from_invocation_options(&self, opts: Option<&PerInvocation>) -> Model {
        opts.or(self.per_invocation_options.as_ref())
            .and_then(|opts| opts.model.clone())
            .unwrap_or_default()
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    OpenAIError(#[from] OpenAIError),
}
impl ExecutorError for Error {}

#[async_trait]
impl traits::Executor for Executor {
    type PerInvocationOptions = PerInvocation;
    type PerExecutorOptions = PerExecutor;

    type Output = Output;
    type Token = usize;
    type StepTokenizer<'a> = OpenAITokenizer;
    type TextSplitter<'a> = OpenAITextSplitter;
    type Error = Error;

    fn new_with_options(
        executor_options: Option<Self::PerExecutorOptions>,
        invocation_options: Option<Self::PerInvocationOptions>,
    ) -> Result<Self, ExecutorCreationError> {
        let mut client = async_openai::Client::new();
        if let Some(executor_options) = executor_options {
            if let Some(api_key) = executor_options.api_key {
                client = client.with_api_key(api_key)
            }
        }
        let client = Arc::new(client);
        Ok(Self {
            client,
            per_invocation_options: invocation_options,
        })
    }

    async fn execute(
        &self,
        opts: Option<&PerInvocation>,
        prompt: &Prompt,
    ) -> Result<Self::Output, Self::Error> {
        let client = self.client.clone();
        let model = self.get_model_from_invocation_options(opts);
        let input = create_chat_completion_request(&model, prompt, None).unwrap();
        let res = async move { client.chat().create(input).await }.await?;
        Ok(res.into())
    }

    fn tokens_used(
        &self,
        opts: Option<&PerInvocation>,
        prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        let model = self.get_model_from_invocation_options(opts);
        let messages = format_chat_messages(prompt.to_chat())?;
        let tokens_used = num_tokens_from_messages(&model.to_string(), &messages)
            .map_err(|_| PromptTokensError::NotAvailable)?;

        Ok(TokenCount::new(
            self.max_tokens_allowed(opts),
            tokens_used as i32,
        ))
    }

    /// Get the context size from the model or return default context size
    fn max_tokens_allowed(&self, opts: Option<&PerInvocation>) -> i32 {
        let model = self.get_model_from_invocation_options(opts);
        tiktoken_rs::model::get_context_size(&model.to_string())
            .try_into()
            .unwrap_or(4096)
    }

    fn get_tokenizer(
        &self,
        options: Option<&PerInvocation>,
    ) -> Result<OpenAITokenizer, TokenizerError> {
        let opts = options
            .or(self.per_invocation_options.as_ref())
            .cloned()
            .unwrap_or_default();
        Ok(OpenAITokenizer::new(&opts))
    }

    fn get_text_splitter(
        &self,
        options: Option<&PerInvocation>,
    ) -> Result<Self::TextSplitter<'_>, Self::Error> {
        Ok(OpenAITextSplitter::new(
            self.get_model_from_invocation_options(options),
        ))
    }
}

pub struct OpenAITokenizer {
    model_name: String,
}

impl OpenAITokenizer {
    pub fn new(options: &PerInvocation) -> Self {
        Self {
            model_name: options.model.clone().unwrap_or_default().to_string(),
        }
    }

    fn get_bpe_from_model(&self) -> Result<tiktoken_rs::CoreBPE, PromptTokensError> {
        use tiktoken_rs::get_bpe_from_model;
        get_bpe_from_model(&self.model_name).map_err(|_| PromptTokensError::NotAvailable)
    }
}

impl Tokenizer<usize> for OpenAITokenizer {
    fn tokenize_str(&self, doc: &str) -> Result<Vec<usize>, TokenizerError> {
        Ok(self
            .get_bpe_from_model()
            .map_err(|_| TokenizerError::TokenizationError)?
            .encode_ordinary(doc))
    }

    fn to_string(&self, tokens: Vec<usize>) -> Result<String, TokenizerError> {
        let res = self
            .get_bpe_from_model()
            .map_err(|_e| TokenizerError::ToStringError)?
            .decode(tokens.to_vec())
            .map_err(|_e| TokenizerError::ToStringError)?;
        Ok(res)
    }
}
