use llm_chain::Parameters;
use llm_chain_tools::create_tool_prompt_segment;
use llm_chain_tools::tools::BashTool;
use llm_chain_tools::ToolCollection;
use std::boxed::Box;
// A simple example generating a prompt with some tools.

fn main() {
    let tool_collection = ToolCollection::new(vec![Box::new(BashTool::new())]);
    let prompt =
        create_tool_prompt_segment(&tool_collection, "Please perform the following task: {}");
    println!(
        "{}",
        prompt.format(&Parameters::new_with_text(
            "Find the file GOAL.txt and tell me its content."
        ))
    );
}
