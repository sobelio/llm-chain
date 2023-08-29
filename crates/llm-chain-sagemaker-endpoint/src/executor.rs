use crate::model::Formatter;
use crate::model::Model;
use async_trait::async_trait;

use llm_chain::options;
use llm_chain::options::Opt;
use llm_chain::options::Options;
use llm_chain::options::OptionsCascade;
use llm_chain::output::Output;
use llm_chain::prompt::Prompt;
use llm_chain::tokens::{
    PromptTokensError, TokenCollection, TokenCount, Tokenizer, TokenizerError,
};
use llm_chain::traits::{ExecutorCreationError, ExecutorError};

use tokenizers::tokenizer::Tokenizer as HuggingFaceTokenizer;

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
            options,
            sagemaker_client: client,
        })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let opts = self.cascade(Some(options));
        let model = self.get_model_from_invocation_options(&opts);

        let body_blob = model.format_request(prompt, &opts);

        let result = self
            .sagemaker_client
            .invoke_endpoint()
            .endpoint_name(model.to_jumpstart_endpoint_name())
            .content_type(model.request_content_type())
            .body(body_blob)
            .send()
            .await;
        let response = result.map_err(|e| ExecutorError::InnerError(e.into()))?;
        let generated_text = model.parse_response(response);

        Ok(Output::new_immediate(Prompt::text(generated_text)))
    }

    fn tokens_used(
        &self,
        _options: &Options,
        _prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        // Not all models expose this information.
        unimplemented!();
    }

    fn max_tokens_allowed(&self, _: &Options) -> i32 {
        // Not all models expose this information.
        unimplemented!();
    }

    fn answer_prefix(&self, _prompt: &Prompt) -> Option<String> {
        // Not all models expose this information.
        unimplemented!();
    }

    fn get_tokenizer(&self, options: &Options) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
        Ok(SageMakerEndpointTokenizer::new(self.cascade(Some(options))))
    }
}

pub struct SageMakerEndpointTokenizer {
    tokenizer: Option<HuggingFaceTokenizer>
}

impl SageMakerEndpointTokenizer {
    pub fn new(options: OptionsCascade) -> Self {
        let optional_tokenizer = match options.get(llm_chain::options::OptDiscriminants::Model) {
            Some(Opt::Model(model)) => {
               let model_struct = Model::from_str(&model.to_name()).unwrap(); 
                Some(HuggingFaceTokenizer::from_pretrained(&model_struct.to_huggingface_name(), None).unwrap()) // TODO: no options
            }
            _ => None,
        };
        
        SageMakerEndpointTokenizer {
            tokenizer: optional_tokenizer
        }
    }
}

impl Tokenizer for SageMakerEndpointTokenizer {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        match &self.tokenizer {
            Some(tokenizer) => {
                let encoding = tokenizer.encode(doc, false).map_err(|_| TokenizerError::TokenizationError)?;
                let ids: Vec<i32> = encoding.get_ids().iter().map(|x| *x as i32).collect();
                Ok(TokenCollection::from(ids))
            },
            None => unimplemented!("This model does not have a tokenizer impelmentation.")
        }
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        match &self.tokenizer {
            Some(tokenizer) => {
                let ids: Vec<u32> = tokens.as_i32().unwrap().iter().map(|x| *x as u32).collect::<Vec<u32>>();
                Ok(tokenizer.decode(ids.as_slice(), false).map_err(|_| TokenizerError::TokenizationError)?)
            },
            None => unimplemented!("This model does not have a tokenizer impelmentation.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Model;
    use llm_chain::traits::Executor;
    
    #[test]
    fn test_tokenizer() {
        let opts = options!(
            Model: Model::Falcon7BInstruct
        );
        let executor: super::Executor = Executor::new_with_options(opts.clone()).unwrap();
        let opts_cascade = executor.cascade(Some(&opts));
        let tokenizer = SageMakerEndpointTokenizer::new(opts_cascade);
        let doc = "This is a example string to be tokenized";
        let tokens = vec![1182, 304, 241, 1945, 3821, 271, 314, 10930, 1190];
        
        assert_eq!(tokenizer.tokenize_str(doc).unwrap().len(), 9);
        assert_eq!(tokenizer.tokenize_str(doc).unwrap().as_i32().unwrap(), tokens);
        
        assert_eq!(tokenizer.to_string(TokenCollection::from(tokens)).unwrap(), doc);
    }
}