use llm_chain::Parameters;
use llm_chain_tools::create_tool_prompt_segment;
use llm_chain_tools::tools::BashTool;
use llm_chain_tools::ToolCollection;
// A simple example generating a prompt with some tools.

fn main() {
    let mut tool_collection = ToolCollection::new();
    tool_collection.add_tool(BashTool::new());
    let prompt =
        create_tool_prompt_segment(&tool_collection, "Please perform the following task: {}");
    println!(
        "{}",
        prompt.format(&Parameters::new_with_text(
            "Find the file GOAL.txt and tell me its content."
        ))
    );
}
