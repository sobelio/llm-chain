use super::error::MiniMaxInnerError;
use super::model::Model;
use super::prompt::completion_to_output;
use super::prompt::stream_to_output;
use async_openai::config::OpenAIConfig;

use llm_chain::options::Opt;
use llm_chain::options::Options;
use llm_chain::options::OptionsCascade;
use llm_chain::output::Output;
use llm_chain::tokens::TokenCollection;

use super::prompt::create_chat_completion_request;
use super::prompt::format_chat_messages;
use async_openai::error::OpenAIError;
use llm_chain::prompt::Prompt;

use llm_chain::tokens::PromptTokensError;
use llm_chain::tokens::{Tokenizer, TokenizerError};
use llm_chain::traits;
use llm_chain::traits::{ExecutorCreationError, ExecutorError};

use async_trait::async_trait;
use llm_chain::tokens::TokenCount;

use std::str::FromStr;
use std::sync::Arc;

/// The default MiniMax API base URL (OpenAI-compatible).
const MINIMAX_API_BASE: &str = "https://api.minimax.io/v1";

/// The `Executor` struct for MiniMax models. This executor uses the `async_openai`
/// crate with MiniMax's OpenAI-compatible API endpoint.
///
/// # Configuration
///
/// The executor reads the API key from:
/// 1. The `ApiKey` option passed to `new_with_options`
/// 2. The `MINIMAX_API_KEY` environment variable
///
/// The API base URL can be overridden via the `MINIMAX_API_BASE_URL` environment variable.
#[derive(Clone)]
pub struct Executor {
    client: Arc<async_openai::Client<OpenAIConfig>>,
    options: Options,
}

impl Executor {
    /// Creates a new `Executor` with the given client.
    pub fn for_client(client: async_openai::Client<OpenAIConfig>, options: Options) -> Self {
        use llm_chain::traits::Executor as _;
        let mut exec = Self::new_with_options(options).unwrap();
        exec.client = Arc::new(client);
        exec
    }

    fn get_model_from_invocation_options(&self, opts: &OptionsCascade) -> String {
        let Some(Opt::Model(model)) = opts.get(llm_chain::options::OptDiscriminants::Model) else {
            return Model::default().to_string();
        };
        model.to_name()
    }

    fn cascade<'a>(&'a self, opts: Option<&'a Options>) -> OptionsCascade<'a> {
        let mut v: Vec<&'a Options> = vec![&self.options];
        if let Some(o) = opts {
            v.push(o);
        }
        OptionsCascade::from_vec(v)
    }

    fn get_context_size(model_name: &str) -> usize {
        Model::from_str(model_name)
            .map(|m| m.context_size())
            .unwrap_or(128_000)
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    OpenAIError(#[from] OpenAIError),
}

#[async_trait]
impl traits::Executor for Executor {
    type StepTokenizer<'a> = MiniMaxTokenizer;

    /// Creates a new `Executor` configured for MiniMax's API.
    ///
    /// The API key is read from the `ApiKey` option or the `MINIMAX_API_KEY`
    /// environment variable. The base URL defaults to `https://api.minimax.io/v1`
    /// but can be overridden with `MINIMAX_API_BASE_URL`.
    fn new_with_options(options: Options) -> Result<Self, ExecutorCreationError> {
        let mut cfg = OpenAIConfig::new();

        let opts = OptionsCascade::new().with_options(&options);

        // Set API key from options or environment
        if let Some(Opt::ApiKey(api_key)) = opts.get(llm_chain::options::OptDiscriminants::ApiKey) {
            cfg = cfg.with_api_key(api_key);
        } else if let Ok(api_key) = std::env::var("MINIMAX_API_KEY") {
            cfg = cfg.with_api_key(api_key);
        }

        // Set base URL (MiniMax's OpenAI-compatible endpoint)
        let base_url = std::env::var("MINIMAX_API_BASE_URL")
            .unwrap_or_else(|_| MINIMAX_API_BASE.to_string());
        cfg = cfg.with_api_base(base_url);

        let client = Arc::new(async_openai::Client::with_config(cfg));
        Ok(Self { client, options })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let opts = self.cascade(Some(options));
        let client = self.client.clone();
        let model = self.get_model_from_invocation_options(&opts);
        let input = create_chat_completion_request(model, prompt, opts.is_streaming()).unwrap();
        if opts.is_streaming() {
            let res = async move { client.chat().create_stream(input).await }
                .await
                .map_err(|e| ExecutorError::InnerError(e.into()))?;
            Ok(stream_to_output(res))
        } else {
            let res = async move { client.chat().create(input).await }
                .await
                .map_err(|e| ExecutorError::InnerError(e.into()))?;
            Ok(completion_to_output(res))
        }
    }

    fn tokens_used(
        &self,
        opts: &Options,
        prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        let _opts_cas = self.cascade(Some(opts));

        // Use a simple byte-level estimation since tiktoken doesn't support MiniMax models.
        // A rough estimate is ~4 characters per token for English text.
        let messages = format_chat_messages(prompt.to_chat()).map_err(|e| match e {
            MiniMaxInnerError::StringTemplateError(e) => PromptTokensError::PromptFormatFailed(e),
            _ => PromptTokensError::UnableToCompute,
        })?;

        let mut total_chars = 0usize;
        for msg in &messages {
            let content = match msg {
                async_openai::types::ChatCompletionRequestMessage::System(m) => {
                    m.content.clone().unwrap_or_default()
                }
                async_openai::types::ChatCompletionRequestMessage::User(m) => m
                    .content
                    .as_ref()
                    .and_then(|c| match c {
                        async_openai::types::ChatCompletionRequestUserMessageContent::Text(t) => {
                            Some(t.to_string())
                        }
                        _ => None,
                    })
                    .unwrap_or_default(),
                async_openai::types::ChatCompletionRequestMessage::Assistant(m) => {
                    m.content.clone().unwrap_or_default()
                }
                async_openai::types::ChatCompletionRequestMessage::Tool(m) => {
                    m.content.clone().unwrap_or_default()
                }
                async_openai::types::ChatCompletionRequestMessage::Function(m) => {
                    m.content.clone().unwrap_or_default()
                }
            };
            // ~4 chars per token + overhead per message
            total_chars += content.len() + 4;
        }
        let tokens_used = (total_chars / 4) as i32;

        Ok(TokenCount::new(
            self.max_tokens_allowed(opts),
            tokens_used,
        ))
    }

    fn max_tokens_allowed(&self, opts: &Options) -> i32 {
        let opts_cas = self.cascade(Some(opts));
        let model = self.get_model_from_invocation_options(&opts_cas);
        Self::get_context_size(&model)
            .try_into()
            .unwrap_or(128_000)
    }

    fn answer_prefix(&self, _prompt: &Prompt) -> Option<String> {
        None
    }

    fn get_tokenizer(&self, _options: &Options) -> Result<MiniMaxTokenizer, TokenizerError> {
        Ok(MiniMaxTokenizer)
    }
}

/// A simple byte-level tokenizer for MiniMax models.
///
/// Since MiniMax doesn't provide a public tokenizer, this uses a simple
/// character-based approach that provides reasonable estimates for token
/// counting and text splitting.
pub struct MiniMaxTokenizer;

impl Tokenizer for MiniMaxTokenizer {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        // Simple byte-level tokenization (similar to mock tokenizer)
        let tokens: Vec<i32> = doc.as_bytes().iter().map(|&b| b as i32).collect();
        Ok(tokens.into())
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        let bytes: Vec<u8> = tokens
            .as_i32()
            .map_err(|_| TokenizerError::ToStringError)?
            .into_iter()
            .map(|c| c as u8)
            .collect();
        String::from_utf8(bytes).map_err(|_| TokenizerError::ToStringError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_chain::traits::Executor;

    #[test]
    fn test_default_model() {
        let exec =
            <super::Executor as Executor>::new_with_options(Options::empty().clone()).unwrap();
        let opts = exec.cascade(None);
        let model = exec.get_model_from_invocation_options(&opts);
        assert_eq!(model, "MiniMax-M2.7");
    }

    #[test]
    fn test_custom_model() {
        let mut builder = llm_chain::options::Options::builder();
        builder.add_option(Opt::Model(llm_chain::options::ModelRef::from_model_name(
            "MiniMax-M2.5-highspeed",
        )));
        let options = builder.build();
        let exec = <super::Executor as Executor>::new_with_options(options).unwrap();
        let opts = exec.cascade(None);
        let model = exec.get_model_from_invocation_options(&opts);
        assert_eq!(model, "MiniMax-M2.5-highspeed");
    }

    #[test]
    fn test_context_size() {
        assert_eq!(super::Executor::get_context_size("MiniMax-M2.7"), 1_000_000);
        assert_eq!(
            super::Executor::get_context_size("MiniMax-M2.5-highspeed"),
            204_000
        );
        assert_eq!(super::Executor::get_context_size("unknown"), 128_000);
    }

    #[test]
    fn test_tokenizer() {
        let tokenizer = MiniMaxTokenizer;
        let tokens = tokenizer.tokenize_str("Hello").unwrap();
        assert_eq!(tokens.len(), 5);
        let text = tokenizer.to_string(tokens).unwrap();
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_tokenizer_utf8() {
        let tokenizer = MiniMaxTokenizer;
        let tokens = tokenizer.tokenize_str("Héllo").unwrap();
        // é is 2 bytes in UTF-8
        assert_eq!(tokens.len(), 6);
        let text = tokenizer.to_string(tokens).unwrap();
        assert_eq!(text, "Héllo");
    }

    #[test]
    fn test_max_tokens_allowed() {
        let exec =
            <super::Executor as Executor>::new_with_options(Options::empty().clone()).unwrap();
        let max = exec.max_tokens_allowed(Options::empty());
        assert_eq!(max, 1_000_000);
    }

    #[test]
    fn test_answer_prefix_is_none() {
        let exec =
            <super::Executor as Executor>::new_with_options(Options::empty().clone()).unwrap();
        let prompt = Prompt::text("test".to_string());
        assert!(exec.answer_prefix(&prompt).is_none());
    }
}
