use llm_chain::{traits, Parameters, PromptTemplate};

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

#[derive(Debug, Clone, Default)]
/// LlamaConfig is an overridable collection of configuration parameters for the LLAMA model. It is combined with a prompt to create an invocation.
pub struct LlamaConfig {
    pub n_threads: Option<i32>,
    pub n_tok_predict: Option<usize>,
    pub top_k: Option<i32>,
    pub top_p: Option<f32>,
    pub temp: Option<f32>,
    pub repeat_penalty: Option<f32>,
    pub stop_sequence: Option<String>,
}

impl LlamaConfig {
    /// Creates a new LlamaConfig instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts the current LlamaConfig instance to a LlamaInvocation instance, using the given prompt.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The prompt string for the invocation.
    ///
    /// # Returns
    ///
    /// A LlamaInvocation instance with the specified configuration and prompt.
    fn to_invocation(&self, prompt: String) -> LlamaInvocation {
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
            prompt,
        }
    }
}

/// A step in a chain of LLAMA invocations. It is a combination of a prompt and a configuration.
pub struct Step {
    prompt: PromptTemplate,
    config: LlamaConfig,
}

impl Step {
    /// Create a new step with the given prompt and configuration.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The prompt template for the step.
    /// * `config` - An optional configuration for the step. If `None`, the default configuration will be used.
    pub fn new_with_config(prompt: PromptTemplate, config: Option<LlamaConfig>) -> Self {
        Self {
            prompt,
            config: config.unwrap_or_default(),
        }
    }

    /// Create a new step with the given prompt and default configuration.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The prompt template for the step.
    pub fn new(prompt: PromptTemplate) -> Self {
        Self::new_with_config(prompt, None)
    }
}

/// Implements the `Step` trait for the `Step` struct.
impl traits::Step for Step {
    type Output = LlamaInvocation;

    /// Formats the current step using the given parameters, creating a LlamaInvocation instance.
    ///
    /// # Arguments
    ///
    /// * `parameters` - The parameters used to format the prompt in the step.
    ///
    /// # Returns
    ///
    /// A LlamaInvocation instance with the formatted prompt and configuration.
    fn format(&self, parameters: &Parameters) -> Self::Output {
        self.config.to_invocation(self.prompt.format(parameters))
    }
}
