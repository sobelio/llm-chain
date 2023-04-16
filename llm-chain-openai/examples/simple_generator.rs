use llm_chain::{prompt, traits::StepExt, Parameters};
use llm_chain_openai::chatgpt::{Executor, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec = Executor::new_default();
    let res = Step::for_prompt(prompt!(
        "You are a robot assistant for making personalized greetings",
        "Make a personalized greeting for Joe"
    ))
    .run(&Parameters::new(), &exec)
    .await?;
    println!("{}", res);
    Ok(())
}
