use super::step::Step;
use async_openai::types::CreateChatCompletionResponse;
use llm_chain::tokens::PromptTokensError;
use llm_chain::traits;
use llm_chain::Parameters;

use async_trait::async_trait;
use llm_chain::tokens::TokenCount;
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

#[async_trait]
impl traits::Executor for Executor {
    type Step = Step;
    type Output = CreateChatCompletionResponse;
    type Token = usize;
    async fn execute(
        &self,
        input: <<Executor as traits::Executor>::Step as traits::Step>::Output,
    ) -> Self::Output {
        let client = self.client.clone();
        let toks =
            tiktoken_rs::async_openai::num_tokens_from_messages(&input.model, &input.messages)
                .unwrap();
        println!("toks: {}", toks);

        let res = async move { client.chat().create(input).await.unwrap() }.await;
        res
    }
    fn apply_output_to_parameters(parameters: Parameters, output: &Self::Output) -> Parameters {
        let text = output.choices.first().unwrap().message.content.to_string();
        parameters.with_text(text)
    }
    fn combine_outputs(output: &Self::Output, other: &Self::Output) -> Self::Output {
        let mut combined = output.clone();
        combined.choices.first_mut().unwrap().message.content = [
            output.choices.first().unwrap().message.content.clone(),
            other.choices.first().unwrap().message.content.clone(),
        ]
        .join("\n");
        combined
    }
    fn tokens_used(
        &self,
        step: &Step,
        parameters: &Parameters,
    ) -> Result<TokenCount, PromptTokensError> {
        let max_tokens = tiktoken_rs::model::get_context_size(&step.model.to_string());
        let prompt = step.prompt.format(parameters);

        let prompt_with_empty_params = step.prompt.format(&Parameters::new_non_strict());
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
