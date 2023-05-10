use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::tools::{Describe, Tool, ToolDescription, ToolError};

pub struct BingSearch {
    api_key: String,
}

impl BingSearch {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BingSearchInput {
    pub query: String,
}

impl From<&str> for BingSearchInput {
    fn from(value: &str) -> Self {
        Self {
            query: value.into(),
        }
    }
}

impl From<String> for BingSearchInput {
    fn from(value: String) -> Self {
        Self { query: value }
    }
}

impl Describe for BingSearchInput {
    fn describe() -> crate::tools::Format {
        vec![("query", "Search query to find necessary information").into()].into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct BingSearchOutput {
    pub result: String,
}

impl From<String> for BingSearchOutput {
    fn from(value: String) -> Self {
        Self { result: value }
    }
}

impl From<BingSearchOutput> for String {
    fn from(val: BingSearchOutput) -> Self {
        val.result
    }
}

impl Describe for BingSearchOutput {
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
struct BingWebPage {
    snippet: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BingWebPages {
    value: Vec<BingWebPage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BingSearchResult {
    #[serde(rename = "webPages")]
    web_pages: BingWebPages,
}

#[derive(Debug, Error)]
pub enum BingSearchError {
    #[error("No search results were returned")]
    NoResults,
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
}

impl ToolError for BingSearchError {}

#[async_trait]
impl Tool for BingSearch {
    type Input = BingSearchInput;

    type Output = BingSearchOutput;

    type Error = BingSearchError;

    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let client = reqwest::Client::new();
        let response = client
            .request(Method::GET, "https://api.bing.microsoft.com/v7.0/search")
            .query(&[("q", &input.query)])
            .header("Ocp-Apim-Subscription-Key", self.api_key.clone())
            .send()
            .await?
            .json::<BingSearchResult>()
            .await?;
        let answer = response
            .web_pages
            .value
            .first()
            .ok_or(BingSearchError::NoResults)?
            .snippet
            .clone();
        Ok(answer.into())
    }

    fn description(&self) -> ToolDescription {
        ToolDescription::new(
            "Bing search",
            "Useful for when you need to answer questions about current events. Input should be a search query.",
            "Use this to get information about current events.",
            BingSearchInput::describe(),
            BingSearchOutput::describe(),
        )
    }
}
