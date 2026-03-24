/// This example demonstrates how to use the MiniMax executor for a simple
/// chat completion. Make sure to set the `MINIMAX_API_KEY` environment variable.
///
/// ```bash
/// export MINIMAX_API_KEY="your-api-key-here"
/// cargo run --example simple_invocation
/// ```
use llm_chain::{executor, parameters};
use llm_chain::prompt::{ConversationTemplate, StringTemplate};
use llm_chain::step::Step;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a MiniMax executor with the default model (MiniMax-M2.7)
    let exec = executor!(minimax)?;

    let prompt = ConversationTemplate::new()
        .with_system_template("You are a helpful assistant.")
        .with_user(StringTemplate::tera("{{question}}"));

    let result = Step::for_prompt_template(prompt.into())
        .run(&parameters!("question" => "What is the capital of France?"), &exec)
        .await?;

    println!(
        "Response: {}",
        result
            .to_immediate()
            .await?
            .primary_textual_output()
            .unwrap_or_default()
    );
    Ok(())
}
