use llm_chain::options::{ModelRef, Opt, Options};
use std::{env::args, error::Error};

use llm_chain::{prompt::Data, traits::Executor};
use llm_chain_local::Executor as LocalExecutor;

extern crate llm_chain_local;

/// This example demonstrates how to use the llm-chain-local crate to generate text using a model.
///
/// Usage: cargo run --release --package llm-chain-local --example simple model_type path/to/model
///
/// For example, if the model is a LLaMA-type model located at "/models/llama"
/// cargo run --release --package llm-chain-local --example simple llama /models/llama
///
/// An optional third argument can be used to customize the prompt passed to the model.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let raw_args: Vec<String> = args().collect();
    let args = match &raw_args.len() {
      3 => (raw_args[1].as_str(), raw_args[2].as_str(), "Rust is a cool programming language because"),
      4 => (raw_args[1].as_str(), raw_args[2].as_str(), raw_args[3].as_str()),
      _ => panic!("Usage: cargo run --release --example inference <model type> <path to model> <optional prompt>")
    };

    let model_type = args.0;
    let model_path = args.1;
    let prompt = args.2;

    let options = Options::new()
        .with_option(Opt::Model(ModelRef::from_path(model_path)))
        .with_option(Opt::ModelType(model_type.to_string()));

    let exec = LocalExecutor::new_with_options(options)?;
    let res = exec
        .execute(Options::empty(), &Data::Text(String::from(prompt)), None)
        .await?;

    println!("{}", res);
    Ok(())
}
