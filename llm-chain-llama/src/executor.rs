use crate::context::{LLamaContext, LlamaContextParams};
use crate::step::{LlamaInvocation, Step as LLamaStep};
use crate::tokenizer::{embedding_to_output, llama_token_eos, llama_tokenize_helper, tokenize};

use crate::output::Output;
use async_trait::async_trait;

use llm_chain::traits;
use llm_chain::traits::Executor as ExecutorTrait;
use llm_chain::Parameters;
use llm_chain_llama_sys::llama_context_params;

/// Executor is responsible for running the LLAMA model and managing its context.
pub struct Executor {
    context: LLamaContext,
    context_params: Option<LlamaContextParams>,
}

impl Executor {
    /// Creates a new executor with the given client and optional context parameters.
    pub fn new_with_config(model_path: String, context_params: Option<LlamaContextParams>) -> Self {
        let context = LLamaContext::from_file_and_params(&model_path, &context_params);
        Self {
            context,
            context_params,
        }
    }

    /// Creates a new executor for the given model with default context parameters.
    pub fn new(model_path: String) -> Self {
        Self::new_with_config(model_path, None)
    }

    fn context_params(&self) -> llama_context_params {
        LlamaContextParams::or_default(&self.context_params)
    }
}

// Executes the model with the provided input and context parameters.
fn run_model(
    input_ctx: &LLamaContext,
    input: LlamaInvocation,
    context_params_c: llama_context_params,
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
    }
    embedding_to_output(
        input_ctx,
        &embd[tokenized_input.len()..n_used + 1 - stop_sequence_i],
    )
}

impl Executor {
    // Run the LLAMA model with the provided input and generate output.
    fn run_model(&self, input: LlamaInvocation) -> Output {
        run_model(&self.context, input, self.context_params())
    }
}

// Implement the ExecutorTrait for the Executor, defining methods for handling input and output.
#[async_trait]
impl ExecutorTrait for Executor {
    type Step = LLamaStep;
    type Output = Output;
    // Executes the model asynchronously and returns the output.
    async fn execute(
        &self,
        input: <<Executor as ExecutorTrait>::Step as traits::Step>::Output,
    ) -> Self::Output {
        self.run_model(input)
    }

    // Applies the output to the given parameters.
    fn apply_output_to_parameters(parameters: Parameters, output: &Self::Output) -> Parameters {
        parameters.with_text(output.to_owned())
    }

    // Combines two outputs into a single output.
    fn combine_outputs(output: &Self::Output, other: &Self::Output) -> Self::Output {
        output.combine(other)
    }
}

impl traits::ExecutorPromptTokens<LLamaStep> for Executor {
    fn count_tokens_for_doc(
        &self,
        _step: &LLamaStep,
        doc: &str,
    ) -> Result<usize, traits::PromptTokensError> {
        Ok(llama_tokenize_helper(&self.context, doc, true).len())
    }
    fn max_tokens(&self, _step: &LLamaStep) -> Result<usize, traits::PromptTokensError> {
        self.context_params()
            .n_ctx
            .try_into()
            .map_err(|_| traits::PromptTokensError::UnableToCompute)
    }
    fn count_prompt_tokens(&self, step: &LLamaStep) -> Result<usize, traits::PromptTokensError> {
        let template = step.prompt_source();
        Ok(llama_tokenize_helper(&self.context, template, true).len())
    }
    fn split_at_tokens(
        &self,
        _step: &LLamaStep,
        doc: &str,
        tokens: usize,
    ) -> Result<(String, String), traits::PromptTokensError> {
        let tokenized = llama_tokenize_helper(&self.context, doc, true);
        if tokenized.len() < tokens {
            Ok((doc.to_owned(), "".to_owned()))
        } else {
            let (new_doc, rest) = tokenized.split_at(tokens);
            Ok((
                embedding_to_output(&self.context, new_doc).into(),
                embedding_to_output(&self.context, rest).into(),
            ))
        }
    }
    fn count_tokens_for_output(
        &self,
        step: &LLamaStep,
        output: &Self::Output,
    ) -> Result<usize, traits::PromptTokensError> {
        let template = step.prompt_source();
        let tokenized = tokenize(
            &self.context,
            template,
            self.context_params().n_ctx as usize,
            true,
        )
        .map_err(|e| match e {
            crate::tokenizer::TokenizeError::InputTooLong => {
                traits::PromptTokensError::UnableToCompute
            }
        })?;
        let output_tokens = llama_tokenize_helper(&self.context, output.as_str(), true);
        Ok(tokenized.len() + output_tokens.len())
    }
}
