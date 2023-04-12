# Using Prompt Templates and Parameters

:::tip

Having problems? Don't worry reach out on [discord](https://discord.gg/kewN9Gtjt2) and we will help you out.

:::

In this part of the tutorial series, we'll explore how to use prompt templates and parameters with ll-chain. Prompt templates allow you to create dynamic prompts, and parameters are the text strings you put into your templlates.

Here's a simple Rust program demonstrating how to use prompt templates and parameters:

```rust

// Import the required traits and structs from LLM-Chain and OpenAI driver
use llm_chain::traits::StepExt;
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
                // Define a user role with a prompt template containing a placeholder
                Role::User,
                "Make a personalized greeting tweet for {}",
            ),
        ],
    )
    // Convert the Step into a Chain, which can be executed
    .to_chain();

    // Create a Parameters object with the value to replace the placeholder in the prompt template
    let parameters = "[YOUR NAME HERE]".into();

    // Execute the Chain with the provided parameters and store the result in `res`
    let res = chain.run(parameters, &exec).await.unwrap();

    let res = chain.run(parameters, &exec).await.unwrap();

    // Print the LLM response to the console
    println!("{:?}", res);
}
```

Let's break down the different parts of the code:

1. The import statements remain the same as before, including the necessary traits and structs.
2. The main async function is also unchanged, using Tokio as the runtime.
3. We create a new `Executor` with the default settings.
4. When building the `Step`, we now include a prompt template in the `Role::User` message: `"Make a personalized greeting tweet for {}"`. The `{}` is a placeholder that we'll replace with a specific value later.
5. We convert the `Step` into a `Chain`, which can be executed.
6. We create a `Parameters` object with the value `"[YOUR NAME HERE]"` to replace the placeholder in the prompt template.
7. We execute the `Chain` with the provided `parameters` and store the result in `res`.
8. Finally, we print the response to the console.

There are other ways to create parameters as well, in the above example we used a shorthand way to do it. Here is one.

```rust
// You can also create Parameters directly
let parameters = Parameters::new_with_text("[SOMEBODY ELSES NAME]")
```

In the next tutorial we will combine multiple llm invocations to solve more complicated problems.
