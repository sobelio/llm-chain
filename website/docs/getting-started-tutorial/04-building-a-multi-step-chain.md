# Creating Your First Sequential Chain

:::tip

Having problems? Don't worry reach out on [discord](https://discord.gg/kewN9Gtjt2) and we will help you out.

:::

Sequential chains in LLM-Chain allow you to execute a series of steps, with the output of each step feeding into the next one. This tutorial will guide you through creating a sequential chain, extending it with more steps, and provide some best practices and tips.

Here's a Rust program that demonstrates how to create a sequential chain:

```rust
use llm_chain::{chains::sequential::Chain, prompt};
use llm_chain_openai::chatgpt::{Executor, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Create a new ChatGPT executor with the default settings
    let exec = Executor::new_default();

    // Create a chain of steps with two prompts
    let chain = Chain::new(vec![
        // First step: make a personalized birthday email
        Step::for_prompt(
            prompt!("You are a bot for making personalized greetings", "Make personalized birthday e-mail to the whole company for {{name}} who has their birthday on {{date}}. Include their name")
        ),

        // Second step: summarize the email into a tweet. Importantly, the text parameter becomes the result of the previous prompt.
        Step::for_prompt(
            prompt!( "You are an assistant for managing social media accounts for a company", "Summarize this email into a tweet to be sent by the company, use emoji if you can. \n--\n{{text}}")
        )
    ]);

    // Run the chain with the provided parameters
    let res = chain
        .run(
            // Create a Parameters object with key-value pairs for the placeholders
            vec![("name", "Emil"), ("date", "February 30th 2023")].into(),
            &exec,
        )
        .await
        .unwrap();

    // Print the result to the console
    println!("{:?}", res);
}
```

1. We start by importing the necessary modules from the `llm_chain` and `llm_chain_openai` libraries.
2. The main async function is defined, using Tokio as the runtime.
3. We create a new `Executor` with the default settings.
4. We create a `Chain` that contains two steps, each with a different prompt:

   - The first step has a prompt to make a personalized birthday email for a company.
   - The second step has a prompt to summarize the email into a tweet.

   Both prompts use placeholders (e.g., `{{name}}`, `{{date}}`, and `{{text}}`) that will be replaced with specific values later. Importantly the value of `{{text}}` will replaced by result of the first step in the chain.

5. We run the `Chain` with the provided parameters:

   - We create a `Parameters` object with key-value pairs for the placeholders: `("name", "Emil")` and `("date", "February 30th 2023")`.
   - We pass the `Parameters` object and the `Executor` to the `run()` method.

6. We unwrap the result and print it to the console.

## Best Practices and Tips

When working with sequential chains, consider the following tips and best practices:

1. Use descriptive and clear instructions for the system role to help guide the LLM.
2. Keep the chain as short and simple as possible. Longer chains are harder to manage and debug.
3. Test each step independently before in

For the next tutorial we will switch our focus from sequential to map-reduce chains. Map reduce chains are more complicated than sequential chains but allow us to do things that sequential chains can't. Stay tuned!
