# llm-chain 🚀

`llm-chain` is a collection of Rust crates designed to help you work with Large Language Models (LLMs) more effectively. Our primary focus is on providing robust support for prompt templates and chaining together prompts in multi-step chains, enabling complex tasks that LLMs can't handle in a single step. This includes, but is not limited to, summarizing lengthy texts or performing advanced data processing tasks.

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sobelio/llm-chain/cicd.yaml?branch=main?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/llm-chain?style=flat-square)
![Crates.io](https://img.shields.io/crates/l/llm-chain-openai?style=flat-square)
![License](https://img.shields.io/github/license/sobelio/llm-chain)

## Examples 💡

To help you get started, here is an example demonstrating how to use `llm-chain`. You can find more examples in the [examples folder](/llm-chain-openai/examples) in the repository.

```rust
let exec = Executor::new_default();
let chain = Step::new(
    Model::ChatGPT3_5Turbo,
    [
        (Role::System, "You are a bot for making personalized greetings"),
        (Role::User, "Make a personalied greet for Joe"),
    ]
).to_chain();
let res = chain.run(Parameters::new(), exec).await.unwrap();
println!("{:?}", res);
```

## Features 🌟

- **Prompt templates**: Create reusable and easily customizable prompt templates for consistent and structured interactions with LLMs.
- **Chains**: Build powerful chains of prompts that allow you to execute more complex tasks, step by step, leveraging the full potential of LLMs.
- **ChatGPT support**: Currently supports ChatGPT models, with plans to add support for more LLMs in the future.
- **Extensibility**: Designed with extensibility in mind, making it easy to integrate additional LLMs as the ecosystem grows.
- **Community-driven**: We welcome and encourage contributions from the community to help improve and expand the capabilities of `llm-chain`.

## Getting Started 🚀

To start using `llm-chain`, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
llm-chain = "0.1.0"
llm-chain-openai = "0.1.0
```

Then, refer to the [documentation](https://docs.rs/llm-chain) and [examples](/llm-chain-openai/examples) to learn how to create prompt templates, chains, and more.

## Contributing 🤝

We warmly welcome contributions from everyone! If you're interested in helping improve `llm-chain`, please check out our [`CONTRIBUTING.md`](/docs/CONTRIBUTING.md) file for guidelines and best practices.

## License 📄

`llm-chain` is licensed under the [MIT License](/LICENSE).

## Connect with Us 🌐

If you have any questions, suggestions, or feedback, feel free to open an issue or join our community discussions. We're always excited to hear from our users and learn about your experiences with `llm-chain`.

We hope you enjoy using `llm-chain` to unlock the full potential of Large Language Models in your projects. Happy coding! 🎉
