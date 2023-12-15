use llm_chain::output::StreamExt;
use llm_chain::{executor, parameters, prompt};

/// This example demonstrates how to use the llm-chain-llama crate to generate streaming text using a
/// LLaMA model.
///
/// Usage: cargo run --example stream
///
/// Make sure to have the env var 'LLM_CHAIN_MODEL' set.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!(llama)?;

    let res = prompt!("The Colors of the Rainbow are (in order): ")
        .run(&parameters!(), &exec)
        .await?;
    let mut stream = res.as_stream().await?;
    while let Some(v) = stream.next().await {
        print!("{}", v);
    }

    Ok(())
}
