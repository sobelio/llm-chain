# Introducing v0.8.1: Enhanced Prompt Macro and New Conversational Chain Type

We are excited to announce the release of version 0.8.1, which brings two major improvements to our Large Language Model (LLM) library: an enhanced `prompt!` macro and a new Conversational chain type. These updates make it even easier for developers to create rich and interactive applications powered by LLMs.

## Enhanced Prompt Macro with Prefixes

The `prompt!` macro has been updated to support prefixes, making it more expressive and convenient to use. With this new feature, you can now create chat prompts by simply prefixing them with `user:`, `assistant:`, or `system:`. Here's an example of how to use the new syntax:

```rust

let user_prompt = prompt!(user: "Hello, Mr Bot, help me figure out what to do next");
let system_prompt = prompt!(system: "You are a clever assistant that");
```

By using these prefixes, you can create more complex and interactive prompts for various use cases, such as building chatbots, automating tasks, or generating text.

## New Conversational Chain Type

We're also introducing the Conversational chain type, which enables you to have ongoing conversations with LLMs. Conversational chains manage the conversation history and context, ensuring that the LLM's responses remain relevant and coherent throughout the interaction. This new chain type is particularly useful for chatbot applications, multi-step interactions, and any scenario where context is essential.

Here's a quick example of a Conversational chain:

```rust
use llm_chain::{
    chains::conversation::Chain, executor, output::Output, parameters, prompt, step::Step,
};
use tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor.
    let exec = executor!()?;

    // Create a new Chain with the executor.
    let mut chain = Chain::new(
        prompt!(system: "You are a robot assistant for making personalized greetings."),
    )?;

    // Define the conversation steps.
    let step1 = Step::for_prompt_template(prompt!(user: "Make a personalized greeting for Joe."));
    let step2 =
        Step::for_prompt_template(prompt!(user: "Now, create a personalized greeting for Jane."));
    let step3 = Step::for_prompt_template(
        prompt!(user: "Finally, create a personalized greeting for Alice."),
    );

    let step4 = Step::for_prompt_template(prompt!(user: "Remind me who did we just greet."));

    // Execute the conversation steps.
    let res1 = chain.send_message(step1, &parameters!(), &exec).await?;
    println!("Step 1: {}", res1.primary_textual_output().await.unwrap());

    let res2 = chain.send_message(step2, &parameters!(), &exec).await?;
    println!("Step 2: {}", res2.primary_textual_output().await.unwrap());

    let res3 = chain.send_message(step3, &parameters!(), &exec).await?;
    println!("Step 3: {}", res3.primary_textual_output().await.unwrap());

    let res4 = chain.send_message(step4, &parameters!(), &exec).await?;
    println!("Step 4: {}", res4.primary_textual_output().await.unwrap());

    Ok(())
}
```

With the Conversational chain, you can now easily send multiple messages and manage the conversation context without having to worry about manual context management.

## Upgrade Today

We encourage you to upgrade to version 0.8.1 and take advantage of these new features. The enhanced `prompt!` macro and the new Conversational chain type will make your LLM-powered applications even more interactive and engaging.

As always, we appreciate your feedback and suggestions. Feel free to reach out to our team for any questions or concerns. Happy coding!
