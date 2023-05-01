use llm_chain::{
    agents::self_ask_with_search::{Agent, EarlyStoppingConfig},
    executor,
    tools::{tools::BingSearch, ToolCollection},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let executor = executor!().unwrap();
    let bing_api_key = std::env::var("BING_API_KEY").unwrap();
    let search_tool = BingSearch::new(bing_api_key);
    let agent = Agent::new(
        executor,
        search_tool,
        EarlyStoppingConfig {
            max_iterations: Some(10),
            max_time_elapsed_seconds: Some(30.0),
        },
    );
    let (res, intermediate_steps) = agent
        .run("What is the capital of the birthplace of Levy Mwanawasa?")
        .await
        .unwrap();
    println!(
        "Agent final answer: {}",
        res.return_values.get("output").unwrap()
    );
    println!("Agent full response: {}", res.log);
    println!("Intermediate steps: {:#?}", intermediate_steps);
}