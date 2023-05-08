use llm_chain::prompt::Prompt;
use llm_chain::traits::Options;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::context::ContextParams;

/// Represents a concrete call to the LLM model, with all the parameters specified, and no implicit behavior.
pub struct LlamaInvocation {
    pub(crate) n_threads: i32,
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
    pub(crate) stop_sequence: String,
    pub(crate) prompt: Prompt,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// pub(crate) LlamaConfig is an overridable collection of configuration parameters for the LLAMA model. It is combined with a prompt to create an invocation.
pub struct PerInvocation {
    pub n_threads: Option<i32>,
    pub n_tok_predict: Option<usize>,
    pub top_k: Option<i32>,
    pub top_p: Option<f32>,
    pub tfs_z: Option<f32>,
    pub typical_p: Option<f32>,
    pub temp: Option<f32>,
    pub repeat_penalty: Option<f32>,
    pub repeat_last_n: Option<i32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub mirostat: Option<i32>,
    pub mirostat_tau: Option<f32>,
    pub mirostat_eta: Option<f32>,
    pub penalize_nl: Option<bool>,
    pub stop_sequence: Option<String>,
}

impl Options for PerInvocation {}

impl PerInvocation {
    /// Creates a new `PerInvocation` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts the current `PerInvocation` instance to a LlamaInvocation instance, using the given prompt.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The prompt for the invocation.
    ///
    /// # Returns
    ///
    /// A LlamaInvocation instance with the specified configuration and prompt.
    pub(crate) fn to_invocation(&self, prompt: &Prompt) -> LlamaInvocation {
        LlamaInvocation {
            n_threads: self.n_threads.unwrap_or(1),
            n_tok_predict: self.n_tok_predict.unwrap_or(0),
            logit_bias: HashMap::new(),
            top_k: self.top_k.unwrap_or(40),
            top_p: self.top_p.unwrap_or(0.95),
            tfs_z: self.tfs_z.unwrap_or(1.0),
            typical_p: self.typical_p.unwrap_or(1.0),
            temp: self.temp.unwrap_or(0.8),
            repeat_penalty: self.repeat_penalty.unwrap_or(1.1),
            repeat_last_n: self.repeat_last_n.unwrap_or(64),
            frequency_penalty: self.frequency_penalty.unwrap_or(0.0),
            presence_penalty: self.presence_penalty.unwrap_or(0.0),
            mirostat: self.mirostat.unwrap_or(0),
            mirostat_tau: self.mirostat_tau.unwrap_or(5.0),
            mirostat_eta: self.mirostat_eta.unwrap_or(0.1),
            penalize_nl: self.penalize_nl.unwrap_or(true),
            stop_sequence: self
                .stop_sequence
                .clone()
                .unwrap_or_else(|| "\n\n".to_string()),
            prompt: prompt.clone(),
        }
    }
}

/// `PerExecutor` represents a collection of configuration parameters for the executor of the LLAMA model.
/// It contains optional fields for the model path and context parameters.
///
/// # Examples
///
/// ```
/// use llm_chain_llama::PerExecutor;
/// let executor = PerExecutor::new().with_model_path("path/to/model");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerExecutor {
    /// Optional path to the LLAMA model.
    pub(crate) model_path: Option<String>,
    /// Optional context parameters for the LLAMA model.
    pub(crate) context_params: Option<ContextParams>,
}

impl PerExecutor {
    /// Creates a new `PerExecutor` instance with default values.
    ///
    /// # Returns
    ///
    /// A `PerExecutor` instance with default values for the model path and context parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the model path for the current `PerExecutor` instance.
    ///
    /// # Arguments
    ///
    /// * `model_path` - The path to the LLAMA model.
    ///
    /// # Returns
    ///
    /// A new `PerExecutor` instance with the updated model path.
    pub fn with_model_path(mut self, model_path: &str) -> Self {
        self.model_path = Some(model_path.to_string());
        self
    }

    /// Sets the context_params for the current `PerExecutor` instance.
    ///
    /// # Arguments
    ///
    /// * `context_params` - LLama Context Params  
    ///
    /// # Returns
    ///
    /// A new `PerExecutor` instance with the updated context_params
    pub fn with_context_params(mut self, context_params: ContextParams) -> Self {
        self.context_params = Some(context_params);
        self
    }
}
impl Options for PerExecutor {}
