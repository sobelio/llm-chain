use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::tools::{Describe, Tool, ToolDescription, ToolError};

pub struct GoogleSerper {
    api_key: String,
}

impl GoogleSerper {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GoogleSerperInput {
    pub query: String,
}

impl From<&str> for GoogleSerperInput {
    fn from(value: &str) -> Self {
        Self {
            query: value.into(),
        }
    }
}

impl From<String> for GoogleSerperInput {
    fn from(value: String) -> Self {
        Self { query: value }
    }
}

impl Describe for GoogleSerperInput {
    fn describe() -> crate::tools::Format {
        vec![("query", "Search query to find necessary information").into()].into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct GoogleSerperOutput {
    pub result: String,
}

impl From<String> for GoogleSerperOutput {
    fn from(value: String) -> Self {
        Self { result: value }
    }
}

impl From<GoogleSerperOutput> for String {
    fn from(val: GoogleSerperOutput) -> Self {
        val.result
    }
}

impl Describe for GoogleSerperOutput {
    fn describe() -> crate::tools::Format {
        vec![(
            "result",
            "Information retrieved from the internet that should answer your query",
        )
            .into()]
        .into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SiteLinks {
    title: String,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Organic {
    title: String,
    link: String,
    snippet: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleSerperResult {
    organic: Vec<Organic>,
}

#[derive(Debug, Error)]
pub enum GoogleSerperError {
    #[error("No search results were returned")]
    NoResults,
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
}

impl ToolError for GoogleSerperError {}

#[async_trait]
impl Tool for GoogleSerper {
    type Input = GoogleSerperInput;

    type Output = GoogleSerperOutput;

    type Error = GoogleSerperError;

    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let client = reqwest::Client::new();
        let response = client
            .request(Method::GET, "https://google.serper.dev/search")
            .query(&[("q", &input.query)])
            .header("X-API-KEY", self.api_key.clone())
            .send()
            .await?
            .json::<GoogleSerperResult>()
            .await?;
        let answer = response
            .organic
            .first()
            .ok_or(GoogleSerperError::NoResults)?
            .snippet
            .clone();
        Ok(answer.into())
    }

    fn description(&self) -> ToolDescription {
        ToolDescription::new(
            "Google search",
            "Useful for when you need to answer questions about current events. Input should be a search query.",
            "Use this to get information about current events.",
            GoogleSerperInput::describe(),
            GoogleSerperOutput::describe(),
        )
    }
}
