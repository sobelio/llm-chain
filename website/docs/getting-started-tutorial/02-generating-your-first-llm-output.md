# Generating Your First LLM Output

:::tip

Having problems? Don't worry reach out on [discord](https://discord.gg/kewN9Gtjt2) and we will help you out.

:::

First, we need to install `tokio` in our project. Since this is a tutorial we will install the full `tokio` package crate, in production, of course we should be more selective with what features we install.

```bash
cargo add tokio --features full
```

First, let's start by writing a simple Rust program that generates an LLM output using LLM-Chain and the OpenAI driver:

```rust
// Import the required traits and structs from llm-chain and our openai driver
use llm_chain::{traits::StepExt, Parameters};
use llm_chain_openai::chatgpt::{Executor, Model, Role, Step};

// Declare the main async function using Tokio as the runtime
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Create a new Executor with the default settings
    let exec = Executor::new_default();

    // Build a new Step with the ChatGPT3-5Turbo model and a set of messages with roles
    let chain = Step::new(
        Model::ChatGPT3_5Turbo,
        [
            (
                // Define a system role with an instruction for the LLM
                Role::System,
                "You are a bot for making personalized greetings",
            ),
            (
                // Define a user role with a request for the LLM
                Role::User,
                "Make a personalized greet for Joe",
            ),
        ],
    ).to_chain();
    // Convert the Step into a Chain, which can be executed

    // Execute the Chain and store the result in `res`
    let res = chain.run(Parameters::new(), &exec).await.unwrap();

    // Print the LLM response to the console
    println!("{:?}", res);
}

```

## Understanding LLM Response

When you run the program, you'll receive an LLM response, which is the standard output from OpenAI's ChatGPT API. The response contains the generated text and other metadata.

## Error Handling and Common Issues

One common issue you might encounter is forgetting to set the OpenAI API key. Make sure you have set the API key in your `OPENAI_API_KEY` environment variable.

```bash
export OPENAI_API_KEY="YOUR_OPEN_AI_KEY" # TIP: It stars with sk-
```

In the next tutorial, we'll cover adding parameters to customize the LLM prompt to create more complicated interactions.
