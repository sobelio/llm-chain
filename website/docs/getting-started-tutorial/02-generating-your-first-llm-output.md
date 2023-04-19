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
use llm_chain::{prompt, traits::StepExt, Parameters};
use llm_chain_openai::chatgpt::{Executor, Step};

// Declare an async main function
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let exec = Executor::new_default();
    // Create our prompt...
    let res = Step::for_prompt(prompt!(
        "You are a robot assistant for making personalized greetings",
        "Make a personalized greeting for Joe"
    ))
    .run(&Parameters::new(), &exec) // ...and run it
    .await?;
    println!("{}", res);
    Ok(())
}
```

## Understanding LLM Response

When you run the program, you'll receive an LLM response. The response contains the generated text and other metadata.

## Error Handling and Common Issues

One common issue you might encounter is forgetting to set the OpenAI API key. Make sure you have set the API key in your `OPENAI_API_KEY` environment variable.

```bash
export OPENAI_API_KEY="YOUR_OPEN_AI_KEY" # TIP: It stars with sk-
```

In the next tutorial, we'll cover adding parameters to customize the LLM prompt to create more complicated interactions.
