use super::output::Output;
use super::step::Step;
use async_openai::error::OpenAIError;
use llm_chain::tokens::PromptTokensError;
use llm_chain::traits;
use llm_chain::Parameters;

use async_trait::async_trait;
use llm_chain::tokens::TokenCount;
use llm_chain::traits::StepError;
use tiktoken_rs::async_openai::num_tokens_from_messages;

use std::sync::Arc;

/// The executor for the ChatGPT model. This executor uses the `async_openai` crate to communicate with the OpenAI API.
#[derive(Clone)]
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

    fn get_bpe_from_model(&self, step: &Step) -> Result<tiktoken_rs::CoreBPE, PromptTokensError> {
        use tiktoken_rs::get_bpe_from_model;
        get_bpe_from_model(&step.model.to_string()).map_err(|_| PromptTokensError::NotAvailable)
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error<E: StepError> {
    OpenAIError(OpenAIError),
    StepError(#[from] E),
}

impl<E: StepError> traits::ExecutorError for Error<E> {}

#[async_trait]
impl traits::Executor for Executor {
    type Step = Step;
    type Output = Output;
    type Token = usize;
    type Error = Error<<Step as traits::Step>::Error>;
    async fn execute(
        &self,
        input: <<Executor as traits::Executor>::Step as traits::Step>::Output,
    ) -> Result<Self::Output, Self::Error> {
        let client = self.client.clone();
        let res = async move { client.chat().create(input).await }.await;
        res.map(|x| x.into()).map_err(Error::OpenAIError)
    }
    fn tokens_used(
        &self,
        step: &Step,
        parameters: &Parameters,
    ) -> Result<TokenCount, PromptTokensError> {
        let max_tokens = tiktoken_rs::model::get_context_size(&step.model.to_string());
        let prompt = step.prompt.format(parameters)?;
        // This approach will break once we add support for non-string valued parameters.
        let prompt_with_empty_params = step.prompt.format(&parameters.with_placeholder_values())?;
        let num_tokens_with_empty_params =
            num_tokens_from_messages(&step.model.to_string(), &prompt_with_empty_params)
                .map_err(|_| PromptTokensError::NotAvailable)?;
        let tokens_used = num_tokens_from_messages(&step.model.to_string(), &prompt)
            .map_err(|_| PromptTokensError::NotAvailable)?;

        Ok(TokenCount::new(
            max_tokens as i32,
            tokens_used as i32,
            num_tokens_with_empty_params as i32,
        ))
    }
    fn tokenize_str(&self, step: &Step, doc: &str) -> Result<Vec<usize>, PromptTokensError> {
        Ok(self.get_bpe_from_model(step)?.encode_ordinary(doc))
    }
    fn to_string(&self, step: &Step, tokens: &[usize]) -> Result<String, PromptTokensError> {
        let res = self
            .get_bpe_from_model(step)?
            .decode(tokens.to_vec())
            .map_err(|_| PromptTokensError::UnableToCompute)?;
        Ok(res)
    }
}
