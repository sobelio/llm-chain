use async_trait::async_trait;
use llm_chain::{
    executor,
    step::Step,
    tools::{Tool, ToolCollection, ToolError},
    Parameters,
};
use llm_chain_openai::chatgpt::Executor;
use thiserror::Error;

pub struct SearchTool;

#[derive(Debug, Error)]
#[error("My error occurred")]
pub struct MyError(String);

impl ToolError for MyError {}

impl From<serde_yaml::Error> for MyError {
    fn from(value: serde_yaml::Error) -> Self {
        Self("From yaml error occurred".into())
    }
}

#[async_trait]
impl Tool for SearchTool {
    type Input = String;

    type Output = String;

    type Error = MyError;

    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(input.into())
    }

    fn description(&self) -> llm_chain::tools::ToolDescription {
        todo!()
    }
}

pub struct AgentAction {
    tool: String,
    tool_input: serde_yaml::Value,
}
pub struct AgentFinish {
    return_values: Parameters,
    intermediate_steps: Vec<AgentIntermediateStep>,
}

pub struct AgentIntermediateStep {
    action: AgentAction,
    observation: serde_yaml::Value,
}

pub enum AgentIntermediateStepOutput {
    Step(AgentIntermediateStep),
    Finish(AgentFinish),
}

pub enum AgentDecision {
    Action(AgentAction),
    Finish(AgentFinish),
}

pub struct Agent {
    executor: Executor,
    search_tool: SearchTool,
}

impl Agent {
    fn should_continue(&self) -> bool {
        todo!()
    }

    /// Ask a model for a decision on what to do next, e.x. which tool to use
    ///
    /// Perform the action
    async fn take_next_step(
        &self,
        intermediate_steps: &Vec<AgentIntermediateStep>,
        inputs: &Parameters,
    ) -> Result<AgentIntermediateStepOutput, MyError> {
        let decision = self.plan(intermediate_steps, inputs).await;
        Ok(match decision {
            AgentDecision::Action(action) => {
                AgentIntermediateStepOutput::Step(AgentIntermediateStep {
                    observation: self.search_tool.invoke(action.tool_input.clone()).await?,
                    action,
                })
            }
            AgentDecision::Finish(finish) => AgentIntermediateStepOutput::Finish(finish),
        })
    }

    /// Ask a model for a decision on what to do next, e.x. which tool to use
    async fn plan(
        &self,
        intermediate_steps: &Vec<AgentIntermediateStep>,
        inputs: &Parameters,
    ) -> AgentDecision {
        todo!()
    }

    pub async fn run(&self, query: &str) -> AgentFinish {
        let inputs = Parameters::new(); // TODO
        let mut intermediate_steps = vec![];
        while self.should_continue() {
            let decision = self.take_next_step(&intermediate_steps, &inputs).await;
            match decision {
                AgentIntermediateStepOutput::Step(step) => intermediate_steps.push(step),
                AgentIntermediateStepOutput::Finish(finish) => return finish,
            }
        }
        AgentFinish {
            return_values: Parameters::new(),
            intermediate_steps,
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let executor = executor!().unwrap();
    let search_tool = SearchTool;
    let agent = Agent {
        executor,
        search_tool,
    };
    let res = agent
        .run("Give me a birth date of an associate of the inventor of Catan")
        .await;
    println!("Agent response: {}", res);
}
