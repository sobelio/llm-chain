use super::options::PerInvocation;
use super::output::Output;
use super::prompt::create_chat_completion_request;
use super::Model;
use async_openai::error::OpenAIError;
use llm_chain::step::{Step, StepError};
use llm_chain::tokens::PromptTokensError;
use llm_chain::traits::ExecutorError;
use llm_chain::Parameters;
use llm_chain::{traits, PromptTemplateError};

use super::options::PerExecutor;
use async_trait::async_trait;
use llm_chain::tokens::TokenCount;

use tiktoken_rs::async_openai::num_tokens_from_messages;

use std::sync::Arc;

/// The executor for the ChatGPT model. This executor uses the `async_openai` crate to communicate with the OpenAI API.
#[derive(Clone, Default)]
pub struct Executor {
    client: Arc<async_openai::Client>,
}

impl Executor {
    /// Creates a new executor with the given client.
    pub fn new(client: async_openai::Client) -> Self {
        let client = Arc::new(client);
        Self { client }
    }
    /// Creates a new executor with the default client, which uses the `OPENAI_API_KEY` environment variable.
    pub fn new_default() -> Self {
        let client = async_openai::Client::new();
        Self::new(client)
    }

    fn get_bpe_from_model(
        &self,
        step: &Step<Self>,
    ) -> Result<tiktoken_rs::CoreBPE, PromptTokensError> {
        use tiktoken_rs::get_bpe_from_model;
        get_bpe_from_model(&self.get_model_from_step(step).to_string())
            .map_err(|_| PromptTokensError::NotAvailable)
    }

    fn get_model_from_step(&self, step: &Step<Self>) -> Model {
        step.options()
            .and_then(|opts| opts.model.clone())
            .unwrap_or_default()
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    OpenAIError(#[from] OpenAIError),
    StepError(#[from] StepError),
    PromptTemplateError(#[from] PromptTemplateError),
}

impl ExecutorError for Error {}

#[async_trait]
impl traits::Executor for Executor {
    type PerInvocationOptions = PerInvocation;
    type PerExecutorOptions = PerExecutor;

    type Output = Output;
    type Token = usize;
    type Error = Error;
    async fn execute(
        &self,
        step: &Step<Self>,
        parameters: &Parameters,
    ) -> Result<Self::Output, Self::Error> {
        let client = self.client.clone();
        let model = self.get_model_from_step(step);
        let input = create_chat_completion_request(&model, step.prompt(), parameters)?;
        let res = async move { client.chat().create(input).await }.await?;
        Ok(res.into())
    }
    fn tokens_used(
        &self,
        step: &Step<Self>,
        parameters: &Parameters,
    ) -> Result<TokenCount, PromptTokensError> {
        let model = self.get_model_from_step(step);
        let model_s = model.to_string();

        let max_tokens = tiktoken_rs::model::get_context_size(&model_s);
        let prompt = step.prompt();
        let completion_req = create_chat_completion_request(&model, prompt, parameters)?;
        // This approach will break once we add support for non-string valued parameters.
        let prompt_with_empty_params =
            create_chat_completion_request(&model, prompt, &parameters.with_placeholder_values())?;

        let num_tokens_with_empty_params =
            num_tokens_from_messages(&model_s, &prompt_with_empty_params.messages)
                .map_err(|_| PromptTokensError::NotAvailable)?;
        let tokens_used = num_tokens_from_messages(&model.to_string(), &completion_req.messages)
            .map_err(|_| PromptTokensError::NotAvailable)?;

        Ok(TokenCount::new(
            max_tokens as i32,
            tokens_used as i32,
            num_tokens_with_empty_params as i32,
        ))
    }
    fn tokenize_str(&self, step: &Step<Self>, doc: &str) -> Result<Vec<usize>, PromptTokensError> {
        Ok(self.get_bpe_from_model(step)?.encode_ordinary(doc))
    }
    fn to_string(&self, step: &Step<Self>, tokens: &[usize]) -> Result<String, PromptTokensError> {
        let res = self
            .get_bpe_from_model(step)?
            .decode(tokens.to_vec())
            .map_err(|_| PromptTokensError::UnableToCompute)?;
        Ok(res)
    }
}
