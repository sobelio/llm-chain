# Web search with Parallel

The self-ask agent can use [Parallel Search MCP](https://docs.parallel.ai/integrations/mcp/search-mcp)
for free web search without a search-provider account or API key. Anonymous search
uses Fast mode. Search results retain the server's source URLs and excerpts so
the model can use the evidence in its next step.

With an executor from your chosen LLM provider, construct the agent with:

```rust,ignore
use llm_chain::agents::self_ask_with_search::{Agent, EarlyStoppingConfig};

let agent = Agent::with_default_search(executor, EarlyStoppingConfig {
    max_iterations: Some(5),
    max_time_elapsed_seconds: Some(60.0),
})?;
let (answer, steps) = agent.run("Who created the Rust programming language?").await?;
```

This is the default search choice when using `with_default_search`. The existing
`Agent::new(executor, search_tool, config)` continues to use your explicitly supplied
tool, including Bing, Google Serper or a custom implementation. Errors from an
explicit tool propagate without falling back to Parallel. Your LLM executor still
uses its own configuration and credentials.

To try the search tool directly from this checkout, install Rust and run:

```sh
cargo run -p llm-chain --example parallel_search -- "Rust programming language official website"
```

No LLM or search credentials are needed for this standalone command. The client
connects to `https://search.parallel.ai/mcp` using HTTP MCP. It does not read API
keys from environment variables. Anonymous access has service rate limits; quota,
HTTP, protocol and tool errors are returned to the caller without automatic retries.
If a transport session expires (HTTP 404), the error is returned and the cached
connection is cleared. A subsequent invocation reconnects; retain the same
`ToolContext` when explicitly retrying that research task.

For related direct searches and page extraction, retain one `ToolContext`:

```rust,ignore
use llm_chain::tools::{tools::ParallelSearch, Tool, ToolContext};

let search = ParallelSearch::new()?;
let context = ToolContext::default();
let results = search.invoke_typed_with_context(&"Rust release notes".into(), &context).await?;
let page = search.fetch(&["https://www.rust-lang.org/".into()], &context).await?;
```

Create a new context for a new conversation or research task. Keep the same
context for retries and follow-ups, even after a quota error. The self-ask agent
does this automatically for each `run`, including concurrent runs on one agent.
An ordinary `invoke_typed` or YAML tool invocation is a standalone task with a
fresh context. Page fetching is available through `fetch`; the self-ask agent
itself invokes only search.

Each MCP request has a 30-second timeout, configurable with
`ParallelSearch::new()?.with_timeout(std::time::Duration::from_secs(10))`. To use a custom
timeout in self-ask, pass that tool to `Agent::new`. Dropping the invocation or
agent-run future cancels its pending request. The existing agent elapsed-time
limit is checked between iterations, so it is not a hard deadline for a running
request; a host can wrap `agent.run` in `tokio::time::timeout` for an overall limit.
HTTP MCP connections are cached per client; their transport session IDs are
independent of the per-conversation tool context.
