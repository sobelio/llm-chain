use llm_chain::{executor, prompt, Parameters};

// Declare an async main function
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let exec = executor!()?;
    // Create our prompt...
    let res = prompt!(
        "You are a robot assistant for making personalized greetings",
        "Make a personalized greeting for Joe"
    )
    .run(&Parameters::new(), &exec) // ...and run it
    .await?;
    println!("{}", res);
    Ok(())
}
