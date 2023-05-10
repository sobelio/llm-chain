use llm_chain::tools::{tools::BingSearch, Tool};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let bing_api_key = std::env::var("BING_API_KEY").unwrap();
    let bing = BingSearch::new(bing_api_key);
    let result = bing
        .invoke_typed(&"Who was the inventor of Catan?".into())
        .await
        .unwrap();
    println!("Best answer from bing: {}", result.result);
}
