use llm_chain::output::Output;
use llm_chain::prompt::chat::{ChatMessage, ChatPrompt, ChatRole};
use llm_chain::step::Step;
use llm_chain::tools::tools::BashTool;
use llm_chain::tools::ToolCollection;
use llm_chain::PromptTemplate;
use llm_chain::{executor, Parameters};

// A simple example generating a prompt with some tools.

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!()?;

    let mut tool_collection = ToolCollection::new();
    tool_collection.add_tool(BashTool::new());

    let template = PromptTemplate::combine(vec![
        tool_collection.to_prompt_template().unwrap(),
        PromptTemplate::tera("Please perform the following task: {{task}}."),
    ]);

    let task = "Find the file GOAL.txt and tell me its content";

    let prompt = ChatPrompt::builder()
        .system("You are an automated agent for performing tasks. Your output must always be YAML.")
        .add_message(ChatMessage::from_template(ChatRole::User, template))
        .build()
        .unwrap();

    let result = Step::for_prompt(prompt.into())
        .run(&Parameters::new().with("task", task), &exec)
        .await?;

    println!("{}", result);
    match tool_collection.process_chat_input(&result.primary_textual_output().await.unwrap()) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
