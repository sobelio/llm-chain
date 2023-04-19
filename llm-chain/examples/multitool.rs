use llm_chain::{
    multitool,
    tools::{
        tools::{BashTool, BashToolError},
        Format, Tool, ToolError,
    },
    tools::{ToolCollection, ToolDescription},
};
use thiserror::Error;

/// Your custom tool's implementation:
#[derive(Debug, Error)]
#[error("MyTool custom error")]
struct MyToolError(#[from] serde_yaml::Error);

impl ToolError for MyToolError {}

struct MyTool {}

impl Tool for MyTool {
    type Error = MyToolError;

    fn description(&self) -> ToolDescription {
        ToolDescription {
            name: "MyTool".into(),
            description: "My custom implementation of a tool".into(),
            description_context: "You are able to use my tool".into(),
            input_format: Format::new(vec![]),
            output_format: Format::new(vec![]),
        }
    }

    fn invoke(&self, _: serde_yaml::Value) -> Result<serde_yaml::Value, Self::Error> {
        Ok(serde_yaml::Value::Null)
    }
}

// Final toolbox Tool type:
multitool!(
    Multitool,
    MultitoolError,
    BashTool,
    BashToolError,
    MyTool,
    MyToolError
);

fn main() {
    // Calling tool methods on the toolbox enum
    let tool = BashTool {};
    let my_tool = MyTool {};

    println!("Original tool: {:?}", tool.description());
    let toolbox1 = Multitool::BashTool(tool);
    println!("Multitool description: {:?}", toolbox1.description());

    println!("Original tool: {:?}", my_tool.description());
    let toolbox2 = Multitool::MyTool(my_tool);
    println!("Multitool description: {:?}", toolbox2.description());

    // Adding tools (as multitools) to a ToolCollection
    let tool = BashTool {};
    let my_tool = MyTool {};

    let mut collection = ToolCollection::<Multitool>::new();
    collection.add_tool(tool.into());
    collection.add_tool(my_tool.into());

    println!("{}", collection.describe().unwrap());
}
