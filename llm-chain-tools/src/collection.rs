use crate::tool::Tool;
use llm_chain::parsing::find_yaml;
use serde::{Deserialize, Serialize};

pub struct ToolCollection {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolCollection {
    pub fn new(tools: Vec<Box<dyn Tool>>) -> Self {
        Self { tools }
    }
    pub fn invoke(
        &self,
        name: &str,
        input: &serde_yaml::Value,
    ) -> Result<serde_yaml::Value, String> {
        let tool = self
            .tools
            .iter()
            .find(|t| t.matches(name))
            .ok_or("Tool not found")?;
        tool.invoke(input.clone())
    }
    pub fn process_chat_input(&self, data: &str) -> Result<String, String> {
        let tool_invocations: Vec<ToolInvocationInput> = find_yaml::<ToolInvocationInput>(data)
            .map_err(|e| format!("You must output YAML: {}", e))?;
        if tool_invocations.len() != 1 {
            println!("{:?}", tool_invocations);
            return Err("You must output exactly one tool invocation".to_string());
        }
        let output = self.invoke(&tool_invocations[0].command, &tool_invocations[0].input)?;
        Ok(serde_yaml::to_string(&output).unwrap())
    }

    pub fn describe(&self) -> String {
        let des: Vec<_> = self.tools.iter().map(|t| t.description()).collect();
        serde_yaml::to_string(&des).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ToolInvocationInput {
    command: String,
    input: serde_yaml::Value,
}
