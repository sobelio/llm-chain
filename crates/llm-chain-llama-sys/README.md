# llama-sys

llama-sys is a set of bindgen generated wrappers for llama.cpp. This crate provides a low-level interface to llama.cpp, allowing you to use it in your Rust projects. To use llama-sys, simply add the following to your Cargo.toml file:

```toml
[dependencies]
llama-sys = "0.1.0"
```

```rust
use llama_sys::\*;
```

Note that llama-sys provides a lower-level interface than llm-chain-llama, and may be more difficult to use. However, if you need fine-grained control over llama.cpp, llama-sys is the way to go.
