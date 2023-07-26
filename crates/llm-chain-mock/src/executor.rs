use async_trait::async_trait;
use llm_chain::options;
use llm_chain::options::{options_from_env, Opt, OptDiscriminants, Options, OptionsCascade};
use llm_chain::output::Output;
use llm_chain::prompt::Prompt;
use llm_chain::tokens::{
    PromptTokensError, TokenCollection, TokenCount, Tokenizer, TokenizerError,
};
use llm_chain::traits::{ExecutorCreationError, ExecutorError};
use thiserror::Error;

/// Executor is responsible for running the LLM and managing its context.
pub struct Executor {
    options: Options,
}

// TODO: cleanup
#[derive(Debug, Error)]
pub enum Error {
    #[error("an invalid token was encountered during tokenization")]
    /// During tokenization, one of the produced tokens was invalid / zero.
    TokenizationFailed,
    #[error("the context window is full")]
    /// The context window for the model is full.
    ContextFull,
    #[error("reached end of text")]
    /// The model has produced an end of text token, signalling that it thinks that the text should end here.
    ///
    /// Note that this error *can* be ignored and inference can continue, but the results are not guaranteed to be sensical.
    EndOfText,
}

#[async_trait]
impl llm_chain::traits::Executor for Executor {
    type StepTokenizer<'a> = MockTokenizer;
    
    fn new_with_options(options: Options) -> Result<Self, ExecutorCreationError> {
        Ok(Executor { options })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        // TODO: print options1
        let mut output = format!( "As a mock large language model, I'm here to help you debug. I have received your prompt: \"{prompt}\".");
        Ok(Output::new_immediate(Prompt::text(output)))
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
        None
    }

    fn get_tokenizer(&self, _: &Options) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
        unimplemented!();
    }
}

pub struct MockTokenizer {}

impl MockTokenizer {
    pub fn new(executor: Executor) -> Self {
        MockTokenizer {}
    }
}

impl Tokenizer for MockTokenizer {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        unimplemented!()
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        unimplemented!()
    }
}