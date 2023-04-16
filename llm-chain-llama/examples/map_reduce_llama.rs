use llm_chain::chains::map_reduce::Chain;
use llm_chain::{prompt, Parameters};
use llm_chain_llama::{Executor, Step};
use std::{env, path::Path};

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
    let map_prompt = Step::for_prompt(prompt!("== ARTICLE ==\n{{text}}== SUMMARY ==\n"));
    let reduce_prompt = Step::for_prompt(prompt!("== ARTICLE ==\n{{text}}== FINAL SUMMARY ==\n"));

    let chain = Chain::new(map_prompt, reduce_prompt);
    let article = include_str!("article_to_summarize.md");
    let docs = vec![Parameters::new_with_text(article)];
    let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();
    println!("{:?}", res);
}
