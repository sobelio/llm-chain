use crate::context::{ContextParams, LLamaContext};
use crate::options::PerInvocation;
use crate::options::{LlamaInvocation, PerExecutor};
use crate::tokenizer::{embedding_to_output, llama_token_eos, llama_tokenize_helper, tokenize};
use crate::LLamaTextSplitter;

use crate::output::Output;
use async_trait::async_trait;

use llm_chain::prompt::{Prompt, ChatRole};

use llm_chain::tokens::{PromptTokensError, TokenCount};
use llm_chain::tokens::{Tokenizer, TokenizerError};
use llm_chain::traits::{Executor as ExecutorTrait, ExecutorCreationError, ExecutorError};

use llm_chain_llama_sys::llama_context_params;
/// Executor is responsible for running the LLAMA model and managing its context.
pub struct Executor {
    context: LLamaContext,
    options: Option<PerExecutor>,
    callback: Option<fn(&Output)>,
    invocation_options: Option<PerInvocation>,
}

impl Executor {
    pub fn with_callback(mut self, callback: fn(&Output)) -> Self {
        self.callback = Some(callback);
        self
    }
    fn context_params(&self) -> llama_context_params {
        let cp = self
            .options
            .as_ref()
            .and_then(|p| p.context_params.as_ref());
        ContextParams::or_default(cp)
    }

    pub(crate) fn get_context(&self) -> &LLamaContext {
        &self.context
    }
}

impl Executor {
    // Run the LLAMA model with the provided input and generate output.
    // Executes the model with the provided input and context parameters.
    fn run_model(&self, input: LlamaInvocation) -> Output {
        // Tokenize the stop sequence and input prompt.
        let context_params = self.context_params();

        let tokenized_stop_prompt = tokenize(
            &self.context,
            input.stop_sequence.as_str(),
            context_params.n_ctx as usize,
            false,
        )
        .unwrap();


        let prompt_text = input.prompt.to_text();
        let tokenized_input = tokenize(
            &self.context,
            prompt_text.as_str(),
            context_params.n_ctx as usize,
            true,
        ).unwrap();
        // Embd contains the prompt and the completion. The longer the prompt, the shorter the completion.
        let mut embd = tokenized_input.clone();

        // Evaluate the prompt in full.
        self.context
            .llama_eval(
                tokenized_input.as_slice(),
                tokenized_input.len() as i32,
                0,
                &input,
            )
            .unwrap();
        let mut n_remaining = self.context_params().n_ctx - tokenized_input.len() as i32;
        let mut n_used = tokenized_input.len() - 1;
        if let Some(prefix) = self.answer_prefix(&input.prompt) {
            let tokenized_answer_prefix = tokenize(
                &self.context,
                prefix.as_str(),
                context_params.n_ctx as usize,
                false,
            )
            .unwrap();
            // Evaluate the answer prefix (the role -- should be Assistant: )
            self.context
                .llama_eval(
                    tokenized_answer_prefix.as_slice(),
                    tokenized_answer_prefix.len() as i32,
                    n_used as i32,
                    &input,
                )
                .unwrap();
            n_remaining -= tokenized_answer_prefix.len() as i32;
            n_used += tokenized_answer_prefix.len();
            embd.extend(tokenized_answer_prefix);
        }
        embd.resize(context_params.n_ctx as usize, 0);
        let token_eos = llama_token_eos();
        let mut stop_sequence_i = 0;
        // Generate remaining tokens.
        while n_remaining > 0 {
            let tok = self.context.llama_sample(embd.as_slice(), n_used as i32, &input);
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
            self.context
                .llama_eval(&embd[n_used..], 1, n_used as i32, &input)
                .unwrap();

            if let Some(callback) = self.callback {
                let output = self.context.llama_token_to_str(&embd[n_used]);
                callback(&output.into());
            }
        }
        embedding_to_output(
            &self.context,
            &embd[tokenized_input.len()..n_used + 1 - stop_sequence_i],
        )
    }

}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unable to tokenize prompt")]
    PromptTokensError(PromptTokensError),
}

impl ExecutorError for Error {}

// Implement the ExecutorTrait for the Executor, defining methods for handling input and output.
#[async_trait]
impl ExecutorTrait for Executor {
    type Output = Output;
    type Token = i32;
    type StepTokenizer<'a> = LLamaTokenizer<'a>;
    type TextSplitter<'a> = LLamaTextSplitter<'a>;
    type Error = Error;
    type PerInvocationOptions = PerInvocation;
    type PerExecutorOptions = PerExecutor;
    fn new_with_options(
        executor_options: Option<Self::PerExecutorOptions>,
        invocation_options: Option<Self::PerInvocationOptions>,
    ) -> Result<Executor, ExecutorCreationError> {
        let context_params = match executor_options.as_ref() {
            Some(options) => options.context_params.clone(),
            None => None,
        };
        let model_path = executor_options
            .as_ref()
            .and_then(|x| x.model_path.clone())
            .or_else(|| std::env::var("LLAMA_MODEL_PATH").ok())
            .ok_or(ExecutorCreationError::FieldRequiredError(
                "model_path, ensure to provide the parameter or set `LLAMA_MODEL_PATH` environment variable ".to_string(),
            ))?;
        Ok(Self {
            context: LLamaContext::from_file_and_params(&model_path, context_params.as_ref()),
            options: executor_options,
            callback: None,
            invocation_options,
        })
    }
    // Executes the model asynchronously and returns the output.
    async fn execute(
        &self,
        options: Option<&Self::PerInvocationOptions>,
        prompt: &Prompt,
    ) -> Result<Self::Output, Self::Error> {
        let config = match options {
            Some(options) => options.clone(),
            None => self.invocation_options.clone().unwrap_or_default(),
        };
        let invocation = config.to_invocation(prompt);
        Ok(self.run_model(invocation))
    }

    fn tokens_used(
        &self,
        options: Option<&Self::PerInvocationOptions>,
        prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        let tokenizer = self.get_tokenizer(options)?;
        let input = prompt.to_text();
        let mut tokens_used = tokenizer
            .tokenize_str(&input)
            .map_err(|_e| PromptTokensError::UnableToCompute)?
            .len() as i32;
        // includes answer_prefix
        let answer_prefix = self.answer_prefix(prompt);
        if let Some(prefix) = answer_prefix {
            let answer_used = tokenizer
            .tokenize_str(&prefix)
            .map_err(|_e| PromptTokensError::UnableToCompute)?
            .len() as i32;
            tokens_used += answer_used
        }
        let max_tokens = self.max_tokens_allowed(options);
        Ok(TokenCount::new(max_tokens, tokens_used))
    }

    fn answer_prefix(
        &self, prompt: &Prompt) -> Option<String> {
        if let llm_chain::prompt::Data::Chat(_) = prompt {
            // Tokenize answer prefix
            // XXX: Make the format dynamic
            let prefix = if prompt.to_text().ends_with('\n') { "" } else { "\n" };
            Some(format!("{}{}:", prefix, ChatRole::Assistant))
        }
        else {
            None
        }

    }

    fn max_tokens_allowed(&self, _step: Option<&PerInvocation>) -> i32 {
        self.context_params().n_ctx
    }

    fn get_tokenizer(
        &self,
        _step: Option<&Self::PerInvocationOptions>,
    ) -> Result<LLamaTokenizer, TokenizerError> {
        Ok(LLamaTokenizer::new(self))
    }

    fn get_text_splitter(
        &self,
        _step: Option<&Self::PerInvocationOptions>,
    ) -> Result<Self::TextSplitter<'_>, Self::Error> {
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
