---
slug: using-chatgpt-in-rust
title: Using ChatGPT in Rust with llm-chain
authors: [whn]
tags: [llm-chain, introduction, chatgpt, rust]
---

# Using ChatGPT in Rust with llm-chain

In this blog post, we'll explore how to use ChatGPT in Rust with the help of the `llm-chain` library. We will walk through a simple example that demonstrates how to generate responses using OpenAI's ChatGPT model.

## Getting Started

First, let's start by installing the necessary packages using `cargo add`. You will need the `llm-chain` and `llm-chain-openai` libraries:

```sh
cargo add llm-chain llm-chain-openai
```

Now, let's dive into the code:

```rust

use llm_chain::{traits::StepExt, Parameters};
use llm_chain_openai::chatgpt::{Executor, Model, Role, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let exec = Executor::new_default();
    let chain = Step::new(
        Model::ChatGPT3_5Turbo,
        [
            (
                Role::System,
                "You are a helpful assistant",
            ),
            (Role::User, "Tell me about the Rust programming language"),
        ],
    )
    .to_chain();
    let res = chain.run(Parameters::new(), &exec).await.unwrap();
    println!("{:?}", res);
}
```

In the code snippet above, we begin by importing the necessary modules and functions from the `llm-chain` and `llm-chain-openai` libraries. We then define a simple `main` function that uses the `Executor` and `Step` structs to create a conversational chain.

The `Model::ChatGPT3_5Turbo` model is used as the language model in this example. We also define two steps in the conversation: the first one sets the role of the assistant and the second one asks a question about the Rust programming language.

Finally, we execute the conversation chain using the `run` method and print the generated response.

## Wrapping Up

As you can see, using ChatGPT in Rust with `llm-chain` is a straightforward and efficient process. The library makes it easy to build and manage conversational agents in Rust, allowing developers to focus on creating more powerful and interactive applications.

To continue learning about ChatGPT in Rust and how to make the most of the `llm-chain` library, try our [tutorial](https://chat.openai.com/docs/getting-started-tutorial/index) .
