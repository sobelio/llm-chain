# Summarizing Text with Map-Reduce in LLM-Chain

:::tip

Having problems? Don't worry reach out on [discord](https://discord.gg/kewN9Gtjt2) and we will help you out.

:::

Map-reduce is a powerful technique for processing and aggregating data in parallel. In this tutorial, we'll explore how to use map-reduce in `llm-chain` to summarize text effectively. We'll cover implementing a basic map-reduce for text summarization.

To start create a file named in "article_to_summarize.md" take the content of a wikipedia article and paste it in there.

Here's a Rust program that demonstrates how to create a map-reduce chain for summarizing text:

```rust

use llm_chain::chains::map_reduce::Chain;
use llm_chain::Parameters;
use llm_chain_openai::chatgpt::{Executor, Model, Role, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let exec = Executor::new_default();
    let map_prompt = Step::new(
        Model::ChatGPT3_5Turbo,
        [
            (
                Role::System,
                "You are a bot for summarizing wikipedia articles, you are terse and focus on accuracy",
            ),
            (Role::User, "Summarize this article into bullet points:\n{}"),
        ],
    );
    let reduce_prompt = Step::new(
        Model::ChatGPT3_5Turbo,
        [
            (Role::System, "You are a diligent bot that summarizes text"),
            (
                Role::User,
                "Please combine the articles below into one summary as bullet points:\n{}",
            ),
        ],
    );
    let chain = Chain::new(map_prompt, reduce_prompt);
    let article = include_str!("article_to_summarize.md");
    let docs = vec![Parameters::new_with_text(article)];
    let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();
    println!("{:?}", res);
}
```

Let's break down the code and understand the different parts:

1. Define the map and reduce prompts as Step objects:
   a. The map_prompt summarizes a given article into bullet points.
   b. The reduce_prompt combines multiple summaries into a single summary as bullet points.
2. Create a new map-reduce Chain by providing the map_prompt and reduce_prompt.
3. Load the article to be summarized and create a Parameters object with the text.
4. Execute the map-reduce Chain with the provided Parameters and store the result in res.
5. Print the LLM response to the console.

This should be able to summarize any wikipedia article you might find. Play around with the prompt templates to make it best fit your usecase.

---

That's it folks, thanks for following along for the tutorial. You are now ready to use `llm-chain` for something useful. Don't forget to stop by [discord](https://discord.gg/kewN9Gtjt2) and share what you are making.
