use llm_chain::{chains::conversation::Chain, executor, parameters, prompt, step::Step};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor.
    let exec = executor!()?;

    // Create a new Chain with the executor.
    let mut chain = Chain::new(
        prompt!(system: "You are a robot assistant for making personalized greetings."),
    )?;

    // Define the conversation steps.
    let step1 = Step::for_prompt_template(prompt!(user: "Make a personalized greeting for Joe."));
    let step2 =
        Step::for_prompt_template(prompt!(user: "Now, create a personalized greeting for Jane."));
    let step3 = Step::for_prompt_template(
        prompt!(user: "Finally, create a personalized greeting for Alice."),
    );

    let step4 = Step::for_prompt_template(prompt!(user: "Remind me who did we just greet."));

    // Execute the conversation steps.
    let res1 = chain.send_message(step1, &parameters!(), &exec).await?;
    println!("Step 1: {}", res1.to_immediate().await?);

    let res2 = chain.send_message(step2, &parameters!(), &exec).await?;
    println!("Step 2: {}", res2.to_immediate().await?);

    let res3 = chain.send_message(step3, &parameters!(), &exec).await?;
    println!("Step 3: {}", res3.to_immediate().await?);

    let res4 = chain.send_message(step4, &parameters!(), &exec).await?;
    println!("Step 4: {}", res4.to_immediate().await?);

    Ok(())
}
