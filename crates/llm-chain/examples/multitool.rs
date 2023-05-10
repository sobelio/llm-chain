use async_trait::async_trait;
use llm_chain::{
    multitool,
    tools::{
        tools::{BashTool, BashToolError, BashToolInput, BashToolOutput},
        Format, Tool, ToolError,
    },
    tools::{ToolCollection, ToolDescription},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Your custom tool's implementation:
#[derive(Debug, Error)]
#[error("MyTool custom error")]
struct MyToolError(#[from] serde_yaml::Error);

impl ToolError for MyToolError {}

#[derive(Serialize, Deserialize)]
struct MyToolInput(());
#[derive(Serialize, Deserialize)]
struct MyToolOutput(());

struct MyTool {}

#[async_trait]
impl Tool for MyTool {
    type Input = MyToolInput;
    type Output = MyToolOutput;
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

    async fn invoke(&self, _: serde_yaml::Value) -> Result<serde_yaml::Value, Self::Error> {
        todo!()
    }

    async fn invoke_typed(&self, _: &Self::Input) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

// Final toolbox Tool type:
multitool!(
    Multitool,
    MultiToolInput,
    MultiToolOutput,
    MultitoolError,
    BashTool,
    BashToolInput,
    BashToolOutput,
    BashToolError,
    MyTool,
    MyToolInput,
    MyToolOutput,
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
