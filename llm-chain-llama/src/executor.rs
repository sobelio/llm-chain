use crate::context::{LLamaContext, LlamaContextParams};
use crate::step::{LlamaInvocation, Step as LLamaStep};
use crate::tokenizer::{embedding_to_output, llama_token_eos, llama_tokenize_helper, tokenize};
use crate::LLamaTextSplitter;

use crate::output::Output;
use async_trait::async_trait;

use llm_chain::tokens::{PromptTokensError, TokenCount, Tokenizer, TokenizerError};
use llm_chain::traits::{self, StepError};
use llm_chain::traits::{Executor as ExecutorTrait, Step as StepTrait};
use llm_chain::Parameters;
use llm_chain_llama_sys::llama_context_params;

/// Executor is responsible for running the LLAMA model and managing its context.
pub struct Executor {
    context: LLamaContext,
    context_params: Option<LlamaContextParams>,
    callback: Option<fn(&Output)>,
}

impl Executor {
    /// Creates a new executor with the given client and optional context parameters.
    pub fn new_with_config(
        model_path: &str,
        context_params: Option<LlamaContextParams>,
        callback: Option<fn(&Output)>,
    ) -> Self {
        let context = LLamaContext::from_file_and_params(model_path, &context_params);
        Self {
            context,
            context_params,
            callback,
        }
    }

    // Creates a new executor with callback for the given model with default context parameters.
    pub fn new_with_callback(model_path: &str, callback: fn(&Output)) -> Self {
        Self::new_with_config(model_path, None, Some(callback))
    }

    /// Creates a new executor for the given model with default context parameters.
    pub fn new(model_path: &str) -> Self {
        Self::new_with_config(model_path, None, None)
    }

    fn context_params(&self) -> llama_context_params {
        LlamaContextParams::or_default(&self.context_params)
    }

    pub(crate) fn get_context(&self) -> &LLamaContext {
        &self.context
    }
}

// Executes the model with the provided input and context parameters.
fn run_model(
    input_ctx: &LLamaContext,
    input: LlamaInvocation,
    context_params_c: llama_context_params,
    callback: &Option<fn(&Output)>,
) -> Output {
    // Tokenize the stop sequence and input prompt.
    let tokenized_stop_prompt = tokenize(
        input_ctx,
        input.stop_sequence.as_str(),
        context_params_c.n_ctx as usize,
        false,
    )
    .unwrap();
    let tokenized_input = tokenize(
        input_ctx,
        input.prompt.as_str(),
        context_params_c.n_ctx as usize,
        true,
    )
    .unwrap();

    // Embd contains the prompt and the completion. The longer the prompt, the shorter the completion.
    let mut embd = tokenized_input.clone();
    embd.resize(context_params_c.n_ctx as usize, 0);

    // Evaluate the prompt in full.
    input_ctx
        .llama_eval(
            tokenized_input.as_slice(),
            tokenized_input.len() as i32,
            0,
            &input,
        )
        .unwrap();
    let token_eos = llama_token_eos();

    // Generate remaining tokens.
    let mut n_remaining = context_params_c.n_ctx - tokenized_input.len() as i32;
    let mut n_used = tokenized_input.len() - 1;
    let mut stop_sequence_i = 0;
    while n_remaining > 0 {
        let tok = input_ctx.llama_sample(embd.as_slice(), n_used as i32, &input);
        n_used += 1;
        n_remaining -= 1;
        embd[n_used] = tok;
        if tok == token_eos {
            break;
        }
        if input.n_tok_predict != 0 && n_used > input.n_tok_predict + tokenized_input.len() - 1 {
            break;
        }
        if tok == tokenized_stop_prompt[stop_sequence_i] {
            stop_sequence_i += 1;
            if stop_sequence_i >= tokenized_stop_prompt.len() {
                break;
            }
        } else {
            stop_sequence_i = 0;
        }
        input_ctx
            .llama_eval(&embd[n_used..], 1, n_used as i32, &input)
            .unwrap();

        if let Some(callback) = callback {
            let output = input_ctx.llama_token_to_str(&embd[n_used]);
            callback(&output.into());
        }
    }
    embedding_to_output(
        input_ctx,
        &embd[tokenized_input.len()..n_used + 1 - stop_sequence_i],
    )
}

impl Executor {
    // Run the LLAMA model with the provided input and generate output.
    fn run_model(&self, input: LlamaInvocation) -> Output {
        run_model(&self.context, input, self.context_params(), &self.callback)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error<E: StepError> {
    #[error("unable to tokenize prompt")]
    PromptTokensError(PromptTokensError),
    #[error("unable to format step")]
    StepError(#[from] E),
}
impl<E> traits::ExecutorError for Error<E> where E: StepError {}

// Implement the ExecutorTrait for the Executor, defining methods for handling input and output.
#[async_trait]
impl ExecutorTrait for Executor {
    type Step = LLamaStep;
    type Output = Output;
    type Token = i32;
    type StepTokenizer<'a> = LLamaTokenizer<'a>;
    type TextSplitter<'a> = LLamaTextSplitter<'a>;
    type Error = Error<<Self::Step as traits::Step>::Error>;
    // Executes the model asynchronously and returns the output.
    async fn execute(
        &self,
        input: <<Executor as ExecutorTrait>::Step as traits::Step>::Output,
    ) -> Result<Self::Output, Self::Error> {
        Ok(self.run_model(input))
    }
    fn tokens_used(
        &self,
        step: &LLamaStep,
        parameters: &Parameters,
    ) -> Result<TokenCount, PromptTokensError> {
        let input = step
            .format(parameters)
            .map_err(|_| PromptTokensError::UnableToCompute)?;

        let tokenizer = self
            .get_tokenizer(step)
            .map_err(|_e| PromptTokensError::UnableToCompute)?;

        let tokens_used = tokenizer
            .tokenize_str(&input.prompt)
            .map_err(|_e| PromptTokensError::UnableToCompute)?
            .len() as i32;

        let input_with_empty_params = step
            .format(&parameters.with_placeholder_values())
            .map_err(|_| PromptTokensError::UnableToCompute)?;

        let template_tokens_used = tokenizer
            .tokenize_str(&input_with_empty_params.prompt)
            .map_err(|_e| PromptTokensError::UnableToCompute)?
            .len() as i32;

        let max_tokens = self.max_tokens_allowed(step);
        Ok(TokenCount::new(
            max_tokens,
            tokens_used,
            template_tokens_used,
        ))
    }

    fn max_tokens_allowed(&self, _step: &Self::Step) -> i32 {
        self.context_params().n_ctx
    }

    fn get_tokenizer(&self, _step: &Self::Step) -> Result<LLamaTokenizer, TokenizerError> {
        Ok(LLamaTokenizer::new(self))
    }

    fn get_text_splitter(&self, _step: &Self::Step) -> Result<Self::TextSplitter<'_>, Self::Error> {
        Ok(LLamaTextSplitter::new(self))
    }
}

pub struct LLamaTokenizer<'a> {
    context: &'a LLamaContext,
}

impl<'a> LLamaTokenizer<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        LLamaTokenizer {
            context: &executor.context,
        }
    }
}

impl Tokenizer<i32> for LLamaTokenizer<'_> {
    fn tokenize_str(&self, doc: &str) -> Result<Vec<i32>, TokenizerError> {
        let tokenized = llama_tokenize_helper(self.context, doc, true);
        Ok(tokenized)
    }

    fn to_string(&self, tokens: Vec<i32>) -> Result<String, TokenizerError> {
        let output = embedding_to_output(self.context, &tokens);
        Ok(output.to_string())
    }
}
