use llm_chain::executor;
use llm_chain::options;
use llm_chain::options::Options;
use std::{env::args, error::Error};

use llm_chain::{prompt::Data, traits::Executor};

extern crate llm_chain_ernie;
use llm_chain_ernie::model::Model;

/// This example demonstrates how to use the llm-chain-ernie crate to generate text.
///
/// Usage: before running this example code, you need to export QIANFAN_AK and QIANFAN_SK:
/// export QIANFAN_AK = <YOUR_AK>
/// export QIANFAN_SK = <YOUR_SK>
/// cargo run --release --package llm-chain-ernie --example simple_generator <optional prompt>
///
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let raw_args: Vec<String> = args().collect();
    let prompt = match &raw_args.len() {
        1 => "Rust is a cool programming language because",
        2 => raw_args[1].as_str(),
        _ => panic!("Usage: cargo run --release --example simple <optional prompt>"),
    };

    let opts = options!(
        Model: Model::ErnieBotTurbo,
        MaxTokens: 50usize,
        Temperature: 0.8
    );
    let exec = executor!(ernie_endpoint, opts)?;
    let res = exec
        .execute(Options::empty(), &Data::Text(String::from(prompt)))
        .await?;

    println!("{}", res);
    Ok(())
}
