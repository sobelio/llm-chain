use async_trait::async_trait;
use llm_chain::options::Options;
use llm_chain::options::OptionsCascade;
use llm_chain::options::Opt;
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
use futures;
use crate::model::Model;
use crate::model::Formatter;
use std::str::FromStr;

/// Executor is responsible for running the LLM and managing its context.
pub struct Executor {
    #[allow(dead_code)]
    options: Options,
    sagemaker_client: aws_sdk_sagemakerruntime::Client,
}

impl Executor {
    fn get_model_from_invocation_options(&self, opts: &OptionsCascade) -> Model {
        let Some(Opt::Model(model)) = opts.get(llm_chain::options::OptDiscriminants::Model) else {
            // TODO: fail gracefully
            panic!("The Model option must not be empty. This option does not have a default.");
        };
        Model::from_str(&model.to_name()).unwrap()
    }
    
    fn cascade<'a>(&'a self, opts: Option<&'a Options>) -> OptionsCascade<'a> {
        let mut v: Vec<&'a Options> = vec![&self.options];
        if let Some(o) = opts {
            v.push(o);
        }
        OptionsCascade::from_vec(v)
    }
}

#[async_trait]
impl llm_chain::traits::Executor for Executor {
    type StepTokenizer<'a> = SageMakerEndpointTokenizer;
    
    fn new_with_options(options: Options) -> Result<Self, ExecutorCreationError> {
        let config = futures::executor::block_on(aws_config::load_from_env());
        let client = aws_sdk_sagemakerruntime::Client::new(&config);
        Ok(Executor { 
            options: options, 
            sagemaker_client: client
        })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let opts = self.cascade(Some(options));
        let model = self.get_model_from_invocation_options(&opts);
        
        let body_blob = model.format_request(prompt);
        
        // TODO: pass model parameters like max tokens
        let result = self.sagemaker_client.invoke_endpoint()
            .endpoint_name(model.to_jumpstart_endpoint_name())
            .content_type(model.request_content_type())
            .body(body_blob.into())
            .send()
            .await;
        let response = result.map_err(|e| ExecutorError::InnerError(e.into()))?; 
        let generated_text = model.parse_response(response);
        
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