//! Scripted-model tests: exercise the normal agent constructor, parser, tool
//! dispatcher and next-model scratchpad without requiring model credentials.
use super::*;
use crate::{
    output::Output,
    prompt::Prompt,
    tokens::{TokenCollection, Tokenizer, TokenizerError},
    tools::{
        tools::{
            GoogleSerper, GoogleSerperError, GoogleSerperInput, GoogleSerperOutput,
            ParallelSearchError, ParallelSearchInput, ParallelSearchOutput,
        },
        ToolContext, ToolDescription, ToolError,
    },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};
use thiserror::Error;

#[derive(Clone, Debug)]
struct Request {
    path: String,
    headers: HashMap<String, String>,
    body: Value,
}

struct Server {
    url: String,
    requests: Arc<Mutex<Vec<Request>>>,
    stop: Arc<AtomicBool>,
    worker: Option<thread::JoinHandle<()>>,
}

impl Server {
    fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/mcp", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let captured = Arc::clone(&requests);
        let stop = Arc::new(AtomicBool::new(false));
        let stopped = Arc::clone(&stop);
        let worker = thread::spawn(move || {
            for stream in listener.incoming() {
                if stopped.load(Ordering::Relaxed) {
                    break;
                }
                let captured = Arc::clone(&captured);
                thread::spawn(move || serve(stream.unwrap(), captured));
            }
        });
        Self {
            url,
            requests,
            stop,
            worker: Some(worker),
        }
    }

    fn agent(&self) -> Agent<ScriptedModel> {
        let mut agent = Agent::with_default_search(
            ScriptedModel,
            EarlyStoppingConfig {
                max_iterations: Some(5),
                max_time_elapsed_seconds: Some(10.0),
            },
        )
        .unwrap();
        // Resolve the production default through the real public constructor,
        // then route its HTTP requests to a deterministic local MCP server.
        agent.search_tool = agent.search_tool.with_test_endpoint(&self.url);
        agent
    }

    fn calls(&self) -> Vec<Request> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.body["method"] == "tools/call")
            .cloned()
            .collect()
    }

    fn assert_transport(&self) {
        let requests = self.requests.lock().unwrap();
        assert!(requests.iter().any(|r| r.body["method"] == "initialize"));
        assert!(requests
            .iter()
            .any(|r| r.body["method"] == "notifications/initialized"));
        assert!(requests.iter().any(|r| r.body["method"] == "tools/list"));
        for request in requests.iter() {
            assert_eq!(request.path, "/mcp");
            assert_eq!(
                request.headers["user-agent"],
                concat!("llm-chain/", env!("CARGO_PKG_VERSION"))
            );
            assert_eq!(
                request.headers["accept"],
                "application/json, text/event-stream"
            );
            assert!(!request.headers.contains_key("authorization"));
            if request.body["method"] != "initialize" {
                assert!(request.headers["mcp-session-id"].starts_with("transport-session-"));
                assert_eq!(request.headers["mcp-protocol-version"], "2025-03-26");
            }
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let address = self
            .url
            .trim_start_matches("http://")
            .trim_end_matches("/mcp");
        let _ = TcpStream::connect(address);
        self.worker.take().unwrap().join().unwrap();
    }
}

fn serve(mut stream: TcpStream, requests: Arc<Mutex<Vec<Request>>>) {
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let mut bytes = Vec::new();
    let header_end = loop {
        let mut chunk = [0; 4096];
        let n = stream.read(&mut chunk).unwrap_or(0);
        if n == 0 {
            return;
        }
        bytes.extend_from_slice(&chunk[..n]);
        if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
            break end + 4;
        }
    };
    let header = String::from_utf8(bytes[..header_end].to_vec()).unwrap();
    let mut lines = header.lines();
    let path = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .to_string();
    let headers: HashMap<_, _> = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(k, v)| (k.to_lowercase(), v.trim().to_owned()))
        .collect();
    let length: usize = headers["content-length"].parse().unwrap();
    while bytes.len() < header_end + length {
        let mut chunk = [0; 4096];
        let n = stream.read(&mut chunk).unwrap();
        if n == 0 {
            return;
        }
        bytes.extend_from_slice(&chunk[..n]);
    }
    let body: Value = serde_json::from_slice(&bytes[header_end..header_end + length]).unwrap();
    let transport_session = headers.get("mcp-session-id").cloned();
    requests.lock().unwrap().push(Request {
        path,
        headers,
        body: body.clone(),
    });
    let method = body["method"].as_str().unwrap();
    let query = body["params"]["arguments"]["objective"]
        .as_str()
        .unwrap_or("");
    if query == "timeout" || query == "cancel" {
        thread::sleep(Duration::from_millis(250));
    }
    if query == "expired-delayed" {
        thread::sleep(Duration::from_millis(180));
    }
    let result = match method {
        "initialize" => {
            json!({"protocolVersion": "2025-03-26", "capabilities": {"tools": {}}, "serverInfo": {"name": "fixture", "version": "1"}})
        }
        "tools/list" => json!({"tools": [{"name": "web_search"}, {"name": "web_fetch"}]}),
        "tools/call" if query == "quota" => {
            json!({"isError": true, "content": [{"type": "text", "text": "Anonymous quota exceeded"}]})
        }
        "tools/call" if query == "empty" => json!({"content": []}),
        "tools/call" => {
            json!({"content": [{"type": "text", "text": json!({"results": [{"url": format!("https://example.test/{query}"), "excerpts": [format!("evidence-for={query}"), "UTF-8: café"]}]}).to_string()}]})
        }
        _ => Value::Null,
    };
    let response = if query == "rpc-error" {
        json!({"jsonrpc": "2.0", "id": body["id"], "error": {"code": -32603, "message": "failed"}})
    } else if query == "wrong-id" {
        json!({"jsonrpc": "2.0", "id": 99999, "result": result})
    } else {
        json!({"jsonrpc": "2.0", "id": body["id"], "result": result})
    };
    let (status, content_type, text) = if method == "notifications/initialized" {
        ("202 Accepted", "application/json", String::new())
    } else if query.starts_with("expired")
        && transport_session.as_deref() == Some("transport-session-1")
    {
        (
            "404 Not Found",
            "application/json",
            "expired MCP session".into(),
        )
    } else if query == "http-error" {
        (
            "503 Service Unavailable",
            "application/json",
            "unavailable".into(),
        )
    } else if query == "bad-json" {
        ("200 OK", "application/json", "{broken".into())
    } else if query == "sse" {
        (
            "200 OK",
            "text/event-stream",
            format!(": keepalive\r\n\r\nevent: message\r\ndata: {response}\r\n\r\n"),
        )
    } else {
        ("200 OK", "application/json", response.to_string())
    };
    let session = if method == "initialize" {
        let generation = requests
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.body["method"] == "initialize")
            .count();
        format!("Mcp-Session-Id: transport-session-{generation}\r\n")
    } else {
        String::new()
    };
    let _ = write!(stream, "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\n{session}Content-Length: {}\r\nConnection: close\r\n\r\n{text}", text.len());
}

struct ScriptedModel;
struct UnusedTokenizer;

crate::multitool!(
    ContextTools,
    ContextInput,
    ContextOutput,
    ContextError,
    ParallelSearch,
    ParallelSearchInput,
    ParallelSearchOutput,
    ParallelSearchError,
    GoogleSerper,
    GoogleSerperInput,
    GoogleSerperOutput,
    GoogleSerperError
);

impl Tokenizer for UnusedTokenizer {
    fn tokenize_str(&self, _: &str) -> Result<TokenCollection, TokenizerError> {
        unreachable!()
    }
    fn to_string(&self, _: TokenCollection) -> Result<String, TokenizerError> {
        unreachable!()
    }
}

#[async_trait]
impl Executor for ScriptedModel {
    type StepTokenizer<'a> = UnusedTokenizer;
    fn new_with_options(_: Options) -> Result<Self, crate::traits::ExecutorCreationError> {
        Ok(Self)
    }
    async fn execute(&self, _: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let text = prompt.to_text();
        if text.contains("no-search-needed") {
            return Ok(Output::new_immediate(Prompt::text(
                "So the final answer is: already known".to_owned(),
            )));
        }
        let tag = if text.contains("run-beta") {
            "run-beta"
        } else {
            "run-alpha"
        };
        let response = if !text.contains(&format!("evidence-for={tag}-first")) {
            format!("Follow up: {tag}-first")
        } else if !text.contains(&format!("evidence-for={tag}-second")) {
            assert!(text.contains(&format!("https://example.test/{tag}-first")));
            assert!(text.contains("café"));
            format!("Follow up: {tag}-second")
        } else {
            assert!(text.contains(&format!("https://example.test/{tag}-second")));
            format!("So the final answer is: {tag} complete")
        };
        Ok(Output::new_immediate(Prompt::text(response.to_string())))
    }
    fn tokens_used(
        &self,
        _: &Options,
        _: &Prompt,
    ) -> Result<crate::tokens::TokenCount, crate::tokens::PromptTokensError> {
        unreachable!()
    }
    fn max_tokens_allowed(&self, _: &Options) -> i32 {
        unreachable!()
    }
    fn get_tokenizer(&self, _: &Options) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
        unreachable!()
    }
    fn answer_prefix(&self, _: &Prompt) -> Option<String> {
        None
    }
}

#[tokio::test]
async fn multitool_dispatch_preserves_the_supplied_context() {
    let server = Server::new();
    let tools: ContextTools = ParallelSearch::new()
        .unwrap()
        .with_test_endpoint(&server.url)
        .into();
    let context = ToolContext::default();
    for query in ["wrapped-first", "wrapped-followup"] {
        let input: ContextInput = ParallelSearchInput::from(query).into();
        tools
            .invoke_typed_with_context(&input, &context)
            .await
            .unwrap();
    }
    assert_eq!(server.calls().len(), 2);
    for call in server.calls() {
        assert_eq!(
            call.body["params"]["arguments"]["session_id"],
            context.session_id().to_string()
        );
    }
    server.assert_transport();
}

#[tokio::test]
async fn agent_does_not_connect_when_the_model_finishes_without_search() {
    let server = Server::new();
    let agent = server.agent();
    let (_, steps) = agent.run("no-search-needed").await.unwrap();
    assert!(steps.is_empty());
    assert!(server.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn default_agent_consumes_search_results_and_separates_repeated_runs() {
    let server = Server::new();
    let agent = server.agent();
    for _ in 0..2 {
        let (finish, steps) = agent.run("run-alpha").await.unwrap();
        assert_eq!(
            finish.return_values.get("output").unwrap().as_str(),
            "run-alpha complete"
        );
        assert_eq!(steps.len(), 2);
    }
    let calls = server.calls();
    assert_eq!(calls.len(), 4);
    let ids: Vec<_> = calls
        .iter()
        .map(|r| {
            r.body["params"]["arguments"]["session_id"]
                .as_str()
                .unwrap()
        })
        .collect();
    assert_eq!(ids[0], ids[1]);
    assert_eq!(ids[2], ids[3]);
    assert_ne!(ids[0], ids[2]);
    assert!(uuid::Uuid::parse_str(ids[0]).is_ok());
    assert_eq!(
        server
            .requests
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.body["method"] == "initialize")
            .count(),
        1
    );
    server.assert_transport();
}

#[tokio::test]
async fn concurrent_runs_keep_contexts_with_one_cached_client() {
    let server = Server::new();
    let agent = server.agent();
    let (a, b) = futures::join!(agent.run("run-alpha"), agent.run("run-beta"));
    assert!(a.is_ok() && b.is_ok());
    let calls = server.calls();
    let mut ids = HashMap::new();
    for call in calls {
        let args = &call.body["params"]["arguments"];
        let query = args["objective"].as_str().unwrap();
        let tag = if query.contains("alpha") {
            "alpha"
        } else {
            "beta"
        };
        let id = args["session_id"].as_str().unwrap().to_owned();
        if let Some(previous) = ids.insert(tag, id.clone()) {
            assert_eq!(previous, id);
        }
        assert_eq!(args["search_queries"], json!([query]));
    }
    assert_ne!(ids["alpha"], ids["beta"]);
    server.assert_transport();
}

#[tokio::test]
async fn search_fetch_followup_and_quota_retry_keep_context() {
    let server = Server::new();
    let tool = ParallelSearch::new()
        .unwrap()
        .with_test_endpoint(&server.url);
    let context = ToolContext::default();
    tool.invoke_typed_with_context(&"first".into(), &context)
        .await
        .unwrap();
    tool.fetch(&["https://example.test/page?x=1&y=two".into()], &context)
        .await
        .unwrap();
    tool.invoke_typed_with_context(&"sse".into(), &context)
        .await
        .unwrap();
    for _ in 0..2 {
        assert!(matches!(
            tool.invoke_typed_with_context(&"quota".into(), &context)
                .await,
            Err(ParallelSearchError::Tool(_))
        ));
    }
    let calls = server.calls();
    assert_eq!(calls.len(), 5);
    for call in &calls {
        assert_eq!(
            call.body["params"]["arguments"]["session_id"],
            context.session_id().to_string()
        );
    }
    assert_eq!(
        calls[1].body["params"]["arguments"]["urls"],
        json!(["https://example.test/page?x=1&y=two"])
    );
    tool.invoke_typed_with_context(&"new-conversation".into(), &ToolContext::default())
        .await
        .unwrap();
    assert_ne!(
        server.calls().last().unwrap().body["params"]["arguments"]["session_id"],
        context.session_id().to_string()
    );
    server.assert_transport();
}

#[tokio::test]
async fn expired_sessions_recover_on_explicit_retry_without_losing_context() {
    let server = Server::new();
    let tool = ParallelSearch::new()
        .unwrap()
        .with_test_endpoint(&server.url);
    let context = ToolContext::default();
    tool.invoke_typed_with_context(&"first".into(), &context)
        .await
        .unwrap();
    let delayed_input = "expired-delayed".into();
    let old_request = tool.invoke_typed_with_context(&delayed_input, &context);
    let recover = async {
        tokio::time::timeout(Duration::from_secs(2), async {
            while server.calls().len() < 2 {
                tokio::time::sleep(Duration::from_millis(2)).await;
            }
        })
        .await
        .unwrap();
        assert!(
            matches!(tool.invoke_typed_with_context(&"expired".into(), &context).await,
            Err(ParallelSearchError::Request(error)) if error.status() == Some(reqwest::StatusCode::NOT_FOUND))
        );
        // The explicit retry, using the same context, initializes a new transport.
        tool.invoke_typed_with_context(&"expired".into(), &context)
            .await
            .unwrap();
    };
    let (old, ()) = futures::join!(old_request, recover);
    assert!(
        matches!(old, Err(ParallelSearchError::Request(error)) if error.status() == Some(reqwest::StatusCode::NOT_FOUND))
    );
    // The late 404 for the first transport must not invalidate the new one.
    tool.invoke_typed_with_context(&"after-recovery".into(), &context)
        .await
        .unwrap();
    assert_eq!(server.calls().len(), 5);
    assert_eq!(
        server
            .requests
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.body["method"] == "initialize")
            .count(),
        2
    );
    for call in server.calls() {
        assert_eq!(
            call.body["params"]["arguments"]["session_id"],
            context.session_id().to_string()
        );
    }
    server.assert_transport();
}

#[tokio::test]
async fn protocol_http_tool_input_and_timeout_errors_are_visible() {
    let server = Server::new();
    let tool = ParallelSearch::new()
        .unwrap()
        .with_test_endpoint(&server.url);
    for query in ["rpc-error", "wrong-id", "empty"] {
        assert!(matches!(
            tool.invoke_typed(&query.into()).await,
            Err(ParallelSearchError::Protocol(_))
        ));
    }
    assert!(matches!(
        tool.invoke_typed(&"bad-json".into()).await,
        Err(ParallelSearchError::Json(_))
    ));
    assert!(matches!(
        tool.invoke_typed(&"http-error".into()).await,
        Err(ParallelSearchError::Request(_))
    ));
    let count = server.calls().len();
    assert!(matches!(
        tool.invoke_typed(&"  ".into()).await,
        Err(ParallelSearchError::InvalidInput(_))
    ));
    assert!(tool
        .fetch(&["file:///etc/passwd".into()], &ToolContext::default())
        .await
        .is_err());
    assert!(tool.fetch(&[], &ToolContext::default()).await.is_err());
    assert_eq!(server.calls().len(), count);
    let tool = tool.with_timeout(Duration::from_millis(40));
    assert!(
        matches!(tool.invoke_typed(&"timeout".into()).await, Err(ParallelSearchError::Request(e)) if e.is_timeout())
    );
}

#[tokio::test]
async fn cancelling_a_call_does_not_retry_or_poison_the_client() {
    let server = Server::new();
    let tool = ParallelSearch::new()
        .unwrap()
        .with_test_endpoint(&server.url);
    let context = ToolContext::default();
    // Initialize before cancelling a tool call, so cancellation is exercised
    // while an actual tools/call HTTP response is pending.
    tool.invoke_typed_with_context(&"first".into(), &context)
        .await
        .unwrap();
    assert!(tokio::time::timeout(
        Duration::from_millis(40),
        tool.invoke_typed_with_context(&"cancel".into(), &context)
    )
    .await
    .is_err());
    tool.invoke_typed_with_context(&"after-cancel".into(), &context)
        .await
        .unwrap();
    assert_eq!(server.calls().len(), 3);
    for call in server.calls() {
        assert_eq!(
            call.body["params"]["arguments"]["session_id"],
            context.session_id().to_string()
        );
    }
}

#[tokio::test]
async fn explicit_tool_selection_is_preserved_without_network_fallback() {
    // The pre-existing Tool implementation only supplies invoke_typed, proving
    // the new context hook remains compatible with existing tools.
    struct ExplicitTool;
    #[async_trait]
    impl Tool for ExplicitTool {
        type Input = String;
        type Output = String;
        type Error = ParallelSearchError;
        async fn invoke_typed(&self, input: &String) -> Result<String, Self::Error> {
            if input.contains("first") {
                Ok(format!(
                    "evidence-for={input} https://example.test/{input} café"
                ))
            } else {
                Err(ParallelSearchError::Tool("explicit tool failed".into()))
            }
        }
        fn description(&self) -> crate::tools::ToolDescription {
            unreachable!()
        }
    }
    let agent = Agent::new(ScriptedModel, ExplicitTool, EarlyStoppingConfig::default());
    assert!(
        matches!(agent.run("run-alpha").await, Err(SelfAskWithSearchAgentError::SearchToolError(ParallelSearchError::Tool(message))) if message == "explicit tool failed")
    );
}

#[tokio::test]
#[ignore = "makes an anonymous live request to Parallel Search MCP"]
async fn live_default_agent_uses_anonymous_search_and_consumes_evidence() {
    // A scripted model drives the production dispatcher; no LLM call is claimed.
    struct LiveModel;
    #[async_trait]
    impl Executor for LiveModel {
        type StepTokenizer<'a> = UnusedTokenizer;
        fn new_with_options(_: Options) -> Result<Self, crate::traits::ExecutorCreationError> {
            Ok(Self)
        }
        async fn execute(&self, _: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
            let scratchpad = prompt.to_text();
            let response = if scratchpad.contains("https://www.rust-lang.org")
                || scratchpad.contains("https://rust-lang.org")
            {
                "So the final answer is: Rust evidence received"
            } else {
                "Follow up: Rust programming language official website"
            };
            Ok(Output::new_immediate(Prompt::text(response.to_string())))
        }
        fn tokens_used(
            &self,
            _: &Options,
            _: &Prompt,
        ) -> Result<crate::tokens::TokenCount, crate::tokens::PromptTokensError> {
            unreachable!()
        }
        fn max_tokens_allowed(&self, _: &Options) -> i32 {
            unreachable!()
        }
        fn get_tokenizer(&self, _: &Options) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
            unreachable!()
        }
        fn answer_prefix(&self, _: &Prompt) -> Option<String> {
            None
        }
    }
    let agent = Agent::with_default_search(
        LiveModel,
        EarlyStoppingConfig {
            max_iterations: Some(2),
            max_time_elapsed_seconds: Some(90.0),
        },
    )
    .unwrap();
    let (finish, steps) = agent
        .run("Find the official Rust programming language website.")
        .await
        .unwrap();
    assert_eq!(
        finish.return_values.get("output").unwrap().as_str(),
        "Rust evidence received"
    );
    assert_eq!(steps.len(), 1);
    println!(
        "Live anonymous result: {}",
        steps[0].observation.as_str().unwrap()
    );
}
