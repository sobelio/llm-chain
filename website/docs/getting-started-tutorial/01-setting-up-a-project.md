# Setting up a project with llm-chain

:::tip

Having problems? Don't worry reach out on [discord](https://discord.gg/kewN9Gtjt2) and we will help you out.

Welcome to llm-chain, a Rust library designed to simplify working with large language models (LLMs) and help you create powerful applications. In this tutorial, we'll walk you through installing Rust, setting up a new project, and getting started with LLM-Chain.

:::

## Installing Rust

To begin, you'll need to install Rust on your machine. We recommend using [rustup](https://rustup.rs/) , the official Rust toolchain manager, to ensure you have the latest version and can manage your installations easily.

1. Follow the instructions on the [rustup website](https://rustup.rs/) to install Rust.

## Creating a New Rust Project

Now that you have Rust installed, it's time to create a new Rust project. Run the following command to set up a new binary project:

```bash

cargo new --bin my-llm-project
```

This command will create a new directory called `my-llm-project` with the necessary files and directories for a Rust project.

## Installing LLM-Chain

With your Rust project set up, it's time to add LLM-Chain as a dependency. To do this, run the following command:

```bash

cd my-llm-project
cargo add llm-chain
```

This will add LLM-Chain to your project's `Cargo.toml` file.

## Choosing a Driver: LLAMA vs OpenAI

LLM-Chain supports multiple drivers for working with different LLMs. You can choose between the LLAMA driver (which runs a LLaMA LLM on your computer) and the OpenAI driver (which connects to the OpenAI API). For ease of use and getting started quickly, we'll be using the OpenAI driver in this tutorial. To install it run

```bash
cargo add llm-chain-openai
```

In the next tutorial, we'll cover generating your first LLM output using the OpenAI driver.
