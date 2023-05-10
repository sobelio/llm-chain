use llm_chain::{executor, parameters, prompt};

/// This example demonstrates how to use the llm-chain-llama crate to generate text using a
/// LLaMA model.
///
/// Usage: cargo run --example simple path/to/llama-or-alpaca-model
///
/// For example, if the model is located at "/models/llama"
/// cargo run --example simple /models/llama
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!(llama)?;

    let res = prompt!("The Colors of the Rainbow are (in order): ")
        .run(&parameters!(), &exec)
        .await?;
    println!("{}", res);
    Ok(())
}
