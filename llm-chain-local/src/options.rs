use std::path::PathBuf;

use llm::{InferenceParameters, ModelParameters};
use llm_chain::traits::Options;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// An overridable collection of configuration parameters for an LLM. It is combined with a prompt to create an invocation.
pub struct PerInvocation {
    pub n_threads: Option<usize>,
    pub n_batch: Option<usize>,
    pub n_tok_predict: Option<usize>,
    pub top_k: Option<usize>,
    pub top_p: Option<f32>,
    pub temp: Option<f32>,
    /// A comma separated list of token biases. The list should be in the format
    /// "TID=BIAS,TID=BIAS" where TID is an integer token ID and BIAS is a
    /// floating point number.
    /// For example, "1=-1.0,2=-1.0" sets the bias for token IDs 1
    /// (start of document) and 2 (end of document) to -1.0 which effectively
    /// disables the model from generating responses containing those token IDs.
    pub bias_tokens: Option<String>,
    pub repeat_penalty: Option<f32>,
    pub repeat_penalty_last_n: Option<usize>,
}

impl Options for PerInvocation {}

impl Into<ModelParameters> for PerInvocation {
    fn into(self) -> ModelParameters {
        let inference_parameters = InferenceParameters {
            n_threads: self.n_threads.unwrap_or(4),
            n_batch: self.n_batch.unwrap_or(8),
            top_k: self.top_k.unwrap_or(40),
            top_p: self.top_p.unwrap_or(0.95),
            repeat_penalty: self.temp.unwrap_or(1.3),
            repetition_penalty_last_n: self.repeat_penalty_last_n.unwrap_or(512),
            temperature: self.temp.unwrap_or(0.8),
            bias_tokens: Default::default(),
        };

        ModelParameters {
            prefer_mmap: true,
            n_context_tokens: 2048,
            inference_parameters,
        }
    }
}

/// `PerExecutor` represents a collection of configuration parameters for the executor of the LLM.
/// It contains optional fields for the model path and context parameters.
///
/// # Examples
///
/// ```
/// use llm_chain_local::options::PerExecutor;
/// let executor = PerExecutor::new().with_model_path("path/to/model");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerExecutor {
    /// Optional path to the LLM.
    pub model_path: Option<PathBuf>,
    /// Optional type (e.g. LLaMA, GPT-Neo-X) of the LLM.
    pub model_type: Option<String>,
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
    /// * `model_path` - The path to the LLM.
    ///
    /// # Returns
    ///
    /// A new `PerExecutor` instance with the updated model path.
    pub fn with_model_path(mut self, model_path: &str) -> Self {
        self.model_path = Some(PathBuf::from(model_path));
        self
    }
}

impl Options for PerExecutor {}
