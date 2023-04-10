---
id: basic-text-generation
title: "Example: Simple Text Generation"
sidebar_label: "Simple Text Generation"
---

## Example: Creating a Personalized Greeting

In this example, we demonstrate how to use LLM-chain to create a personalized greeting using a simple chain of prompts.

```rust
let exec = Executor::new_default();
let chain = Step::new(
    Model::ChatGPT3_5Turbo,
    [
        (Role::System, "You are a bot for making personalized greetings"),
        (Role::User, "Make a personalized greet for Joe"),
    ]
).to_chain();
let res = chain.run(Parameters::new(), exec).await.unwrap();
println!("{:?}", res);
```

Here's a step-by-step explanation of the code:

1. Create an `Executor` using the `new_default()` function. The `Executor` is responsible for managing the execution of the chain.
2. Define a `Step` using the `Step::new()` function, providing the model (`Model::ChatGPT3_5Turbo`) and an array of tuples representing the conversation between the system and user.
3. Convert the `Step` into a chain using the `to_chain()` function.
4. Run the chain using the `run()` function, passing in the parameters and the executor. The await keyword is used since the function is asynchronous.
5. Once the chain completes, the result is unwrapped and printed to the console.

In this example, the LLM-chain library interacts with the ChatGPT model to create a personalized greeting for Joe. This is just a simple demonstration of how LLM-chain can be used to interact with LLMs and perform more complex tasks by chaining prompts. Explore the documentation and examples for more advanced use cases.
