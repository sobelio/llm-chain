use llm_chain::{prompt, traits::StepExt, Parameters};
use llm_chain_openai::chatgpt::{Executor, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let exec = Executor::new_default();
    // Create our step containing our prompt template
    let step = Step::for_prompt(prompt!(
        "You are a bot for making personalized greetings",
        "Make a personalized greeting tweet for {{text}}" // Text is the default parameter name, but you can use whatever you want
    ));

    // A greeting for emil!
    let res = step.run(&Parameters::new_with_text("Emil"), &exec).await?;
    println!("{}", res);

    // A greeting for you
    let res = step
        .run(&Parameters::new_with_text("Your Name Here"), &exec)
        .await?;

    println!("{}", res);

    Ok(())
}
