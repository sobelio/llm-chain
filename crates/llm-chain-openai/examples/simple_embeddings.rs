use llm_chain::traits::Embeddings;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let embeddings = llm_chain_openai::embeddings::Embeddings::default();
    let embedded_vecs = embeddings
        .embed_texts(vec![
            "This is an amazing way of writing LLM-powered applications".to_string(),
        ])
        .await
        .unwrap();
    println!("Embedded text: {:?}", embedded_vecs[0])
}
