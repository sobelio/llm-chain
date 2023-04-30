use llm_chain::prompt;
use llm_chain::serialization::StorableEntity;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_prompt = prompt!(
        "You are a robot assistant for making personalized greetings.",
        "Make a personalized greeting for Joe."
    );
    println!("{}", my_prompt);

    let prompt = my_prompt.write_file_sync("my_prompt.json")?;

    Ok(())
}
