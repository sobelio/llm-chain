# Using Prompt Templates and Parameters

:::tip

Having problems? Don't worry reach out on [discord](https://discord.gg/kewN9Gtjt2) and we will help you out.

:::

In this part of the tutorial series, we'll explore how to use prompt templates and parameters with llm-chain. Prompt templates allow you to create dynamic prompts, and parameters are the text strings you put into your templates.

Here's a simple Rust program demonstrating how to use prompt templates and parameters:

```rust
use llm_chain::{executor, parameters, prompt, step::Step};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let exec = executor!()?;
    // Create our step containing our prompt template
    let step = Step::for_prompt_template(prompt!(
        "You are a bot for making personalized greetings",
        "Make a personalized greeting tweet for {{text}}" // Text is the default parameter name, but you can use whatever you want
    ));

    // A greeting for emil!
    let res = step.run(&parameters!("Emil"), &exec).await?;
    println!("{}", res);

    // A greeting for you
    let res = step.run(&parameters!("Your Name Here"), &exec).await?;

    println!("{}", res.to_immediate().await?.as_content());

    Ok(())
}

```

Let's break down the different parts of the code:

1. We start with importing the necessary libraries, including the traits and structs required for our program.
2. The main async function is defined, using Tokio as the runtime.
3. We create a new `Executor` with the default settings.
4. A `Step` is created containing our prompt template with a placeholder (`{{text}}`) that will be replaced with a specific value later.
5. We create a `Parameters` object with the value "Emil" to replace the placeholder in the prompt template.
6. We execute the `Step` with the provided `parameters` and store the result in `res`, then print the response to the console.
7. We create another `Parameters` object, this time with the value "Your Name Here" to replace the placeholder.
8. We execute the `Step` again with the new `parameters`, store the result in `res`, and print the response to the console.

In the next tutorial, we will combine multiple LLM invocations to solve more complicated problems.
