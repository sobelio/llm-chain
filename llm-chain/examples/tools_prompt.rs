use llm_chain::prompt::StringTemplate;
use llm_chain::tools::tools::BashTool;
use llm_chain::tools::ToolCollection;
use llm_chain::Parameters;
// A simple example generating a prompt with some tools.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tool_collection = ToolCollection::new();
    tool_collection.add_tool(BashTool::new());

    let prompt = StringTemplate::combine(vec![
        tool_collection.to_prompt_template().unwrap(),
        StringTemplate::tera("Please perform the following task: {{text}}"),
    ]);

    println!(
        "{}",
        prompt
            .format(&Parameters::new_with_text(
                "Find the file GOAL.txt and tell me its content."
            ))
            .unwrap()
    );
    Ok(())
}
