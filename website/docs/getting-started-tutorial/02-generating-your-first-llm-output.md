# Generating Your First LLM Output

:::tip

Having problems? Don't worry, reach out on [discord](https://discord.gg/kewN9Gtjt2) and we will help you out.

:::

First, we need to install `tokio` in our project. Since this is a tutorial we will install the full `tokio` package crate, in production, of course we should be more selective with what features we install.

```bash
cargo add tokio --features full
```

First, let's start by writing a simple Rust program that generates an LLM output using LLM-Chain and the OpenAI driver:

```rust
use llm_chain::{executor, parameters, prompt};

// Declare an async main function
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let exec = executor!()?;
    // Create our prompt...
    let res = prompt!(
        "You are a robot assistant for making personalized greetings",
        "Make a personalized greeting for Joe"
    )
    .run(&parameters!(), &exec) // ...and run it
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
If you don't want to set enviroment variable or want to multiple api-keys. Then you can use a different macro like this. 

```rust
use llm_chain::{executor, options, parameters, prompt};
use tokio;

// Declare an async main function
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let options = options! {
        ApiKey: "sk-proj-..."
    };

    let exec = executor!(chatgpt, options);
    match exec {
        Ok(exec) => {
            
            let res = prompt!(
                "You are a robot assistant for making personalized greetings",
                "Make a personalized greeting for Joe"
            )
            .run(&parameters!(), &exec) // ...and run it
            .await?;
            println!("{}", res);
        }
        Err(err) => panic!("Unable to create executor: {}", err),
    }
    // Create our step containing our prompt template

    Ok(())
}

```

In the next tutorial, we'll cover adding parameters to customize the LLM prompt to create more complicated interactions.
