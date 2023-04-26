use llm_chain::{chains::sequential::Chain, executor, output::Output, prompt, step::Step};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor with the default settings
    let exec = executor!()?;

    // Create a chain of steps with two prompts
    // Second step is streaming
    let chain = Chain::new(vec![
        // First step: make a personalized birthday email
        Step::for_prompt_template(
            prompt!("You are a bot for making personalized greetings", "Make personalized birthday e-mail to the whole company for {{name}} who has their birthday on {{date}}. Include their name")
        ),

        // Second step: summarize the email into a tweet. Importantly, the text parameter becomes the result of the previous prompt.
        Step::for_prompt_with_streaming(
            prompt!( "You are an assistant for managing social media accounts for a company", "Summarize this email into a tweet to be sent by the company, use emoji if you can. \n--\n{{text}}")
        )
    ]);

    // Run the chain with the provided parameters
    let res = chain
        .run(
            // Create a Parameters object with key-value pairs for the placeholders
            vec![("name", "Emil"), ("date", "February 30th 2023")].into(),
            &exec,
        )
        .await
        .unwrap();

    // Print the result to the console
    // Call `res.primary_textual_output()` explictly to get the streamed response.
    println!("{:?}", res.primary_textual_output().await);

    // Call `res.as_stream()` to access the Stream without polling.
    Ok(())
}
