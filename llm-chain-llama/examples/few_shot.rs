use llm_chain::prompt::{Conversation};
use llm_chain::{
    chains::conversation::Chain, executor, parameters, prompt, step::Step, output::Output
};
use llm_chain_llama::PerInvocation;
/// This example demonstrates how to use the llm-chain for few-shot prompting
///
/// This example can be seen as a "chain of thought"
///
/// Usage: cargo run --example few-shot
///
/// Make sure to have the env var 'LLAMA_MODEL_PATH' set
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut inv_options = PerInvocation::new();
    inv_options.n_threads = Some(4); // default is one
    inv_options.stop_sequence = Some("\n".to_string());
    let exec = executor!(llama)?;

    let system_prompt = "Given two words, you are tasked to concatenate the last letter of each word together. answer as Assistant:";
    let user_prompt = "Take the last letters of the words in {{ full_name }} and concatenate them";
    let res =
        Step::for_prompt_and_options(prompt!(system_prompt, user_prompt), inv_options.clone())
            .run(&parameters!().with("full_name", "Elon Musk"), &exec)
            .await?;
    println!("{} (zero-shot answer)", res); // probably not correct
    let conversation = Conversation::new()
        .with_system(system_prompt.to_string())
        .with_user_template(
            user_prompt,
            &parameters!().with("full_name", "Saquon Barkley"),
        )?
        .with_assistant("SB".to_string())
        .with_user_template(
            user_prompt,
            &parameters!().with("full_name", "Sean Connery"),
        )?
        .with_assistant("SC".to_string())
        .with_user_template(
            user_prompt,
            &parameters!().with("full_name", "Julius Ceasar"),
        )?
        .with_assistant("JC".to_string());
    // build a chain, that has the above conversation stored in its state
    let mut chain = Chain::new_with_message_collection(&conversation);
    // Try the Elon Musk problem again
    // Define the step
    let step = Step::for_prompt_and_options(prompt!(user: user_prompt), inv_options);
    // Execute the chain.
    let res = chain.send_message(step, &parameters!().with("full_name", "Elon Musk"), &exec).await?;
    println!(
        "{} (few-shot CoT answer)",
        res.primary_textual_output().await.unwrap()
    );
    Ok(())
}
