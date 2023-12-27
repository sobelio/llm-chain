use lazy_static::lazy_static;
use llm_chain::{
    options,
    options::{Opt, OptDiscriminants, Options, OptionsCascade},
    prompt::Prompt,
    traits::ExecutorCreationError,
};

use std::collections::HashMap;

use crate::context::ContextParams;
use crate::model::ModelParams;

/// Represents a concrete call to the LLM model, with all the parameters specified, and no implicit behavior.
pub struct LlamaInvocation {
    pub(crate) n_tok_predict: usize,
    pub(crate) logit_bias: HashMap<i32, f32>,
    pub(crate) top_k: i32,
    pub(crate) top_p: f32,
    pub(crate) tfs_z: f32,
    pub(crate) typical_p: f32,
    pub(crate) temp: f32,
    pub(crate) repeat_penalty: f32,
    pub(crate) repeat_last_n: i32,
    pub(crate) frequency_penalty: f32,
    pub(crate) presence_penalty: f32,
    pub(crate) mirostat: i32,
    pub(crate) mirostat_tau: f32,
    pub(crate) mirostat_eta: f32,
    pub(crate) penalize_nl: bool,
    pub(crate) stop_sequence: Vec<String>,
    pub(crate) prompt: Prompt,
}

macro_rules! opt_extract {
    ($opt:ident, $var:ident, $discriminant:ident) => {
        if let Some(Opt::$discriminant($var)) = $opt.get(OptDiscriminants::$discriminant) {
            Ok($var)
        } else {
            Err(ExecutorCreationError::FieldRequiredError(
                stringify!($discriminant).to_string(),
            ))
        }
    };
}

impl LlamaInvocation {
    pub(crate) fn new(
        opt: OptionsCascade,
        prompt: &Prompt,
    ) -> Result<LlamaInvocation, ExecutorCreationError> {
        let n_tok_predict = opt_extract!(opt, n_tok_predict, MaxTokens)?;
        let top_k = opt_extract!(opt, top_k, TopK)?;
        let top_p = opt_extract!(opt, top_p, TopP)?;
        let tfs_z = opt_extract!(opt, tfs_z, TfsZ)?;
        let typical_p = opt_extract!(opt, typical_p, TypicalP)?;
        let temp = opt_extract!(opt, temp, Temperature)?;
        let repeat_penalty = opt_extract!(opt, repeat_penalty, RepeatPenalty)?;
        let repeat_last_n = opt_extract!(opt, repeat_last_n, RepeatPenaltyLastN)?;
        let frequency_penalty = opt_extract!(opt, frequency_penalty, FrequencyPenalty)?;
        let presence_penalty = opt_extract!(opt, presence_penalty, PresencePenalty)?;
        let mirostat = opt_extract!(opt, mirostat, Mirostat)?;
        let mirostat_tau = opt_extract!(opt, mirostat_tau, MirostatTau)?;
        let mirostat_eta = opt_extract!(opt, mirostat_eta, MirostatEta)?;
        let penalize_nl = opt_extract!(opt, penalize_nl, PenalizeNl)?;
        let stop_sequence = opt_extract!(opt, stop_sequence, StopSequence)?;

        // Skip TokenBias for now
        let logit_bias = HashMap::<i32, f32>::new(); // token_bias.as_i32_f32_hashmap()?;

        Ok(LlamaInvocation {
            n_tok_predict: *n_tok_predict,
            logit_bias,
            top_k: *top_k,
            top_p: *top_p,
            tfs_z: *tfs_z,
            typical_p: *typical_p,
            temp: *temp,
            repeat_penalty: *repeat_penalty,
            repeat_last_n: *repeat_last_n as i32,
            frequency_penalty: *frequency_penalty,
            presence_penalty: *presence_penalty,
            mirostat: *mirostat,
            mirostat_tau: *mirostat_tau,
            mirostat_eta: *mirostat_eta,
            penalize_nl: *penalize_nl,
            stop_sequence: stop_sequence.clone(),
            prompt: prompt.clone(),
        })
    }
}

lazy_static! {
    pub(crate) static ref DEFAULT_OPTIONS: Options = options!(
        // ModelType: "llama", // not used
        NThreads: 1_usize,
        MaxTokens: 0_usize,
        MaxBatchSize: 512_usize,
        MaxContextSize: 2048_usize,
        TopK: 40_i32,
        TopP: 0.95,
        TfsZ: 1.0,
        TypicalP: 1.0,
        Temperature: 0.8,
        RepeatPenalty: 1.1,
        RepeatPenaltyLastN: 64_usize,
        FrequencyPenalty: 1.1,
        PresencePenalty: 0.0,
        Mirostat: 0_i32,
        MirostatTau: 5.0,
        MirostatEta: 0.1,
        PenalizeNl: true,
        StopSequence: vec!["\n\n".to_string()],
        NGpuLayers: 0_i32,
        MainGpu: 0_i32,
        TensorSplit: None,
        VocabOnly: false,
        UseMmap: true,
        UseMlock: false
    );
}

pub(crate) fn get_executor_initial_opts(
    opt: &OptionsCascade,
) -> Result<(String, ModelParams, ContextParams), ExecutorCreationError> {
    let model = opt_extract!(opt, model, Model)?;

    let mut mp = ModelParams::new();
    if let Some(Opt::NGpuLayers(value)) = opt.get(OptDiscriminants::NGpuLayers) {
        mp.n_gpu_layers = *value;
    }
    if let Some(Opt::MainGpu(value)) = opt.get(OptDiscriminants::MainGpu) {
        mp.main_gpu = *value;
    }
    if let Some(Opt::TensorSplit(values)) = opt.get(OptDiscriminants::TensorSplit) {
        mp.tensor_split = values.clone();
    }
    // Currently, the setting of vocab_only is not allowed as it will cause
    // a crash when using the llama executor which needs to have wieghts loaded
    // in order to work.
    mp.vocab_only = false;

    if let Some(Opt::UseMmap(value)) = opt.get(OptDiscriminants::UseMmap) {
        mp.use_mmap = *value;
    }
    if let Some(Opt::UseMlock(value)) = opt.get(OptDiscriminants::UseMlock) {
        mp.use_mlock = *value;
    }

    let mut cp = ContextParams::new();
    if let Some(Opt::NThreads(value)) = opt.get(OptDiscriminants::NThreads) {
        cp.n_threads = *value as u32;
    }

    let max_context_size = opt_extract!(opt, max_context_size, MaxContextSize)?;
    cp.n_ctx = *max_context_size as u32;

    let n_batch = opt_extract!(opt, nbatch, MaxBatchSize)?;
    cp.n_batch = *n_batch as u32;
    if max_context_size < n_batch {
        return Err(ExecutorCreationError::InvalidValue(
            "MaxBatchSize must be less than or equal to MaxContextSize".to_string(),
        ));
    }

    Ok((model.to_path(), mp, cp))
}
