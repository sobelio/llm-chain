use llm_chain::{prompt, traits::StepExt};
use llm_chain_openai::chatgpt::{Executor, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec = Executor::new_default();
    let res = Step::for_prompt(prompt!(
        "You are a bot for making personalized greetings",
        "Make a personalized greeting tweet for {text}"
    ))
    .run(&("Emil".into()), &exec)
    .await?;
    println!("{}", res);
    Ok(())
}
