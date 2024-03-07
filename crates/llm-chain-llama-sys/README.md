# llm-chain-llama-sys

llm-chain-llama-sys is a set of bindgen generated wrappers for llama.cpp. This crate provides a low-level interface to llama.cpp, allowing you to use it in your Rust projects. To use llm-chain-llama-sys, simply add the following to your Cargo.toml file:

```toml
[dependencies]
llm-chain-llama-sys = "0.1.0"
```

```rust
use llm-chain-llama-sys::*;
```

Note that llm-chain-llama-sys provides a lower-level interface than llm-chain-llama, and may be more difficult to use. However, if you need fine-grained control over llama.cpp, llm-chain-llama-sys is the way to go.

## Updating llama.cpp submodule
To update the llama.cpp submodule, run the following command:

```console
$ git submodule update --remote --merge llama.cpp
```
Then to save the generated bindings run the build and set the environment
variable `LLAMA_SAVE_BINDINGS` to `true`:

```console
$ env LLAMA_SAVE_BINDINGS=true cargo build --release
```

And then check-in the generated bindings in `src/bindings.rs`.
