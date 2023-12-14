use std::marker::PhantomData;
use std::sync::Arc;

use crate::context::{ContextParams, LLamaContext, LlamaBatch};
use crate::options::{get_executor_initial_opts, LlamaInvocation, DEFAULT_OPTIONS};
use crate::tokenizer::{embedding_to_output, tokenize};

use async_trait::async_trait;

use llm_chain::options::{options_from_env, Options, OptionsCascade};
use llm_chain::output::{Output, StreamSegment};
use llm_chain::prompt::{ChatRole, Prompt};

use llm_chain::tokens::{PromptTokensError, TokenCollection, TokenCount};
use llm_chain::tokens::{Tokenizer, TokenizerError};
use llm_chain::traits::{Executor as ExecutorTrait, ExecutorCreationError, ExecutorError};
use tokio::sync::Mutex;

macro_rules! bail {
    ($val:expr, $sender:expr) => {
        match $val {
            Ok(value) => value,
            Err(err) => {
                must_send!($sender, StreamSegment::Err(err.into()));
                return;
            }
        }
    };
}

macro_rules! must_send {
    ($sender:expr, $val:expr) => {
        if $sender.send($val).is_err() {
            panic!("unable to send message");
        }
    };
}

/// Executor is responsible for running the LLAMA model and managing its context.
pub struct Executor {
    context: Arc<Mutex<LLamaContext>>,
    options: Options,
    context_params: ContextParams,
}

impl Executor {
    fn get_cascade<'a>(&'a self, options: &'a Options) -> OptionsCascade<'a> {
        let v: Vec<&'a Options> = vec![&DEFAULT_OPTIONS, &self.options, options];
        OptionsCascade::from_vec(v)
    }

    // Run the LLAMA model with the provided input and generate output.
    // Executes the model with the provided input and context parameters.
    async fn run_model(&self, input: LlamaInvocation) -> Output {
        let (sender, output) = Output::new_stream();
        // Tokenize the stop sequence and input prompt.
        let context = self.context.clone();
        let context_params = self.context_params.clone();
        let context_size = context_params.n_ctx as usize;
        let answer_prefix = self.answer_prefix(&input.prompt);
        tokio::task::spawn_blocking(move || {
            let context_size = context_size;
            let context = context.blocking_lock();

            let tokenized_stop_prompt = tokenize(
                &context,
                input
                    .stop_sequence
                    .first() // XXX: Handle multiple stop seqs
                    .map(|x| x.as_str())
                    .unwrap_or("\n\n"),
                false,
                true,
            );

            if tokenized_stop_prompt.len() > context_size {
                must_send!(sender, StreamSegment::Err(ExecutorError::ContextTooSmall));
                return;
            }

            let prompt_text = input.prompt.to_text();

            let tokenized_input = tokenize(&context, prompt_text.as_str(), true, false);
            if tokenized_input.len() > context_size {
                must_send!(sender, StreamSegment::Err(ExecutorError::ContextTooSmall));
                return;
            }

            // embd contains the prompt and the completion. The longer the
            // prompt, the shorter the completion.
            // It will initially contain a copy the tokenized prompt and then
            // may be extended with the tokenized answer prefix. After each
            // sampling the sampled token will also be added to this vector.
            // This is done so that the sampling function has access to all the
            // tokens which it may need for repetition penalties, etc.
            let mut embd = tokenized_input.clone();

            let mut batch = LlamaBatch::new_with_tokens(tokenized_input.clone(), 1);
            let last_idx = (batch.token_count() - 1) as usize;
            batch.enable_logits(last_idx);

            bail!(
                context
                    .llama_decode(&batch)
                    .map_err(|e| ExecutorError::InnerError(e.into())),
                sender
            );
            let mut n_cur = batch.token_count();
            let mut n_used = (batch.token_count() - 1) as usize;

            let mut n_remaining = context_size - tokenized_input.len();
            if let Some(prefix) = answer_prefix {
                let tokenized_answer_prefix = tokenize(&context, prefix.as_str(), true, true);
                if tokenized_answer_prefix.len() > context_size {
                    must_send!(sender, StreamSegment::Err(ExecutorError::ContextTooSmall));
                    return;
                }
                let batch = LlamaBatch::new_with_tokens(tokenized_answer_prefix.clone(), 1);
                // Evaluate the answer prefix (the role -- should be Assistant: )
                bail!(
                    context
                        .llama_decode(&batch)
                        .map_err(|e| ExecutorError::InnerError(e.into())),
                    sender
                );
                n_remaining -= tokenized_answer_prefix.len();
                embd.extend(tokenized_answer_prefix);
                n_cur += batch.token_count();
                n_used += (batch.token_count() - 1) as usize;
            }
            embd.resize(context_size, 0);
            let token_eos = context.llama_token_eos();

            let mut stop_sequence_i = 0;
            let mut n_batch = batch.token_count();
            let mut n_samples = 0;
            let ignore_initial_nls = input.prompt.to_text().ends_with('?');
            let nl_token = context.llama_token_nl();

            // Generate remaining tokens.
            while n_remaining > 0 {
                let tok = context.llama_sample(
                    context_size as i32,
                    embd.as_slice(),
                    n_used as i32,
                    &input,
                    n_batch as i32,
                );
                n_samples += 1;
                n_used += 1;
                n_remaining -= 1;
                embd[n_used] = tok;

                if tok == token_eos {
                    break;
                }
                if input.n_tok_predict != 0
                    && n_used > input.n_tok_predict + tokenized_input.len() - 1
                {
                    break;
                }

                // If the input prompt is in the form of a question then next
                // predicted tok will be a new line to finish off the question
                // itself, followed by another new line before the actual
                // answer. This is what the following is checking for.
                if n_samples <= 2 && ignore_initial_nls && tok == nl_token {
                    continue;
                }

                if tok == tokenized_stop_prompt[stop_sequence_i] {
                    stop_sequence_i += 1;
                    if stop_sequence_i >= tokenized_stop_prompt.len() {
                        break;
                    }
                } else {
                    let piece = bail!(
                        context
                            .llama_token_to_piece(tok)
                            .map_err(|e| ExecutorError::InnerError(e.into())),
                        sender
                    );
                    must_send!(sender, StreamSegment::Content(piece));
                    stop_sequence_i = 0;

                    let batch = LlamaBatch::new_with_token(tok, n_cur as i32);

                    n_batch = batch.token_count();
                    n_cur += 1;

                    bail!(
                        context
                            .llama_decode(&batch)
                            .map_err(|e| ExecutorError::InnerError(e.into())),
                        sender
                    );
                }
            }
        }); //JoinHandle is dropped? not sure how this works

        output
    }
}

/// Implement the ExecutorTrait for the Executor, defining methods for handling input and output.
#[async_trait]
impl ExecutorTrait for Executor {
    type StepTokenizer<'a> = LLamaTokenizer<'a>;
    fn new_with_options(options: Options) -> Result<Executor, ExecutorCreationError> {
        let opts_from_env =
            options_from_env().map_err(|err| ExecutorCreationError::InnerError(err.into()))?;
        let cas = OptionsCascade::new()
            .with_options(&DEFAULT_OPTIONS)
            .with_options(&opts_from_env)
            .with_options(&options);

        let (model_path, model_params, context_params) = get_executor_initial_opts(&cas)?;
        Ok(Self {
            context: Arc::new(Mutex::new(LLamaContext::from_file_and_params(
                &model_path,
                Some(&model_params),
                Some(&context_params),
            )?)),
            options,
            context_params,
        })
    }
    // Executes the model asynchronously and returns the output.
    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let invocation = LlamaInvocation::new(self.get_cascade(options), prompt)
            .map_err(|_| ExecutorError::InvalidOptions);
        Ok(self.run_model(invocation?).await)
    }

    fn tokens_used(
        &self,
        options: &Options,
        prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        let tokenizer = self.get_tokenizer(options)?;
        let input = prompt.to_text();
        let mut tokens_used = tokenizer
            .tokenize_str(&input)
            .map_err(|_e| PromptTokensError::UnableToCompute)?
            .len();
        // includes answer_prefix
        let answer_prefix = self.answer_prefix(prompt);
        if let Some(prefix) = answer_prefix {
            let answer_used = tokenizer
                .tokenize_str(&prefix)
                .map_err(|_e| PromptTokensError::UnableToCompute)?
                .len();
            tokens_used += answer_used
        }
        let max_tokens = self.max_tokens_allowed(options);
        Ok(TokenCount::new(max_tokens, tokens_used as i32))
    }

    fn answer_prefix(&self, prompt: &Prompt) -> Option<String> {
        if let llm_chain::prompt::Data::Chat(_) = prompt {
            // Tokenize answer prefix
            // XXX: Make the format dynamic
            let prefix = if prompt.to_text().ends_with('\n') {
                ""
            } else {
                "\n"
            };
            Some(format!("{}{}:", prefix, ChatRole::Assistant))
        } else {
            None
        }
    }

    fn max_tokens_allowed(&self, _step: &Options) -> i32 {
        self.context_params.n_ctx as i32
    }

    fn get_tokenizer(&self, _step: &Options) -> Result<LLamaTokenizer, TokenizerError> {
        Ok(LLamaTokenizer::new(self))
    }
}

pub struct LLamaTokenizer<'a> {
    _lifetime: PhantomData<&'a ()>,
    context: Arc<Mutex<LLamaContext>>,
}

impl<'a> LLamaTokenizer<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        LLamaTokenizer {
            context: executor.context.clone(),
            _lifetime: PhantomData::default(),
        }
    }
}

impl Tokenizer for LLamaTokenizer<'_> {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        let tokenized = tokio::task::block_in_place(|| {
            let context = self.context.blocking_lock();
            tokenize(&context, doc, true, false)
        });
        Ok(tokenized.into())
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        let tokens = &tokens.as_i32()?;
        let output = tokio::task::block_in_place(|| {
            let context = self.context.blocking_lock();
            embedding_to_output(&context, tokens)
        });
        Ok(output.to_string())
    }
}
