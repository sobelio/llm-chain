use super::step::Step;
use async_openai::types::CreateChatCompletionResponse;
use llm_chain::traits;
use llm_chain::Parameters;

use async_trait::async_trait;

use std::sync::Arc;

/// The executor for the ChatGPT model. This executor uses the `async_openai` crate to communicate with the OpenAI API.
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
        async move {
            let create = client.chat().create(input).await.unwrap();
            create
        }
        .await
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
