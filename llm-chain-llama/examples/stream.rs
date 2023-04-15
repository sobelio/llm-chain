use std::{env, io::Write, path::Path};

use llm_chain::{traits::StepExt, Parameters, PromptTemplate};
use llm_chain_llama::{Executor, Step};

/// This example demonstrates how to use the llm-chain-llama crate to generate text using a
/// LLaMA model.
///
/// Usage: cargo run --example simple path/to/llama-or-alpaca-model
///
/// For example, if the model is located at "/models/llama"
/// cargo run --example simple /models/llama
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
    let exec = Executor::new_with_callback(path.to_str().unwrap(), |output| {
        print!("{}", output);
        std::io::stdout().flush().unwrap();
    });

    // Create a chain with a single step using a prompt template
    let chain = Step::new(PromptTemplate::new(
        "The Colors of the Rainbow are (in order): ",
    ))
    .to_chain();

    // Execute the chain and print the result
    let res = chain.run(Parameters::new(), &exec).await.unwrap().unwrap();
    println!("{}", res);
}
