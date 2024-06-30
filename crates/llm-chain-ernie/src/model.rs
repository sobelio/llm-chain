use llm_chain::options::{ModelRef, Opt};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[non_exhaustive]
pub enum Model {
    /// Ernie 3.5 turbo
    #[default]
    #[strum(serialize = "eb-instant")]
    #[serde(rename = "eb-instant")]
    ErnieBotTurbo,
    /// ernie 3.5
    #[strum(serialize = "completions")]
    #[serde(rename = "completions")]
    ErnieBot,
    /// ernie 4.0
    #[strum(serialize = "completions_pro")]
    #[serde(rename = "completions_pro")]
    Ernie40,
    /// A variant that allows you to specify a custom model name as a string, in case new models
    /// are introduced or you have access to specialized models.
    #[strum(default)]
    Other(String),
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
    #[test]
    fn test_model_to_string() {
        use super::Model;
        assert_eq!(Model::ErnieBotTurbo.to_string(), "eb-instant");
        assert_eq!(Model::ErnieBot.to_string(), "completions");
        assert_eq!(Model::Ernie40.to_string(), "completions_pro");
        assert_eq!(
            Model::Other("your_custom_model_name".to_string()).to_string(),
            "your_custom_model_name"
        );
    }

    #[test]
    fn test_model_from_string() {
        use super::Model;
        use std::str::FromStr;
        assert_eq!(Model::from_str("eb-instant").unwrap(), Model::ErnieBotTurbo);
        assert_eq!(Model::from_str("completions").unwrap(), Model::ErnieBot);
        assert_eq!(Model::from_str("completions_pro").unwrap(), Model::Ernie40);
        assert_eq!(
            Model::from_str("your_custom_model_name").unwrap(),
            Model::Other("your_custom_model_name".to_string())
        );
    }
}
