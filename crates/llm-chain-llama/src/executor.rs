use std::marker::PhantomData;
use std::sync::Arc;

use crate::context::{ContextParams, LLamaContext};
use crate::options::{get_executor_initial_opts, LlamaInvocation, DEFAULT_OPTIONS};
use crate::tokenizer::{embedding_to_output, llama_token_eos, tokenize, tokens_to_string};

use async_trait::async_trait;

use llm_chain::options::{Options, OptionsCascade};
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
        if $sender.send($val).await.is_err() {
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
    fn run_model(&self, input: LlamaInvocation) -> Output {
        let (sender, output) = Output::new_stream();
        // Tokenize the stop sequence and input prompt.
        let context = self.context.clone();
        let context_params = self.context_params.clone();
        let context_size = context_params.n_ctx as usize;
        let answer_prefix = self.answer_prefix(&input.prompt);
        tokio::task::spawn_blocking(move || {
            async move {
                let context_size = context_size;
                let context = context.lock().await;

                let tokenized_stop_prompt = tokenize(
                    &context,
                    input
                        .stop_sequence
                        .first() // XXX: Handle multiple stop seqs
                        .map(|x| x.as_str())
                        .unwrap_or("\n\n"),
                    false,
                );

                if tokenized_stop_prompt.len() > context_size {
                    must_send!(sender, StreamSegment::Err(ExecutorError::ContextTooSmall));
                    return;
                }

                let prompt_text = input.prompt.to_text();
                let tokenized_input = tokenize(&context, prompt_text.as_str(), true);
                if tokenized_input.len() > context_size {
                    must_send!(sender, StreamSegment::Err(ExecutorError::ContextTooSmall));
                    return;
                }

                // Embd contains the prompt and the completion. The longer the prompt, the shorter the completion.
                let mut embd = tokenized_input.clone();

                // Evaluate the prompt in full.
                bail!(
                    context
                        .llama_eval(
                            tokenized_input.as_slice(),
                            tokenized_input.len() as i32,
                            0,
                            &input,
                        )
                        .map_err(|e| ExecutorError::InnerError(e.into())),
                    sender
                );

                let mut n_remaining = context_size - tokenized_input.len();
                let mut n_used = tokenized_input.len() - 1;
                if let Some(prefix) = answer_prefix {
                    let tokenized_answer_prefix = tokenize(&context, prefix.as_str(), false);
                    if tokenized_answer_prefix.len() > context_size {
                        must_send!(sender, StreamSegment::Err(ExecutorError::ContextTooSmall));
                        return;
                    }

                    // Evaluate the answer prefix (the role -- should be Assistant: )
                    bail!(
                        context
                            .llama_eval(
                                tokenized_answer_prefix.as_slice(),
                                tokenized_answer_prefix.len() as i32,
                                n_used as i32,
                                &input,
                            )
                            .map_err(|e| ExecutorError::InnerError(e.into())),
                        sender
                    );
                    n_remaining -= tokenized_answer_prefix.len();
                    n_used += tokenized_answer_prefix.len();
                    embd.extend(tokenized_answer_prefix);
                }
                embd.resize(context_size, 0);
                let token_eos = llama_token_eos();
                let mut stop_sequence_i = 0;
                // Generate remaining tokens.
                while n_remaining > 0 {
                    let tok = context.llama_sample(
                        context_size as i32,
                        embd.as_slice(),
                        n_used as i32,
                        &input,
                    );
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
                    if tok == tokenized_stop_prompt[stop_sequence_i] {
                        stop_sequence_i += 1;
                        if stop_sequence_i >= tokenized_stop_prompt.len() {
                            break;
                        }
                    } else {
                        let str_output =
                            tokens_to_string(&context, &embd[n_used - stop_sequence_i..n_used]);
                        // XXX: make into chat if chat
                        must_send!(sender, StreamSegment::Content(str_output));
                        stop_sequence_i = 0;
                    }
                    bail!(
                        context
                            .llama_eval(&embd[n_used..], 1, n_used as i32, &input)
                            .map_err(|e| ExecutorError::InnerError(e.into())),
                        sender
                    );

                    if n_used >= tokenized_input.len() && stop_sequence_i == 0 {
                        let str_output = context.llama_token_to_str(&embd[n_used]);
                        // XXX: make into chat if chat
                        if sender
                            .send(StreamSegment::Content(str_output))
                            .await
                            .is_err()
                        {
                            panic!("Failed to send");
                        }
                    }
                }
            }
        });

        output
    }
}

/// Implement the ExecutorTrait for the Executor, defining methods for handling input and output.
#[async_trait]
impl ExecutorTrait for Executor {
    type StepTokenizer<'a> = LLamaTokenizer<'a>;
    fn new_with_options(options: Options) -> Result<Executor, ExecutorCreationError> {
        let cas = OptionsCascade::new()
            .with_options(&DEFAULT_OPTIONS)
            .with_options(&options);
        let (model_path, context_params) = get_executor_initial_opts(&cas).ok_or(
            ExecutorCreationError::FieldRequiredError("generic".to_string()),
        )?;
        Ok(Self {
            context: Arc::new(Mutex::new(LLamaContext::from_file_and_params(
                &model_path,
                Some(&context_params),
            ))),
            options,
            context_params,
        })
    }
    // Executes the model asynchronously and returns the output.
    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let invocation = LlamaInvocation::new(self.get_cascade(options), prompt)
            .ok_or(ExecutorError::InvalidOptions)?;
        Ok(self.run_model(invocation))
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
        self.context_params.n_ctx
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
        let context = self.context.blocking_lock();
        let tokenized = tokenize(&context, doc, true);
        Ok(tokenized.into())
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        let context = self.context.blocking_lock();
        let output = embedding_to_output(&context, &tokens.as_i32()?);
        Ok(output.to_string())
    }
}
