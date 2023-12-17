use llm_chain::options;
use llm_chain::traits::Embeddings;

/// This example demonstrates using llm-chain-llama for generating
/// embeddings.
///
/// Usage:
/// env LLM_CHAIN_MODEL=<path_to_model> cargo run --example simple_embeddings
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = options!(
        NThreads: 4_usize,
        MaxTokens: 2048_usize
    );
    let embeddings = llm_chain_llama::embeddings::Embeddings::new_with_options(opts)?;
    let embedded_vecs = embeddings
        .embed_texts(vec![
            "This is an amazing way of writing LLM-powered applications".to_string(),
        ])
        .await
        .unwrap();
    println!("Embedded text: {:?}", embedded_vecs[0]);

    Ok(())
}
