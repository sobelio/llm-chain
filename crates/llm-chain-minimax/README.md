# llm-chain-minimax

Use [MiniMax](https://www.minimax.io/) LLM models with `llm-chain`! This crate provides seamless integration with MiniMax's OpenAI-compatible API, giving you access to powerful models like MiniMax-M2.7 and MiniMax-M2.5.

## Supported Models

| Model | Context Window | Description |
|---|---|---|
| `MiniMax-M2.7` | 1M tokens | Latest and most capable model |
| `MiniMax-M2.7-highspeed` | 1M tokens | Faster variant of M2.7 |
| `MiniMax-M2.5` | 1M tokens | Previous generation, strong capabilities |
| `MiniMax-M2.5-highspeed` | 204K tokens | Optimized for speed |

## Quick Start

1. Get your MiniMax API key from [MiniMax Platform](https://www.minimax.io/)
2. Set the environment variable:

```bash
export MINIMAX_API_KEY="your-api-key-here"
```

3. Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
llm-chain = "0.13.0"
llm-chain-minimax = "0.13.0"
```

4. Use the MiniMax executor:

```rust
use llm_chain::executor;

let exec = executor!(minimax)?;
```

Or with a specific model:

```rust
use llm_chain::{executor, options};

let exec = executor!(minimax, options!(Model: "MiniMax-M2.5-highspeed"))?;
```

## Environment Variables

| Variable | Description |
|---|---|
| `MINIMAX_API_KEY` | Your MiniMax API key (required) |
| `MINIMAX_API_BASE_URL` | Custom API base URL (default: `https://api.minimax.io/v1`) |

## Examples

See the [examples](./examples) directory for usage examples.
