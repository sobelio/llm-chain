# Creating Your First Sequential Chain

:::tip

Having problems? Don't worry reach out on [discord](https://discord.gg/kewN9Gtjt2) and we will help you out.

:::

Sequential chains in LLM-Chain allow you to execute a series of steps, with the output of each step feeding into the next one. This tutorial will guide you through creating a sequential chain, extending it with more steps, and provide some best practices and tips.

Here's a Rust program that demonstrates how to create a sequential chain:

use llm_chain::chains::sequential::Chain;
use llm_chain_openai::chatgpt::{Executor, Model, Role, Step};

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let exec = Executor::new_default();
    let chain = Chain::new(vec![
        Step::new(
            Model::ChatGPT3_5Turbo,
            [
                (
                    Role::System,
                    "You are a bot for making personalized greetings",
                ),
                (
                    Role::User,
                    "Make personalized birthday e-mail to the whole company for {name} who has their birthday on {date}. Include their name",
                ),
            ],
        ),
        Step::new(
            Model::ChatGPT3_5Turbo,
            [
                (
                    Role::System,
                    "You are an assistant for managing social media accounts for a company",
                ),
                (
                    Role::User,
                    "Summarize this email into a tweet to be sent by the company, use emoji if you can. \n--\n{}",
                ),
            ],
        ),
    ]);
    let res = chain
        .run(
            vec![("name", "Emil"), ("date", "February 30th 2023")].into(),
            &exec,
        )
        .await
        .unwrap();
    println!("{:?}", res);
}
```

Let's break down the code and understand the different parts:

1. Build a new Chain by providing a vector of Steps. In this example, we have two steps:
   a. The first step creates a personalized birthday email for a person named Emil.
   b. The second step summarizes the email into a tweet, including an emoji if possible.
   To extend the chain with more steps, simply add additional Steps to the vector. The output of the previous step will be fed into the next step as input.
2. Create a Parameters object with the necessary values for the placeholders in the prompt templates.
3. Execute the Chain with the provided parameters and store the result in res.
4. Print the LLM response to the console.

## Best Practices and Tips

When working with sequential chains, consider the following tips and best practices:

1. Use descriptive and clear instructions for the system role to help guide the LLM.
2. Keep the chain as short and simple as possible. Longer chains are harder to manage and debug.
3. Test each step independently before in

For the next tutorial we will switch our focus from sequential to map-reduce chains. Map reduce chains are more complicated than sequential chains but allow us to do things that sequential chains can't. Stay tuned!
