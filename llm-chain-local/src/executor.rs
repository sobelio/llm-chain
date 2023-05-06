use std::env::var;
use std::path::Path;

use crate::options::{PerExecutor, PerInvocation};
use crate::output::Output;
use crate::LocalLlmTextSplitter;

use async_trait::async_trait;
use llm::models::{Bloom, Gpt2, GptJ, Llama, NeoX};
use llm::{
    load_progress_callback_stdout, KnownModel, LoadError, Model, ModelParameters, TokenId,
    TokenUtf8Buffer,
};
use llm_chain::prompt::Prompt;
use llm_chain::tokens::{PromptTokensError, TokenCount, Tokenizer, TokenizerError};
use llm_chain::traits::{ExecutorCreationError, ExecutorError};
use thiserror::Error;

/// Executor is responsible for running the LLM and managing its context.
pub struct Executor {
    llm: Box<dyn Model>,
    callback: Option<fn(&Output)>,
}

impl Executor {
    pub fn with_callback(mut self, callback: fn(&Output)) -> Self {
        self.callback = Some(callback);
        self
    }

    pub(crate) fn get_llm(&self) -> &dyn llm::Model {
        self.llm.as_ref()
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("unable to tokenize prompt")]
    PromptTokensError(PromptTokensError),
    #[error("unable to create executor: {0}")]
    InnerError(#[from] Box<dyn std::error::Error>),
}

impl ExecutorError for Error {}

#[async_trait]
impl llm_chain::traits::Executor for Executor {
    type PerInvocationOptions = PerInvocation;
    type PerExecutorOptions = PerExecutor;
    type Output = Output;
    type Error = Error;
    type Token = i32;
    type StepTokenizer<'a> = LocalLlmTokenizer<'a>;
    type TextSplitter<'a> = LocalLlmTextSplitter<'a>;

    fn new_with_options(
        options: Option<Self::PerExecutorOptions>,
        invocation_options: Option<Self::PerInvocationOptions>,
    ) -> Result<Self, ExecutorCreationError> {
        let model_type = options
            .as_ref()
            .and_then(|x| x.model_type.clone())
            .or_else(|| var("LLM_MODEL_TYPE").ok())
            .ok_or(ExecutorCreationError::FieldRequiredError(
                "model_type, ensure to provide the parameter or set `LLM_MODEL_TYPE` environment variable ".to_string(),
            ))?;
        let model_path = options
                .as_ref()
                .and_then(|x| x.model_path.clone())
                .or_else(|| var("LLM_MODEL_PATH").ok())
                .ok_or(ExecutorCreationError::FieldRequiredError(
                    "model_path, ensure to provide the parameter or set `LLM_MODEL_PATH` environment variable ".to_string(),
                ))?;

        let params = invocation_options.unwrap_or_default().into();
        let llm: Box<dyn Model> = match model_type.as_str() {
            "bloom" => load::<Bloom>(&model_path, params),
            "gpt2" => load::<Gpt2>(&model_path, params),
            "gptj" => load::<GptJ>(&model_path, params),
            "llama" => load::<Llama>(&model_path, params),
            "neox" => load::<NeoX>(&model_path, params),
            m => Err(LoadError::InvariantBroken {
                path: None,
                invariant: format!("Unsupported model type {m}"),
            }),
        }
        .map_err(|e| ExecutorCreationError::InnerError(Box::new(e)))?;

        Ok(Executor {
            llm,
            callback: None,
        })
    }

    async fn execute(
        &self,
        _: Option<&Self::PerInvocationOptions>,
        prompt: &Prompt,
    ) -> Result<Self::Output, Self::Error> {
        let session = &mut self.llm.start_session(Default::default());
        let mut output = String::new();
        session
            .infer::<std::convert::Infallible>(
                self.llm.as_ref(),
                prompt.to_text().as_str(),
                // EvaluateOutputRequest
                &mut Default::default(),
                &mut rand::thread_rng(),
                |t| {
                    output.push_str(t);

                    Ok(())
                },
            )
            .map_err(|e| Error::InnerError(Box::new(e)))?;

        Ok(output.into())
    }

    fn tokens_used(
        &self,
        options: Option<&Self::PerInvocationOptions>,
        prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        let tokenizer = self.get_tokenizer(options)?;
        let input = prompt.to_text();

        let tokens_used = tokenizer
            .tokenize_str(&input)
            .map_err(|_e| PromptTokensError::UnableToCompute)?
            .len() as i32;
        let max_tokens = self.max_tokens_allowed(options);
        Ok(TokenCount::new(max_tokens, tokens_used))
    }

    fn max_tokens_allowed(&self, _: Option<&Self::PerInvocationOptions>) -> i32 {
        self.llm.n_context_tokens().try_into().unwrap_or(2048)
    }

    fn get_tokenizer(
        &self,
        _: Option<&Self::PerInvocationOptions>,
    ) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
        Ok(LocalLlmTokenizer::new(self))
    }

    fn get_text_splitter(
        &self,
        _: Option<&Self::PerInvocationOptions>,
    ) -> Result<Self::TextSplitter<'_>, Self::Error> {
        Ok(LocalLlmTextSplitter::new(self))
    }
}

pub struct LocalLlmTokenizer<'a> {
    llm: &'a dyn Model,
}

impl<'a> LocalLlmTokenizer<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        LocalLlmTokenizer {
            llm: executor.llm.as_ref(),
        }
    }
}

impl Tokenizer<llm::TokenId> for LocalLlmTokenizer<'_> {
    fn tokenize_str(&self, doc: &str) -> Result<Vec<llm::TokenId>, TokenizerError> {
        match &self.llm.vocabulary().tokenize(doc, false) {
            Ok(tokens) => Ok(tokens.into_iter().map(|t| t.1).collect()),
            Err(_) => Err(TokenizerError::TokenizationError),
        }
    }

    fn to_string(&self, tokens: Vec<TokenId>) -> Result<String, TokenizerError> {
        let mut res = String::new();
        let mut token_utf8_buf = TokenUtf8Buffer::new();
        for token_id in tokens {
            // Buffer the token until it's valid UTF-8, then call the callback.
            if let Some(tokens) =
                token_utf8_buf.push(self.llm.vocabulary().token(token_id as usize))
            {
                res.push_str(&tokens)
            }
        }

        Ok(res)
    }
}

pub fn load<M: KnownModel + 'static>(
    model_path: &str,
    params: ModelParameters,
) -> Result<Box<dyn Model>, LoadError> {
    let model = llm::load::<M>(Path::new(model_path), params, load_progress_callback_stdout)?;

    Ok(Box::new(model))
}
