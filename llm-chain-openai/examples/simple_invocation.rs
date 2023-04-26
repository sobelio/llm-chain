use llm_chain::executor;
use llm_chain::output::Output;
use llm_chain::parameters;

use llm_chain::prompt::{ChatMessageCollection, StringTemplate};
use llm_chain::step::Step;
use llm_chain::tools::tools::BashTool;
use llm_chain::tools::ToolCollection;

// A simple example generating a prompt with some tools.

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!()?;

    let mut tool_collection = ToolCollection::new();
    tool_collection.add_tool(BashTool::new());

    let template = StringTemplate::combine(vec![
        tool_collection.to_prompt_template().unwrap(),
        StringTemplate::tera("Please perform the following task: {{task}}."),
    ]);

    let task = "Find the file GOAL.txt and tell me its content";

    let prompt = ChatMessageCollection::new()
        .with_system_template(
            "You are an automated agent for performing tasks. Your output must always be YAML.",
        )
        .with_user(template);

    let result = Step::for_prompt_template(prompt.into())
        .run(&parameters!("task" => task), &exec)
        .await?;

    println!("{}", result);
    match tool_collection
        .process_chat_input(&result.primary_textual_output().await.unwrap())
        .await
    {
        Ok(output) => println!("{}", output),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
