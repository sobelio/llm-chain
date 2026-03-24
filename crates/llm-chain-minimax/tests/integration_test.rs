/// Integration tests for llm-chain-minimax.
///
/// These tests require a valid `MINIMAX_API_KEY` environment variable.
/// They are ignored by default and can be run with:
///
/// ```bash
/// MINIMAX_API_KEY="your-key" cargo test -p llm-chain-minimax --test integration_test -- --ignored
/// ```
use llm_chain::options::{ModelRef, Opt, Options};
use llm_chain::prompt::{ConversationTemplate, StringTemplate};
use llm_chain::step::Step;
use llm_chain::traits::Executor;
use llm_chain_minimax::chatgpt;

fn has_minimax_key() -> bool {
    std::env::var("MINIMAX_API_KEY").is_ok()
}

fn create_executor() -> chatgpt::Executor {
    chatgpt::Executor::new().expect("Failed to create MiniMax executor")
}

fn create_executor_with_model(model: &str) -> chatgpt::Executor {
    let mut builder = Options::builder();
    builder.add_option(Opt::Model(ModelRef::from_model_name(model)));
    chatgpt::Executor::new_with_options(builder.build())
        .expect("Failed to create MiniMax executor")
}

#[tokio::test]
#[ignore]
async fn test_simple_chat_completion() {
    if !has_minimax_key() {
        eprintln!("Skipping: MINIMAX_API_KEY not set");
        return;
    }
    let exec = create_executor();
    let prompt = ConversationTemplate::new()
        .with_system_template("You are a helpful assistant. Reply concisely.")
        .with_user(StringTemplate::tera("What is 2+2? Just say the number."));

    let result = Step::for_prompt_template(prompt.into())
        .run(&llm_chain::parameters!(), &exec)
        .await
        .expect("Execution failed");

    let output = result
        .to_immediate()
        .await
        .expect("Failed to get immediate output")
        .primary_textual_output()
        .expect("No textual output");

    assert!(output.contains('4'), "Expected '4' in output, got: {}", output);
}

#[tokio::test]
#[ignore]
async fn test_m27_model() {
    if !has_minimax_key() {
        eprintln!("Skipping: MINIMAX_API_KEY not set");
        return;
    }
    let exec = create_executor_with_model("MiniMax-M2.7");
    let prompt = ConversationTemplate::new()
        .with_system_template("You are a helpful assistant.")
        .with_user(StringTemplate::tera("Say 'hello' and nothing else."));

    let result = Step::for_prompt_template(prompt.into())
        .run(&llm_chain::parameters!(), &exec)
        .await
        .expect("Execution failed with MiniMax-M2.7");

    let output = result
        .to_immediate()
        .await
        .expect("Failed to get output")
        .primary_textual_output()
        .expect("No output");

    let output_lower = output.to_lowercase();
    assert!(
        output_lower.contains("hello"),
        "Expected 'hello' in output, got: {}",
        output
    );
}

#[tokio::test]
#[ignore]
async fn test_m25_highspeed_model() {
    if !has_minimax_key() {
        eprintln!("Skipping: MINIMAX_API_KEY not set");
        return;
    }
    let exec = create_executor_with_model("MiniMax-M2.5-highspeed");
    let prompt = ConversationTemplate::new()
        .with_system_template("You are a helpful assistant.")
        .with_user(StringTemplate::tera(
            "What programming language is the Linux kernel written in? Reply in one word.",
        ));

    let result = Step::for_prompt_template(prompt.into())
        .run(&llm_chain::parameters!(), &exec)
        .await
        .expect("Execution failed with MiniMax-M2.5-highspeed");

    let output = result
        .to_immediate()
        .await
        .expect("Failed to get output")
        .primary_textual_output()
        .expect("No output");

    let output_lower = output.to_lowercase();
    assert!(
        output_lower.contains('c'),
        "Expected 'C' in output, got: {}",
        output
    );
}
