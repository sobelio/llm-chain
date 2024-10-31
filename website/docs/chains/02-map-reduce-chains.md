# Map-Reduce Chains

Map-Reduce chains are a powerful way to process large amounts of text using large language models (LLMs). They consist of two main steps: a "map" step, which processes each text chunk independently, and a "reduce" step, which combines the results of the map step into a single output. This approach enables the efficient processing of large documents that exceed the LLM's context window size.

In this guide, we'll explain how to create and execute a map-reduce chain using an example. The example demonstrates how to summarize a Wikipedia article into bullet points using a two-step process:

1. The "map" step summarizes each chunk of the article into bullet points.
2. The "reduce" step combines all bullet point summaries into a single summary.

```rust
use llm_chain::chains::map_reduce::Chain;
use llm_chain::step::Step;
use llm_chain::{executor, parameters, prompt, Parameters};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor with the default settings
    let exec = executor!()?;

    // Create the "map" step to summarize an article into bullet points
    let map_prompt = Step::for_prompt_template(prompt!(
        "You are a bot for summarizing wikipedia articles, you are terse and focus on accuracy",
        "Summarize this article into bullet points:\n{{text}}"
    ));

    // Create the "reduce" step to combine multiple summaries into one
    let reduce_prompt = Step::for_prompt_template(prompt!(
        "You are a diligent bot that summarizes text",
        "Please combine the articles below into one summary as bullet points:\n{{text}}"
    ));

    // Create a map-reduce chain with the map and reduce steps
    let chain = Chain::new(map_prompt, reduce_prompt);

    // Load the content of the article to be summarized
    let article = include_str!("article_to_summarize.md");

    // Create a vector with the Parameters object containing the text of the article
    let docs = vec![parameters!(article)];

    // Run the chain with the provided documents and an empty Parameters object for the "reduce" step
    let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();

    // Print the result to the console
    println!("{}", res);
    Ok(())
}
```

In this example, we start by importing the necessary modules and defining the main function. We then create a new ChatGPT executor using the executor!() macro.

Next, we create the "map" and "reduce" steps using Step::for_prompt_template(). The "map" step is responsible for summarizing each article chunk, while the "reduce" step combines the summaries into a single output.

After defining the steps, we create a new Chain object by passing in the "map" and "reduce" steps. We then load the content of the article to be summarized and create a Parameters object containing the text.

Finally, we execute the map-reduce chain using the chain.run() method, passing in the documents, an empty Parameters object for the "reduce" step, and the executor. The result is printed to the console.

Map-Reduce chains offer an effective way to handle large documents or multiple documents using LLMs. By breaking the text into manageable chunks and combining the results, you can create efficient pipelines for text processing tasks such as summarization, translation, and analysis.
