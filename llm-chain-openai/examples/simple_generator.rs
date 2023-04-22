use llm_chain::{executor, parameters, prompt};

// Declare an async main function
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let exec = executor!()?;
    // Create our prompt...
<<<<<<< HEAD
    let res = prompt!(
        "You are a robot assistant for making personalized greetings",
        "Make a personalized greeting for Joe"
    )
    .run(&parameters!(), &exec) // ...and run it
=======
    let res = Step::for_prompt(prompt!(
        "You are a robot assistant for making personalized greetings",
        "Make a personalized greeting for Joe"
    ))
    .run(&Parameters::new(), &exec) // ...and run it
>>>>>>> 2fbaff0 (Add for_prompt_stream method to openai Step.)
    .await?;
    println!("{}", res);
    Ok(())
}
