use llm_chain::traits::Options;
use serde::{Deserialize, Serialize};

use crate::context::ContextParams;

/// Represents a concrete call to the LLM model, with all the parameters specified, and no implicit behavior.
pub struct LlamaInvocation {
    pub(crate) n_threads: i32,
    pub(crate) n_tok_predict: usize,
    pub(crate) top_k: i32,
    pub(crate) top_p: f32,
    pub(crate) temp: f32,
    pub(crate) repeat_penalty: f32,
    pub(crate) stop_sequence: String,
    pub(crate) prompt: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// LlamaConfig is an overridable collection of configuration parameters for the LLAMA model. It is combined with a prompt to create an invocation.
pub struct PerInvocation {
    pub n_threads: Option<i32>,
    pub n_tok_predict: Option<usize>,
    pub top_k: Option<i32>,
    pub top_p: Option<f32>,
    pub temp: Option<f32>,
    pub repeat_penalty: Option<f32>,
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
    /// * `prompt` - The prompt string for the invocation.
    ///
    /// # Returns
    ///
    /// A LlamaInvocation instance with the specified configuration and prompt.
    pub(crate) fn to_invocation(&self, prompt: &str) -> LlamaInvocation {
        LlamaInvocation {
            n_threads: self.n_threads.unwrap_or(1),
            n_tok_predict: self.n_tok_predict.unwrap_or(0),
            top_k: self.top_k.unwrap_or(40),
            top_p: self.top_p.unwrap_or(0.0),
            temp: self.temp.unwrap_or(0.7),
            repeat_penalty: self.repeat_penalty.unwrap_or(1.2),
            stop_sequence: self
                .stop_sequence
                .clone()
                .unwrap_or_else(|| "\n\n".to_string()),
            prompt: prompt.to_string(),
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
}
impl Options for PerExecutor {}
