use llm_chain::options::{ModelRef, Opt};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// The `Model` enum represents the available MiniMax models.
///
/// MiniMax provides high-performance language models through an
/// OpenAI-compatible API. The M2.7 family offers up to 1M token context,
/// while M2.5-highspeed provides 204K context with faster inference.
///
/// # Example
///
/// ```
/// use llm_chain_minimax::chatgpt::Model;
///
/// let default_model = Model::MiniMaxM27;
/// let highspeed = Model::MiniMaxM27Highspeed;
/// let custom = Model::Other("custom-model".to_string());
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[non_exhaustive]
pub enum Model {
    /// MiniMax-M2.7: The latest and most capable model with up to 1M token context.
    #[default]
    #[strum(serialize = "MiniMax-M2.7")]
    MiniMaxM27,

    /// MiniMax-M2.7-highspeed: Faster variant of M2.7 optimized for speed.
    #[strum(serialize = "MiniMax-M2.7-highspeed")]
    MiniMaxM27Highspeed,

    /// MiniMax-M2.5: Previous generation model with strong capabilities.
    #[strum(serialize = "MiniMax-M2.5")]
    MiniMaxM25,

    /// MiniMax-M2.5-highspeed: Fast variant with 204K token context window.
    #[strum(serialize = "MiniMax-M2.5-highspeed")]
    MiniMaxM25Highspeed,

    /// A variant that allows specifying a custom model name.
    #[strum(default)]
    Other(String),
}

impl Model {
    /// Returns the maximum context size for the model.
    pub fn context_size(&self) -> usize {
        match self {
            Model::MiniMaxM27 => 1_000_000,
            Model::MiniMaxM27Highspeed => 1_000_000,
            Model::MiniMaxM25 => 1_000_000,
            Model::MiniMaxM25Highspeed => 204_000,
            Model::Other(_) => 128_000, // conservative default
        }
    }
}

impl ToString for Model {
    fn to_string(&self) -> String {
        match self {
            Model::MiniMaxM27 => "MiniMax-M2.7".to_string(),
            Model::MiniMaxM27Highspeed => "MiniMax-M2.7-highspeed".to_string(),
            Model::MiniMaxM25 => "MiniMax-M2.5".to_string(),
            Model::MiniMaxM25Highspeed => "MiniMax-M2.5-highspeed".to_string(),
            Model::Other(model) => model.to_string(),
        }
    }
}

impl From<Model> for ModelRef {
    fn from(value: Model) -> Self {
        ModelRef::from_model_name(value.to_string())
    }
}

impl From<Model> for Opt {
    fn from(value: Model) -> Self {
        Opt::Model(value.into())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_str() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Model::from_str("MiniMax-M2.7")?, Model::MiniMaxM27);
        assert_eq!(
            Model::from_str("MiniMax-M2.7-highspeed")?,
            Model::MiniMaxM27Highspeed
        );
        assert_eq!(Model::from_str("MiniMax-M2.5")?, Model::MiniMaxM25);
        assert_eq!(
            Model::from_str("MiniMax-M2.5-highspeed")?,
            Model::MiniMaxM25Highspeed
        );
        assert_eq!(
            Model::from_str("custom-model")?,
            Model::Other("custom-model".to_string())
        );
        Ok(())
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Model::MiniMaxM27.to_string(), "MiniMax-M2.7");
        assert_eq!(
            Model::MiniMaxM27Highspeed.to_string(),
            "MiniMax-M2.7-highspeed"
        );
        assert_eq!(Model::MiniMaxM25.to_string(), "MiniMax-M2.5");
        assert_eq!(
            Model::MiniMaxM25Highspeed.to_string(),
            "MiniMax-M2.5-highspeed"
        );
        assert_eq!(
            Model::Other("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_default_model() {
        assert_eq!(Model::default(), Model::MiniMaxM27);
    }

    #[test]
    fn test_context_size() {
        assert_eq!(Model::MiniMaxM27.context_size(), 1_000_000);
        assert_eq!(Model::MiniMaxM27Highspeed.context_size(), 1_000_000);
        assert_eq!(Model::MiniMaxM25.context_size(), 1_000_000);
        assert_eq!(Model::MiniMaxM25Highspeed.context_size(), 204_000);
        assert_eq!(Model::Other("unknown".to_string()).context_size(), 128_000);
    }

    #[test]
    fn test_model_to_opt() {
        let opt: Opt = Model::MiniMaxM27.into();
        match opt {
            Opt::Model(model_ref) => assert_eq!(model_ref.to_name(), "MiniMax-M2.7"),
            _ => panic!("Expected Opt::Model"),
        }
    }
}
