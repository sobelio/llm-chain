use std::{env, path::Path};

use llm_chain::chains::map_reduce::Chain;
use llm_chain::{Parameters, default_prompt};
use llm_chain::prompt::{ExtractiveSummaryChat, DefaultPrompt};
use llm_chain_llama::{Executor, Step};
use llm_chain_llama::LlamaConfig;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --example simple path/to/llama-or-alpaca-model");
        std::process::exit(1);
    }
    // Get the path to the model.
    let path = Path::new(&args[1]);
    // Initialize the Executor with the model path.
    let exec = Executor::new(path.to_str().unwrap());

    // Create the prompts
    let map_step = Step::for_prompt(default_prompt!(ExtractiveSummaryChat));
    let reduce_step = Step::for_prompt(default_prompt!(ExtractiveSummaryChat));
    // println!("{}",default_prompt!(ExtractiveSummaryChat).to_string());
    // System: You are an extractive summarizer that follows the output pattern
    // User: Please extract sentences as the summary. The summary should contain {{sentences}} sentences. Document: {{text}}
    let chain = Chain::new(map_step, reduce_step);
    let article = include_str!("article_to_summarize.md");
    let docs = vec![Parameters::new_with_text(article)];
    let res = chain.run(docs, Parameters::new().with("sentences", "5"), &exec).await.unwrap();
    println!("{:?}", res);
}
