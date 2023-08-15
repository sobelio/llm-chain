use aws_sdk_sagemakerruntime::operation::invoke_endpoint::InvokeEndpointOutput;
use aws_sdk_sagemakerruntime::primitives::Blob;
use llm_chain::options::{ModelRef, Opt, OptDiscriminants, OptionsCascade};
use llm_chain::prompt::Prompt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;
use strum_macros::EnumString;

/// The `Model` enum represents the available SageMaker Endpoint models.
/// Use SageMaker JumpStart to deploy the model listed here. Or use Model::Other
/// to reference your custom models. For Model::Other, you need to write your own
/// formatting logic for the request and response.
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
    #[strum(serialize = "falcon-7b-instruct")]
    Falcon7BInstruct,

    #[strum(serialize = "falcon-40b-instruct")]
    Falcon40BInstruct,

    /// A variant that allows you to specify a custom model name as a string, in case new models
    /// are introduced or you have access to specialized models.
    #[strum(default)]
    Other(String),
}

pub trait Formatter {
    fn format_request(&self, prompt: &Prompt, options: &OptionsCascade) -> Blob;
    fn request_content_type(&self) -> String;
    fn parse_response(&self, response: InvokeEndpointOutput) -> String;
}

impl Formatter for Model {
    fn format_request(&self, prompt: &Prompt, options: &OptionsCascade) -> Blob {
        match self {
            Model::Falcon7BInstruct | Model::Falcon40BInstruct => {
                #[skip_serializing_none]
                #[derive(Serialize)]
                struct Parameters {
                    max_new_tokens: Option<usize>,
                    max_length: Option<usize>,
                    temperature: Option<f32>,
                    top_k: Option<i32>,
                    top_p: Option<f32>,
                    stop: Option<Vec<String>>,
                    // TODO: num_beams, no_repeat_ngram_size, early_stopping, do_sample, return_full_text
                }

                let parameters = Parameters {
                    max_new_tokens: options.get(OptDiscriminants::MaxTokens).map(|x| match x {
                        Opt::MaxTokens(i) => *i,
                        _ => unreachable!("options.get should restrict the enum variant."),
                    }),
                    max_length: options.get(OptDiscriminants::MaxContextSize).map(|x| match x {
                        Opt::MaxContextSize(i) => *i,
                        _ => unreachable!("options.get should restrict the enum variant."),
                    }),
                    temperature: options.get(OptDiscriminants::Temperature).map(|x| match x {
                        Opt::Temperature(i) => *i,
                        _ => unreachable!("options.get should restrict the enum variant."),
                    }),
                    top_k: options.get(OptDiscriminants::TopK).map(|x| match x {
                        Opt::TopK(i) => *i,
                        _ => unreachable!("options.get should restrict the enum variant."),
                    }),
                    top_p: options.get(OptDiscriminants::TopP).map(|x| match x {
                        Opt::TopP(i) => *i,
                        _ => unreachable!("options.get should restrict the enum variant."),
                    }),
                    stop: options.get(OptDiscriminants::StopSequence).map(|x| match x {
                        Opt::StopSequence(i) => i.clone(),
                        _ => unreachable!("options.get should restrict the enum variant."),
                    }),
                };

                let body_json = json!({
                    "inputs": prompt.to_string(),
                    "parameters": parameters
                });

                let body_string = body_json.to_string();
                let body_blob = Blob::new(body_string.as_bytes().to_vec());
                body_blob
            }
            _ => {
                unimplemented!("This model does not have a default formatter. Please format the request with your own code.");
            }
        }
    }

    fn request_content_type(&self) -> String {
        match self {
            Model::Falcon7BInstruct | Model::Falcon40BInstruct => "application/json".to_string(),
            _ => {
                unimplemented!("This model does not have a default formatter. Please format the request with your own code.");
            }
        }
    }

    fn parse_response(&self, response: InvokeEndpointOutput) -> String {
        match self {
            Model::Falcon7BInstruct => {
                let output = String::from_utf8(response.body.unwrap().into_inner()).unwrap();
                let output_json: serde_json::Value = serde_json::from_str(&output).unwrap();

                output_json[0]["generated_text"].to_string()
            }
            _ => {
                unimplemented!("This model does not have a default formatter. Please format the response with your own code.");
            }
        }
    }
}

impl Model {
    /// Convert the model to its SageMaker JumpStart default endpoint name
    pub fn to_jumpstart_endpoint_name(&self) -> String {
        match &self {
            Model::Falcon7BInstruct => "jumpstart-dft-hf-llm-falcon-7b-instruct-bf16".to_string(),
            Model::Falcon40BInstruct => "jumpstart-dft-hf-llm-falcon-40b-instruct-bf16".to_string(),
            _ => self.to_string(),
        }
    }
}

/// The `Model` enum implements the `ToString` trait, allowing you to easily convert it to a string.
impl ToString for Model {
    fn to_string(&self) -> String {
        match &self {
            Model::Falcon7BInstruct => "falcon-7b-instruct".to_string(),
            Model::Falcon40BInstruct => "falcon-40b-instruct".to_string(),
            //jumpstart-dft-hf-llm-falcon-7b-instruct-bf16
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
