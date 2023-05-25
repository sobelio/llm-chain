use llm_chain::options::{ModelRef, Opt};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// The `Model` enum represents the available ChatGPT models that you can use through the OpenAI
/// API.
///
/// These models have different capabilities and performance characteristics, allowing you to choose
/// the one that best suits your needs. See <https://platform.openai.com/docs/models> for more
/// information.
///
/// # Example
///
/// ```
/// use llm_chain_openai::chatgpt::Model;
///
/// let turbo_model = Model::Gpt35Turbo;
/// let custom_model = Model::Other("your_custom_model_name".to_string());
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[non_exhaustive]
pub enum Model {
    /// A high-performance and versatile model that offers a great balance of speed, quality, and
    ///   affordability.
    #[default]
    #[strum(
        serialize = "gpt-3.5-turbo",
        serialize = "gpt-35-turbo",
        serialize = "gpt3.5",
        serialize = "gpt35"
    )]
    Gpt35Turbo,

    /// Snapshot of gpt-3.5-turbo from March 1st 2023. Unlike gpt-3.5-turbo, this model will not
    /// receive updates, and will be deprecated 3 months after a new version is released.
    #[strum(serialize = "gpt-3.5-turbo-0301")]
    Gpt35Turbo0301,

    /// A high-performance model that offers the best quality, but is slower and more expensive than
    /// the `ChatGPT3_5Turbo` model.
    #[strum(serialize = "gpt-4", serialize = "gpt4")]
    Gpt4,

    /// Snapshot of gpt-4 from March 14th 2023. Unlike gpt-4, this model will not receive updates,
    /// and will be deprecated 3 months after a new version is released.
    #[strum(serialize = "gpt-4-0314")]
    Gpt4_0314,

    /// Same capabilities as the base gpt-4 mode but with 4x the context length. Will be updated
    /// with our latest model iteration.
    #[strum(serialize = "gpt-4-32k")]
    Gpt4_32k,

    /// Snapshot of gpt-4-32 from March 14th 2023. Unlike gpt-4-32k, this model will not receive
    /// updates, and will be deprecated 3 months after a new version is released.
    #[strum(serialize = "gpt-4-32k-0314")]
    Gpt4_32k0314,

    /// A variant that allows you to specify a custom model name as a string, in case new models
    /// are introduced or you have access to specialized models.
    #[strum(default)]
    Other(String),
}

impl Model {
    /// included for backwards compatibility
    #[deprecated(note = "Use `Model::Gpt35Turbo` instead")]
    #[allow(non_upper_case_globals)]
    pub const ChatGPT3_5Turbo: Model = Model::Gpt35Turbo;
    /// included for backwards compatibility
    #[deprecated(note = "Use `Model::Gpt4` instead")]
    pub const GPT4: Model = Model::Gpt4;
}

/// The `Model` enum implements the `ToString` trait, allowing you to easily convert it to a string.
impl ToString for Model {
    fn to_string(&self) -> String {
        match &self {
            Model::Gpt35Turbo => "gpt-3.5-turbo".to_string(),
            Model::Gpt4 => "gpt-4".to_string(),
            Model::Gpt35Turbo0301 => "gpt-3.5-turbo-0301".to_string(),
            Model::Gpt4_0314 => "gpt-4-0314".to_string(),
            Model::Gpt4_32k => "gpt-4-32k".to_string(),
            Model::Gpt4_32k0314 => "gpt-4-32k-0314".to_string(),
            Model::Other(model) => model.to_string(),
        }
    }
}

/// Conversion from Model to ModelRef
impl From<Model> for ModelRef {
    fn from(value: Model) -> Self {
        ModelRef::from_model_name(value.to_string())
    }
}

/// Conversion from Model to Option
impl From<Model> for Opt {
    fn from(value: Model) -> Self {
        Opt::Model(value.into())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    // Tests for FromStr
    #[test]
    fn test_from_str() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Model::from_str("gpt-3.5-turbo")?, Model::Gpt35Turbo);
        assert_eq!(
            Model::from_str("gpt-3.5-turbo-0301")?,
            Model::Gpt35Turbo0301
        );
        assert_eq!(Model::from_str("gpt-4")?, Model::Gpt4);
        assert_eq!(Model::from_str("gpt-4-0314")?, Model::Gpt4_0314);
        assert_eq!(Model::from_str("gpt-4-32k")?, Model::Gpt4_32k);
        assert_eq!(Model::from_str("gpt-4-32k-0314")?, Model::Gpt4_32k0314);
        assert_eq!(
            Model::from_str("custom_model")?,
            Model::Other("custom_model".to_string())
        );
        Ok(())
    }

    // Test ToString
    #[test]
    fn test_to_string() {
        assert_eq!(Model::Gpt35Turbo.to_string(), "gpt-3.5-turbo");
        assert_eq!(Model::Gpt4.to_string(), "gpt-4");
        assert_eq!(Model::Gpt35Turbo0301.to_string(), "gpt-3.5-turbo-0301");
        assert_eq!(Model::Gpt4_0314.to_string(), "gpt-4-0314");
        assert_eq!(Model::Gpt4_32k.to_string(), "gpt-4-32k");
        assert_eq!(Model::Gpt4_32k0314.to_string(), "gpt-4-32k-0314");
        assert_eq!(
            Model::Other("custom_model".to_string()).to_string(),
            "custom_model"
        );
    }

    #[test]
    #[allow(deprecated)]
    fn test_to_string_deprecated() {
        assert_eq!(Model::ChatGPT3_5Turbo.to_string(), "gpt-3.5-turbo");
        assert_eq!(Model::GPT4.to_string(), "gpt-4");
    }
}
