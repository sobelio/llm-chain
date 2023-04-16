use std::{env, path::Path};

use llm_chain::{prompt, traits::StepExt, Parameters};
use llm_chain_llama::{Executor, Step};

/// This example demonstrates how to use the llm-chain-llama crate to generate text using a
/// LLaMA model.
///
/// Usage: cargo run --example simple path/to/llama-or-alpaca-model
///
/// For example, if the model is located at "/models/llama"
/// cargo run --example simple /models/llama
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --example simple path/to/llama-or-alpaca-model");
        std::process::exit(1);
    }

    // Get the path to the model.
    let path = Path::new(&args[1]);

    // Initialize the Executor with the model path.
    let exec = Executor::new(path.to_str().unwrap());

    let res = Step::for_prompt(prompt!("The Colors of the Rainbow are (in order): "))
        .run(&Parameters::new(), &exec)
        .await?;
    println!("{}", res);
    Ok(())
}
