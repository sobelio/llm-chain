use crate::description::ToolDescription;

macro_rules! gen_invoke_function {
    () => {
        fn invoke(&self, input: serde_yaml::Value) -> Result<serde_yaml::Value, String> {
            let input = serde_yaml::from_value(input).unwrap();
            let output = self.invoke_typed(&input).unwrap();
            Ok(serde_yaml::to_value(output).unwrap())
        }
    };
}
pub(crate) use gen_invoke_function;

pub trait Tool {
    fn description(&self) -> ToolDescription;
    fn invoke(&self, input: serde_yaml::Value) -> Result<serde_yaml::Value, String>;
    fn matches(&self, name: &str) -> bool {
        self.description().name == name
    }
}
