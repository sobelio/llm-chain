use llm_chain::tools::tools::BashTool;
use llm_chain::tools::ToolCollection;
use llm_chain::{Parameters, PromptTemplate};
// A simple example generating a prompt with some tools.

fn main() {
    let mut tool_collection = ToolCollection::new();
    tool_collection.add_tool(BashTool::new());
    let prompt = PromptTemplate::combine(vec![
        tool_collection.to_prompt_template(),
        PromptTemplate::tera("Please perform the following task: {{text}}"),
    ]);

    println!(
        "{}",
        prompt
            .format(&Parameters::new_with_text(
                "Find the file GOAL.txt and tell me its content."
            ))
            .unwrap()
    );
}
