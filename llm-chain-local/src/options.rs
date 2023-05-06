use llm::{InferenceParameters, InferenceWithPromptParameters, ModelParameters};
use llm_chain::traits::Options;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// An overridable collection of configuration parameters for an LLM. It is combined with a prompt to create an invocation.
pub struct PerInvocation {
    pub n_threads: Option<usize>,
    pub n_tok_predict: Option<usize>,
    pub top_k: Option<usize>,
    pub top_p: Option<f32>,
    pub temp: Option<f32>,
    pub repeat_penalty: Option<f32>,
    pub stop_sequence: Option<String>,
}

impl Options for PerInvocation {}

impl Into<ModelParameters> for PerInvocation {
    fn into(self) -> ModelParameters {
        let inference_params = InferenceParameters {
            n_threads: self.n_threads.unwrap_or(4),
            n_batch: 8,
            top_k: self.top_k.unwrap_or(40),
            top_p: self.top_p.unwrap_or(0.95),
            repeat_penalty: self.temp.unwrap_or(1.3),
            temperature: self.temp.unwrap_or(0.8),
            bias_tokens: Default::default(),
        };

        let prompt_params = InferenceWithPromptParameters {
            play_back_previous_tokens: false,
            maximum_token_count: None,
        };

        ModelParameters {
            prefer_mmap: true,
            n_context_tokens: 2048,
            inference_params,
            inference_prompt_params: prompt_params,
        }
    }
}

/// `PerExecutor` represents a collection of configuration parameters for the executor of the LLM.
/// It contains optional fields for the model path and context parameters.
///
/// # Examples
///
/// ```
/// use llm_chain_local::PerExecutor;
/// let executor = PerExecutor::new().with_model_path("path/to/model");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerExecutor {
    /// Optional path to the LLM.
    pub model_path: Option<String>,
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
        self.model_path = Some(model_path.to_string());
        self
    }
}

impl Options for PerExecutor {}
