use async_trait::async_trait;
use lazy_static::lazy_static;
use llm::{
    load_progress_callback_stdout, InferenceError, InferenceParameters, InferenceRequest, Model,
    ModelArchitecture, ModelParameters, TokenUtf8Buffer,
};
use llm_chain::options;
use llm_chain::options::{options_from_env, Opt, OptDiscriminants, Options, OptionsCascade};
use llm_chain::output::Output;
use llm_chain::prompt::Prompt;
use llm_chain::tokens::{
    PromptTokensError, TokenCollection, TokenCount, Tokenizer, TokenizerError,
};
use llm_chain::traits::{ExecutorCreationError, ExecutorError};
use std::convert::Infallible;
use std::path::Path;
use thiserror::Error;

lazy_static! {
    static ref DEFAULT_OPTIONS: Options = options!(
        NThreads: 4_usize,
        NBatch: 8_usize,
        TopK: 40_i32,
        TopP: 0.95,
        RepeatPenalty: 1.3,
        RepeatPenaltyLastN: 512_usize,
        Temperature: 0.8
    );
}
/// Executor is responsible for running the LLM and managing its context.
pub struct Executor {
    llm: Box<dyn Model>,
    options: Options,
}

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
    type StepTokenizer<'a> = LocalLlmTokenizer<'a>;

    fn new_with_options(options: Options) -> Result<Self, ExecutorCreationError> {
        let opts_from_env = options_from_env().unwrap();
        let opts_cas = OptionsCascade::new()
            .with_options(&opts_from_env)
            .with_options(&options);

        let model_type = opts_cas
            .get(OptDiscriminants::ModelType)
            .and_then(|x| match x {
                Opt::ModelType(mt) => Some(mt),
                _ => None,
            })
            .ok_or(ExecutorCreationError::FieldRequiredError(
                "model_type".to_string(),
            ))?;
        let model_path = opts_cas
            .get(OptDiscriminants::Model)
            .and_then(|x| match x {
                Opt::Model(m) => Some(m),
                _ => None,
            })
            .ok_or(ExecutorCreationError::FieldRequiredError(
                "model_path".to_string(),
            ))?;

        let model_arch = model_type
            .parse::<ModelArchitecture>()
            .map_err(|e| ExecutorCreationError::InnerError(Box::new(e)))?;
        let llm: Box<dyn Model> = llm::load_dynamic(
            model_arch,
            Path::new(&model_path.to_path()),
            model_params_from_options(opts_cas).map_err(|_| {
                ExecutorCreationError::FieldRequiredError("Default field missing".to_string())
            })?,
            load_progress_callback_stdout,
        )
        .map_err(|e| ExecutorCreationError::InnerError(Box::new(e)))?;

        Ok(Executor { llm, options })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let opts = OptionsCascade::new()
            .with_options(&DEFAULT_OPTIONS)
            .with_options(&self.options)
            .with_options(options);
        let session = &mut self.llm.start_session(Default::default());
        let mut output = String::new();
        session
            .infer::<Infallible>(
                self.llm.as_ref(),
                &mut rand::thread_rng(),
                &InferenceRequest {
                    prompt: prompt.to_text().as_str(),
                    parameters: Some(
                        &inference_params_from_options(opts)
                            .map_err(|_| ExecutorError::InvalidOptions)?,
                    ),
                    // playback_previous_tokens
                    // maximum_token_count
                    ..Default::default()
                },
                // OutputRequest
                &mut Default::default(),
                |t| {
                    output.push_str(t);

                    Ok(())
                },
            )
            .map_err(|e| match e {
                InferenceError::ContextFull => Error::ContextFull,
                InferenceError::EndOfText => Error::EndOfText,
                InferenceError::TokenizationFailed => Error::TokenizationFailed,
                InferenceError::UserCallback(_) => {
                    panic!("user callback error should not be possible")
                }
            })
            .map_err(|e| ExecutorError::InnerError(e.into()))?;
        Ok(Output::new_immediate(Prompt::text(output)))
    }

    fn tokens_used(
        &self,
        options: &Options,
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

    fn max_tokens_allowed(&self, _: &Options) -> i32 {
        self.llm.n_context_tokens().try_into().unwrap_or(2048)
    }

    fn answer_prefix(&self, _prompt: &Prompt) -> Option<String> {
        None
    }

    fn get_tokenizer(&self, _: &Options) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
        Ok(LocalLlmTokenizer::new(self))
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

impl Tokenizer for LocalLlmTokenizer<'_> {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        match &self.llm.vocabulary().tokenize(doc, false) {
            Ok(tokens) => Ok(tokens.iter().map(|t| t.1).collect::<Vec<_>>().into()),
            Err(_) => Err(TokenizerError::TokenizationError),
        }
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        let mut res = String::new();
        let mut token_utf8_buf = TokenUtf8Buffer::new();
        for token_id in tokens.as_i32()? {
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

fn model_params_from_options(opts: OptionsCascade) -> Result<ModelParameters, ()> {
    Ok(ModelParameters {
        prefer_mmap: true,
        n_context_tokens: 2048,
        inference_parameters: inference_params_from_options(opts)?,
    })
}

fn inference_params_from_options(opts: OptionsCascade) -> Result<InferenceParameters, ()> {
    let Some(Opt::NThreads(n_threads)) = opts.get(OptDiscriminants::NThreads) else {
        return Err(())
    };
    let Some(Opt::NBatch(n_batch)) = opts.get(OptDiscriminants::NBatch) else {
        return Err(())
    };
    let Some(Opt::TopK(top_k)) = opts.get(OptDiscriminants::TopK) else {
        return Err(())
    };
    let Some(Opt::TopP(top_p)) = opts.get(OptDiscriminants::TopP) else {
        return Err(());
    };
    let Some(Opt::RepeatPenalty(repeat_penalty)) = opts.get(OptDiscriminants::RepeatPenalty) else {
        return Err(());
    };

    let Some(Opt::RepeatPenaltyLastN(repetition_penalty_last_n)) = opts.get(OptDiscriminants::RepeatPenaltyLastN) else {
        return Err(());
    };

    let Some(Opt::Temperature(temperature)) = opts.get(OptDiscriminants::Temperature) else {
        return Err(());
    };

    let inference_parameters = InferenceParameters {
        n_threads: *n_threads,
        n_batch: *n_batch,
        top_k: *top_k as usize,
        top_p: *top_p,
        repeat_penalty: *repeat_penalty,
        repetition_penalty_last_n: *repetition_penalty_last_n,
        temperature: *temperature,
        bias_tokens: Default::default(),
    };
    Ok(inference_parameters)
}
