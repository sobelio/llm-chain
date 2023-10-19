use llm_chain::tools::{tools::GoogleSerper, Tool};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let serper_api_key = std::env::var("SERPER_API_KEY").unwrap();
    let serper = GoogleSerper::new(serper_api_key);
    let result = serper
        .invoke_typed(&"Who was the inventor of Catan?".into())
        .await
        .unwrap();
    println!("Best answer from Google Serper: {}", result.result);
}
