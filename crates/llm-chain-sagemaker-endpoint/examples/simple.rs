use llm_chain::executor;
use llm_chain::options::Options;
use std::{env::args, error::Error};

use llm_chain::{prompt::Data, traits::Executor};

extern crate llm_chain_sagemaker_endpoint;

/// This example demonstrates how to use the llm-chain-mock crate to generate text using a mock model.
///
/// Usage: cargo run --release --package llm-chain-mock --example simple
///
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let raw_args: Vec<String> = args().collect();
    let prompt = match &raw_args.len() {
      1 => "Rust is a cool programming language because",
      2 => raw_args[1].as_str(),
      _ => panic!("Usage: cargo run --release --example simple")
    };

    let exec = executor!(sagemaker_endpoint)?;
    let res = exec
        .execute(Options::empty(), &Data::Text(String::from(prompt)))
        .await?;

    println!("{}", res);
    Ok(())
}