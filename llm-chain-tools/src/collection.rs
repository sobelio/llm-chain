use crate::tool::Tool;
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
        let (yaml_start, yaml_end) = match (data.find("```"), data.find("```yaml")) {
            (Some(start), _) => {
                let end = &data[start + 6..];
                match end.find("```") {
                    Some(end_pos) => (start + 6, start + 6 + end_pos),
                    None => return Err("Could not find end of code block".to_string()),
                }
            }
            (_, Some(start)) => {
                let end = &data[start + 7..];
                match end.find("```") {
                    Some(end_pos) => (start + 7, start + 7 + end_pos),
                    None => return Err("Could not find end of code block".to_string()),
                }
            }
            _ => (0, data.len()),
        };

        let yaml_str = &data[yaml_start..yaml_end];
        let yaml_str = yaml_str.trim_start_matches("yaml\n").trim_start();
        let res: serde_yaml::Result<ToolInvocationInput> = serde_yaml::from_str(yaml_str);
        let input = res.map_err(|_e| "INPUT MUST BE YAML ONLY".to_string())?;
        let output = self.invoke(&input.command, &input.input)?;
        Ok(serde_yaml::to_string(&output).unwrap())
    }

    pub fn describe(&self) -> String {
        let des: Vec<_> = self.tools.iter().map(|t| t.description()).collect();
        serde_yaml::to_string(&des).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct ToolInvocationInput {
    command: String,
    input: serde_yaml::Value,
}
