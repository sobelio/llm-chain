use async_trait::async_trait;
use llm_chain::options::Options;
use llm_chain::output::Output;
use llm_chain::prompt::Prompt;
use llm_chain::tokens::{
    PromptTokensError, TokenCollection, TokenCount, Tokenizer, TokenizerError,
};
use llm_chain::traits::{ExecutorCreationError, ExecutorError};
use aws_config;
use aws_sdk_sagemakerruntime;
use aws_sdk_sagemakerruntime::primitives::Blob;
use serde_json;

/// Executor is responsible for running the LLM and managing its context.
pub struct Executor {
    #[allow(dead_code)]
    options: Options,
}

#[async_trait]
impl llm_chain::traits::Executor for Executor {
    type StepTokenizer<'a> = SageMakerEndpointTokenizer;
    
    fn new_with_options(options: Options) -> Result<Self, ExecutorCreationError> {
        Ok(Executor { options: options })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_sagemakerruntime::Client::new(&config);
        
        let body_string = format!("{{\"inputs\": \"{}\"}}", prompt);
        let body_blob = Blob::new(body_string.as_bytes().to_vec());
        
        
        let result = client.invoke_endpoint()
            .endpoint_name("falcon-7b") // TODO: make this an option
            .content_type("application/json")
            .body(body_blob.into())
            .send()
            .await;
        // TODO: error handling if the response is not valid
        let output = String::from_utf8(result.unwrap().body.unwrap().into_inner()).unwrap();
        let output_json: serde_json::Value = serde_json::from_str(&output).unwrap();
        let generated_text = output_json[0]["generated_text"].to_string();
        
        Ok(Output::new_immediate(Prompt::text(generated_text)))
    }

    fn tokens_used(
        &self,
        options: &Options,
        prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        unimplemented!();
    }

    fn max_tokens_allowed(&self, _: &Options) -> i32 {
        unimplemented!();
    }

    fn answer_prefix(&self, _prompt: &Prompt) -> Option<String> {
        unimplemented!();
    }

    fn get_tokenizer(&self, _: &Options) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
        unimplemented!();
    }
}

pub struct SageMakerEndpointTokenizer {}

impl SageMakerEndpointTokenizer {
    pub fn new(_executor: &Executor) -> Self {
        SageMakerEndpointTokenizer {}
    }
}

impl Tokenizer for SageMakerEndpointTokenizer {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        unimplemented!();
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        unimplemented!();
    }
}