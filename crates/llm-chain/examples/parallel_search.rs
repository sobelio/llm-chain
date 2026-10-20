use llm_chain::tools::{tools::ParallelSearch, Tool, ToolContext};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if query.is_empty() {
        eprintln!("Usage: cargo run -p llm-chain --example parallel_search -- <search query>");
        std::process::exit(2);
    }
    let search = ParallelSearch::new()?;
    let context = ToolContext::default();
    let result = search
        .invoke_typed_with_context(&query.into(), &context)
        .await?;
    println!("{}", result.result);
    Ok(())
}
