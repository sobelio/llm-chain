Sequential Chains

Sequential chains are a convenient way to apply large language models (LLMs) to a sequence of tasks. They connect multiple steps together, where the output of the first step becomes the input of the second step, and so on. This method allows for straightforward processing of information, where each step builds upon the results of the previous one.

In this guide, we'll explain how to create and execute a sequential chain using an example. The example demonstrates a two-step process, where the first step generates a personalized birthday email, and the second step summarizes the email into a tweet.

```rust
use llm_chain::parameters;
use llm_chain::step::Step;
use llm_chain::traits::Executor as ExecutorTrait;
use llm_chain::{chains::sequential::Chain, prompt};
use llm_chain_openai::chatgpt::Executor;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor with the default settings
    let exec = Executor::new()?;

    // Create a chain of steps with two prompts
    let chain: Chain<Executor> = Chain::new(vec![
        // First step: make a personalized birthday email
        Step::for_prompt_template(
            prompt!("You are a bot for making personalized greetings", "Make personalized birthday e-mail to the whole company for {{name}} who has their birthday on {{date}}. Include their name")
        ),

        // Second step: summarize the email into a tweet. Importantly, the text parameter becomes the result of the previous prompt.
        Step::for_prompt_template(
            prompt!( "You are an assistant for managing social media accounts for a company", "Summarize this email into a tweet to be sent by the company, use emoji if you can. \n--\n{{text}}")
        )
    ]);

    // Run the chain with the provided parameters
    let res = chain
        .run(
            // Create a Parameters object with key-value pairs for the placeholders
            parameters!("name" => "Emil", "date" => "February 30th 2023"),
            &exec,
        )
        .await
        .unwrap();

    // Print the result to the console
    println!("{:?}", res);
    Ok(())
}
```

In this example, we start by importing the necessary modules and defining the main function. Then, we create a new ChatGPT executor using the Executor::new() function. The executor is responsible for running the LLM.

Next, we create a new Chain object by passing in a vector of Step objects. Each step represents a separate LLM prompt. In this case, we have two steps:

1. The first step generates a personalized birthday email using the provided name and date parameters.
2. The second step summarizes the previously generated email into a tweet. Note that the {{text}} placeholder in the prompt is automatically filled with the result of the previous step.
After defining the chain, we execute it using the chain.run() method. We provide a Parameters object containing key-value pairs for the placeholders in the prompts (e.g., name and date) and the executor.

Finally, we print the result of the chain to the console.

Sequential chains offer an efficient and straightforward way to perform a series of tasks using LLMs. By organizing the steps in a specific order, you can create complex processing pipelines that leverage the capabilities of LLMs effectively.
