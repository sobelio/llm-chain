# llm-chain-llama-sys

llm-chain-llama-sys is a set of bindgen generated wrappers for llama.cpp. This crate provides a low-level interface to llama.cpp, allowing you to use it in your Rust projects. To use llm-chain-llama-sys, simply add the following to your Cargo.toml file:

```toml
[dependencies]
llm-chain-llama-sys = "0.12.0"
```

```rust
use llm_chain_llama_sys::\*;
```

Note that llama-sys provides a lower-level interface than llm-chain-llama, and may be more difficult to use. However, if you need fine-grained control over llama.cpp, llm-chain-llama-sys is the way to go.

Please note that to build this crate succsefully in MacOS you'll need to have CMake (https://cmake.org/install/) already installed and available in the command line
