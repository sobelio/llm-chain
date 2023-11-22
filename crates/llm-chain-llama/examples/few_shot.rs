use llm_chain::prompt::Conversation;
use llm_chain::{chains::conversation::Chain, executor, parameters, prompt, step::Step};
/// This example demonstrates how to use the llm-chain for few-shot prompting
///
/// This example can be seen as a "chain of thought"
///
/// Usage: cargo run --example few_shot
///
/// Make sure to have the env var 'LLM_CHAIN_MODEL' set
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec_1 = executor!(llama)?;

    let user_prompt =
        "Take the last letters of the words in '{{ full_name }}' and concatenate them";
    let res = Step::for_prompt_template(prompt!(user: user_prompt))
        .run(&parameters!().with("full_name", "Elon Musk"), &exec_1)
        .await?;
    println!("{} (zero-shot answer)", res.to_immediate().await?); // probably not correct
    let conversation = Conversation::new()
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
        .with_assistant("JC".to_string())
        .with_user_template(user_prompt, &parameters!().with("full_name", "Ding Liren"))?
        .with_assistant("DL".to_string());
    // build a chain, that has the above conversation stored in its state
    let mut chain = Chain::new_with_message_collection(&conversation);
    // Try the Elon Musk problem again
    // Define the step
    let step = Step::for_prompt_template(prompt!(user: user_prompt));
    // Execute the chain.
    let exec_2 = executor!(llama)?;
    let res = chain
        .send_message(step, &parameters!().with("full_name", "Elon Musk"), &exec_2)
        .await?;
    println!("{} (few-shot CoT answer)", res.to_immediate().await?);
    Ok(())
}
