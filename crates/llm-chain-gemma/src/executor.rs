use crate::context::GemmaContext;
use async_trait::async_trait;
use llm_chain::options::{Opt, OptDiscriminants, Options};
use llm_chain::output::Output;
use llm_chain::prompt::Prompt;
use llm_chain::tokens::{
    PromptTokensError, TokenCollection, TokenCount, Tokenizer, TokenizerError,
};
use llm_chain::traits::{Executor as ExecutorTrait, ExecutorCreationError, ExecutorError};
use std::sync::{Arc, Mutex};
use tokio;

pub struct Executor {
    context: Arc<Mutex<GemmaContext>>,
    stream: bool,
}

#[async_trait]
impl ExecutorTrait for Executor {
    type StepTokenizer<'a> = GemmaTokenizer;

    fn new_with_options(options: Options) -> Result<Executor, ExecutorCreationError> {
        let gemma_context = GemmaContext::new(&options)?;
        Ok(Executor {
            context: Arc::new(Mutex::new(gemma_context)),
            stream: if let Some(Opt::Stream(s)) = options.get(OptDiscriminants::Stream) {
                *s
            } else {
                false
            },
        })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let is_stream = if let Some(Opt::Stream(s)) = options.get(OptDiscriminants::Stream) {
            *s
        } else {
            self.stream
        };
        let (sender, stream) = Output::new_stream();
        let context = self.context.clone();
        let prompt_text = prompt.to_string();
        if is_stream {
            tokio::task::spawn_blocking(move || {
                if let Ok(mut ctx) = context.lock() {
                    ctx.generate(prompt_text, sender);
                }
            });
            return Ok(stream);
        } else {
            let mut ctx = context.lock().map_err(|_| ExecutorError::InvalidOptions)?;
            ctx.generate(prompt_text, sender);
        }
        stream
            .to_immediate()
            .await
            .map(|imm| Output::Immediate(imm))
    }

    fn tokens_used(
        &self,
        options: &Options,
        prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        let tokenizer = self.get_tokenizer(options)?;
        let tokens = tokenizer.tokenize_str(prompt.to_string().as_str())?;
        Ok(TokenCount::new(
            self.max_tokens_allowed(options),
            tokens.len() as i32,
        ))
    }

    fn max_tokens_allowed(&self, options: &Options) -> i32 {
        if let Some(Opt::MaxTokens(mt)) = options.get(OptDiscriminants::MaxTokens) {
            return *mt as i32;
        }
        self.context.lock().unwrap().max_generated_tokens() as i32
    }

    fn get_tokenizer(&self, _options: &Options) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
        Ok(GemmaTokenizer {
            context: self.context.clone(),
        })
    }

    fn answer_prefix(&self, _prompt: &Prompt) -> Option<String> {
        None
    }
}

pub struct GemmaTokenizer {
    context: Arc<Mutex<GemmaContext>>,
}

impl Tokenizer for GemmaTokenizer {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        let ctx = self
            .context
            .lock()
            .map_err(|_| TokenizerError::TokenizationError)?;
        ctx.tokenize_str(doc)
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        let ctx = self
            .context
            .lock()
            .map_err(|_| TokenizerError::ToStringError)?;
        ctx.to_string(tokens)
    }
}
