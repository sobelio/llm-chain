use llm_chain::options::{ModelRef, Opt};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use aws_sdk_sagemakerruntime::primitives::Blob;
use llm_chain::prompt::Prompt;
use aws_sdk_sagemakerruntime::operation::invoke_endpoint::InvokeEndpointOutput;

/// The `Model` enum represents the available SageMaker Endpoint models.
///
/// # Example
///
/// ```
/// use llm_chain_sagemaker_endpoint::model::Model;
///
/// let falcon_model = Model::Falcon7BInstruct;
/// let custom_model = Model::Other("your_custom_model_name".to_string());
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[non_exhaustive]
pub enum Model {
    #[default]
    #[strum(
        serialize = "falcon-7b-instruct",
        serialize = "falcon-7b"
    )]
    Falcon7BInstruct,

    /// A variant that allows you to specify a custom model name as a string, in case new models
    /// are introduced or you have access to specialized models.
    #[strum(default)]
    Other(String),
}

pub trait Formatter {
    fn format_request(&self, prompt: &Prompt) -> Blob;
    fn parse_response(&self, response: InvokeEndpointOutput) -> String;
}

impl Formatter for Model {
    fn format_request(&self, prompt: &Prompt) -> Blob {
        match self {
            Model::Falcon7BInstruct => {
                let body_string = format!("{{\"inputs\": \"{}\"}}", prompt);
                let body_blob = Blob::new(body_string.as_bytes().to_vec());
                body_blob
            }
            _ => {
                // TODO: allow user to pass custom parsers
                unimplemented!();
            }
        }
    }
    fn parse_response(&self, response: InvokeEndpointOutput) -> String {
        match self {
            Model::Falcon7BInstruct => {
                let output = String::from_utf8(response.body.unwrap().into_inner()).unwrap();
                let output_json: serde_json::Value = serde_json::from_str(&output).unwrap();
                let generated_text = output_json[0]["generated_text"].to_string();
                generated_text
            }
            _ => {
                // TODO: allow user to pass custom parsers
                unimplemented!();
            }
        }
    }
}

/// The `Model` enum implements the `ToString` trait, allowing you to easily convert it to a string.
impl ToString for Model {
    fn to_string(&self) -> String {
        match &self {
            Model::Falcon7BInstruct => "falcon-7b".to_string(),
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