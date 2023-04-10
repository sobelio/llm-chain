use super::step::Step;
use async_openai::types::CreateChatCompletionResponse;
use llm_chain::traits;
use llm_chain::Parameters;

use async_trait::async_trait;

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
}

#[async_trait]
impl traits::Executor for Executor {
    type Step = Step;
    type Output = CreateChatCompletionResponse;
    async fn execute(
        &self,
        input: <<Executor as traits::Executor>::Step as traits::Step>::Output,
    ) -> Self::Output {
        let client = self.client.clone();
        async move { client.chat().create(input).await.unwrap() }.await
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
}

impl traits::ExecutorPromptTokens<Step> for Executor {
    fn count_prompt_tokens(&self, step: &Step) -> Result<usize, traits::PromptTokensError> {
        use tiktoken_rs::async_openai::get_chat_completion_max_tokens;
        let placeholder_params = Parameters::new_with_text("".to_string());
        get_chat_completion_max_tokens(
            &step.model.to_string(),
            &step.prompt.format(&placeholder_params).as_slice(),
        )
        .map_err(|_| traits::PromptTokensError::NotAvailable)
    }
    fn count_tokens_for_doc(
        &self,
        step: &Step,
        doc: &str,
    ) -> Result<usize, traits::PromptTokensError> {
        use tiktoken_rs::get_bpe_from_model;
        let tok = get_bpe_from_model(&step.model.to_string())
            .map_err(|_| traits::PromptTokensError::NotAvailable)?;
        Ok(tok.encode_ordinary(doc).len())
    }
    fn split_at_tokens(
        &self,
        step: &Step,
        doc: &str,
        tokens: usize,
    ) -> Result<(String, String), traits::PromptTokensError> {
        use tiktoken_rs::get_bpe_from_model;

        let tok = get_bpe_from_model(&step.model.to_string())
            .map_err(|_| traits::PromptTokensError::NotAvailable)?;
        let v = tok.encode_ordinary(doc);
        if v.len() <= tokens {
            return Ok((doc.to_string(), "".to_string()));
        } else {
            Ok((
                tok.decode(v[..tokens].to_vec())
                    .map_err(|_| traits::PromptTokensError::NotAvailable)?,
                tok.decode(v[tokens..].to_vec())
                    .map_err(|_| traits::PromptTokensError::NotAvailable)?,
            ))
        }
    }
    fn max_tokens(&self, step: &Step) -> Result<usize, traits::PromptTokensError> {
        let context_size = tiktoken_rs::model::get_context_size(&step.model.to_string());
        Ok(context_size)
    }
    fn count_tokens_for_output(
        &self,
        step: &Step,
        output: &Self::Output,
    ) -> Result<usize, traits::PromptTokensError> {
        use tiktoken_rs::get_bpe_from_model;
        let tok = get_bpe_from_model(&step.model.to_string())
            .map_err(|_| traits::PromptTokensError::NotAvailable)?;
        Ok(tok
            .encode_ordinary(&output.choices.first().unwrap().message.content)
            .len())
    }
}
