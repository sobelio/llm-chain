use llm_chain::tools::create_tool_prompt_segment;
use llm_chain::tools::tools::BashTool;
use llm_chain::tools::ToolCollection;
use llm_chain::{traits::StepExt, Parameters};
use llm_chain_openai::chatgpt::{Executor, Model, Role, Step};
// A simple example generating a prompt with some tools.

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tool_collection = ToolCollection::new();
    tool_collection.add_tool(BashTool::new());
    let template =
        create_tool_prompt_segment(&tool_collection, "Please perform the following task: {}");
    let prompt = template
        .format(&Parameters::new_with_text(
            "Find the file GOAL.txt and tell me its content.",
        ))
        .unwrap();

    let exec = Executor::new_default();
    let chain = Step::new(
        Model::ChatGPT3_5Turbo,
        [
            (
                Role::System,
                "You are an automated agent for performing tasks. Your output must always be YAML.",
            ),
            (Role::User, &prompt),
        ],
    )
    .to_chain();
    let res = chain.run(Parameters::new(), &exec).await?;
    let message_text = res.choices.first().unwrap().message.content.clone();
    println!("{}", &message_text);
    match tool_collection.process_chat_input(&message_text) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
