//! Anonymous web search and page extraction via Parallel's HTTP MCP server.

use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use async_trait::async_trait;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;
use tokio::sync::Mutex;

use crate::tools::{Describe, Tool, ToolContext, ToolDescription, ToolError};

const ENDPOINT: &str = "https://search.parallel.ai/mcp";
const PROTOCOL: &str = "2025-03-26";
const MAX_RESPONSE_BYTES: usize = 4 * 1024 * 1024;

/// A reusable, keyless Search MCP client. Conversation identity belongs to
/// `ToolContext`, independently of this client's shared HTTP/MCP connection.
pub struct ParallelSearch {
    client: reqwest::Client,
    endpoint: String,
    timeout: Duration,
    connection: Mutex<Option<Connection>>,
    next_id: AtomicU64,
}

#[derive(Clone)]
struct Connection {
    session: Option<HeaderValue>,
    generation: u64,
}

impl ParallelSearch {
    /// Connect anonymously to Parallel Search MCP. Search uses Fast mode.
    /// Credentials from environment variables are never read by this client.
    pub fn new() -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: reqwest::Client::builder()
                .user_agent(concat!("llm-chain/", env!("CARGO_PKG_VERSION")))
                .redirect(reqwest::redirect::Policy::none())
                .build()?,
            endpoint: ENDPOINT.to_owned(),
            timeout: Duration::from_secs(30),
            connection: Mutex::new(None),
            next_id: AtomicU64::new(1),
        })
    }

    /// Set the timeout for each MCP HTTP request (30 seconds by default).
    /// Dropping an invocation future cancels its in-flight request.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_owned();
        self
    }

    fn id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    async fn post(
        &self,
        message: &Value,
        connection: Option<&Connection>,
    ) -> Result<reqwest::Response, ParallelSearchError> {
        let mut request = self
            .client
            .post(&self.endpoint)
            .timeout(self.timeout)
            .header("Accept", "application/json, text/event-stream")
            .json(message);
        if let Some(connection) = connection {
            request = request.header("MCP-Protocol-Version", PROTOCOL);
            if let Some(session) = &connection.session {
                request = request.header("Mcp-Session-Id", session);
            }
        }
        Ok(request.send().await?.error_for_status()?)
    }

    async fn response(
        mut response: reqwest::Response,
        id: u64,
    ) -> Result<Value, ParallelSearchError> {
        let sse = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.split(';').next().unwrap_or_default().trim() == "text/event-stream")
            .unwrap_or(false);
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            if bytes.len() + chunk.len() > MAX_RESPONSE_BYTES {
                return Err(ParallelSearchError::Protocol(
                    "MCP response exceeds 4 MiB".into(),
                ));
            }
            bytes.extend_from_slice(&chunk);
            // Streamable HTTP can leave an SSE stream open after its response.
            if sse {
                if let Some(value) = sse_response(&bytes, id)? {
                    return rpc_result(value, id);
                }
            }
        }
        if sse {
            Err(ParallelSearchError::Protocol(
                "MCP stream ended without a response".into(),
            ))
        } else {
            rpc_result(serde_json::from_slice(&bytes)?, id)
        }
    }

    async fn connection(&self) -> Result<Connection, ParallelSearchError> {
        let mut cached = self.connection.lock().await;
        if let Some(connection) = &*cached {
            return Ok(connection.clone());
        }
        let id = self.id();
        let response = self
            .post(
                &json!({
                    "jsonrpc": "2.0", "id": id, "method": "initialize",
                    "params": {"protocolVersion": PROTOCOL, "capabilities": {},
                        "clientInfo": {"name": "llm-chain", "version": env!("CARGO_PKG_VERSION")}}
                }),
                None,
            )
            .await?;
        let connection = Connection {
            session: response.headers().get("Mcp-Session-Id").cloned(),
            generation: id,
        };
        let initialized = Self::response(response, id).await?;
        if initialized["protocolVersion"] != PROTOCOL {
            return Err(ParallelSearchError::Protocol(
                "Unsupported MCP protocol version".into(),
            ));
        }
        self.post(
            &json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
            Some(&connection),
        )
        .await?;
        let id = self.id();
        let response = self
            .post(
                &json!({"jsonrpc": "2.0", "id": id, "method": "tools/list"}),
                Some(&connection),
            )
            .await?;
        let tools = Self::response(response, id).await?;
        for name in ["web_search", "web_fetch"] {
            if !tools["tools"]
                .as_array()
                .map(|tools| tools.iter().any(|tool| tool["name"] == name))
                .unwrap_or(false)
            {
                return Err(ParallelSearchError::Protocol(format!(
                    "MCP server does not offer {name}"
                )));
            }
        }
        *cached = Some(connection.clone());
        Ok(connection)
    }

    async fn call(
        &self,
        name: &str,
        mut arguments: Value,
        context: &ToolContext,
    ) -> Result<ParallelSearchOutput, ParallelSearchError> {
        arguments["session_id"] = context.session_id().to_string().into();
        let connection = self.connection().await?;
        let id = self.id();
        let response = self
            .post(
                &json!({"jsonrpc": "2.0", "id": id, "method": "tools/call",
            "params": {"name": name, "arguments": arguments}}),
                Some(&connection),
            )
            .await;
        let response = match response {
            Err(ParallelSearchError::Request(error))
                if error.status() == Some(reqwest::StatusCode::NOT_FOUND) =>
            {
                // MCP sessions can expire. Return the error without replaying;
                // the caller can retry with its existing conversation context.
                // An old concurrent response must not discard a newer session.
                let mut cached = self.connection.lock().await;
                if cached.as_ref().map(|c| c.generation) == Some(connection.generation) {
                    *cached = None;
                }
                return Err(ParallelSearchError::Request(error));
            }
            result => result?,
        };
        let result = Self::response(response, id).await?;
        let text = result["content"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|part| part["type"] == "text")
            .filter_map(|part| part["text"].as_str())
            .collect::<Vec<_>>()
            .join("\n");
        if result["isError"].as_bool().unwrap_or(false) {
            return Err(ParallelSearchError::Tool(text));
        }
        if text.trim().is_empty() {
            return Err(ParallelSearchError::Protocol(
                "MCP tool returned no text".into(),
            ));
        }
        // Keep the server's text verbatim, including source URLs and excerpts.
        Ok(ParallelSearchOutput { result: text })
    }

    /// Extract pages in the same research context as related searches. The server
    /// accepts up to 20 HTTP(S) URLs; this method never fetches URLs directly.
    pub async fn fetch(
        &self,
        urls: &[String],
        context: &ToolContext,
    ) -> Result<ParallelSearchOutput, ParallelSearchError> {
        if urls.is_empty()
            || urls.len() > 20
            || urls.iter().any(|url| {
                reqwest::Url::parse(url)
                    .map(|u| !matches!(u.scheme(), "http" | "https") || u.host_str().is_none())
                    .unwrap_or(true)
            })
        {
            return Err(ParallelSearchError::InvalidInput(
                "Provide 1 to 20 HTTP(S) URLs",
            ));
        }
        self.call("web_fetch", json!({"urls": urls}), context).await
    }
}

fn rpc_result(value: Value, id: u64) -> Result<Value, ParallelSearchError> {
    if value["jsonrpc"] != "2.0" || value["id"].as_u64() != Some(id) {
        return Err(ParallelSearchError::Protocol(
            "Unexpected MCP response identity".into(),
        ));
    }
    if let Some(error) = value.get("error") {
        return Err(ParallelSearchError::Protocol(error.to_string()));
    }
    value
        .get("result")
        .cloned()
        .ok_or_else(|| ParallelSearchError::Protocol("Missing MCP result".into()))
}

fn sse_response(bytes: &[u8], id: u64) -> Result<Option<Value>, ParallelSearchError> {
    let text = String::from_utf8_lossy(bytes).replace("\r\n", "\n");
    // The final partial event is incomplete until its blank line arrives.
    let complete = match text.rfind("\n\n") {
        Some(end) => end,
        None => return Ok(None),
    };
    for event in text[..complete].split("\n\n") {
        let data = event
            .lines()
            .filter_map(|line| {
                line.strip_prefix("data:")
                    .map(|s| s.strip_prefix(' ').unwrap_or(s))
            })
            .collect::<Vec<_>>()
            .join("\n");
        if !data.is_empty() {
            let value: Value = serde_json::from_str(&data)?;
            if value["id"].as_u64() == Some(id) {
                return Ok(Some(value));
            }
        }
    }
    Ok(None)
}

#[derive(Serialize, Deserialize)]
pub struct ParallelSearchInput {
    pub query: String,
}

impl From<String> for ParallelSearchInput {
    fn from(query: String) -> Self {
        Self { query }
    }
}

impl From<&str> for ParallelSearchInput {
    fn from(query: &str) -> Self {
        query.to_owned().into()
    }
}

impl Describe for ParallelSearchInput {
    fn describe() -> crate::tools::Format {
        vec![("query", "Web search question or query").into()].into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ParallelSearchOutput {
    pub result: String,
}

impl From<ParallelSearchOutput> for String {
    fn from(output: ParallelSearchOutput) -> Self {
        output.result
    }
}

impl Describe for ParallelSearchOutput {
    fn describe() -> crate::tools::Format {
        vec![("result", "Web evidence with source URLs and excerpts").into()].into()
    }
}

#[derive(Debug, Error)]
pub enum ParallelSearchError {
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("MCP protocol error: {0}")]
    Protocol(String),
    #[error("Parallel tool error: {0}")]
    Tool(String),
    #[error("Invalid search input: {0}")]
    InvalidInput(&'static str),
}

impl ToolError for ParallelSearchError {}

#[async_trait]
impl Tool for ParallelSearch {
    type Input = ParallelSearchInput;
    type Output = ParallelSearchOutput;
    type Error = ParallelSearchError;

    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        self.invoke_typed_with_context(input, &ToolContext::default())
            .await
    }

    fn invoke_typed_with_context<'a>(
        &'a self,
        input: &'a Self::Input,
        context: &'a ToolContext,
    ) -> futures::future::BoxFuture<'a, Result<Self::Output, Self::Error>> {
        Box::pin(async move {
            if input.query.trim().is_empty() {
                return Err(ParallelSearchError::InvalidInput(
                    "Search query must not be empty",
                ));
            }
            self.call(
                "web_search",
                json!({"objective": input.query, "search_queries": [input.query]}),
                context,
            )
            .await
        })
    }

    fn description(&self) -> ToolDescription {
        ToolDescription::new(
            "Parallel search",
            "Search the web for factual and current information, without an API key.",
            "Use excerpts and source URLs to answer the question.",
            ParallelSearchInput::describe(),
            ParallelSearchOutput::describe(),
        )
    }
}
