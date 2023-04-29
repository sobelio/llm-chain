use std::time::Duration;

use async_trait::async_trait;
use llm_chain::{
    executor,
    output::Output,
    prompt::Prompt,
    step::Step,
    tools::{Tool, ToolCollection, ToolError, ToolUseError},
    traits::Executor,
    Parameters,
};
use thiserror::Error;
use tokio::time::Instant;

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
    log: String,
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

pub struct EarlyStoppingConfig {
    max_iterations: Option<u32>,
    max_time_elapsed_seconds: Option<f64>,
}

impl Default for EarlyStoppingConfig {
    fn default() -> Self {
        Self {
            max_iterations: Default::default(),
            max_time_elapsed_seconds: Default::default(),
        }
    }
}

pub struct Agent {
    executor: llm_chain_openai::chatgpt::Executor,
    tools: ToolCollection<SearchTool>,
    early_stopping_config: EarlyStoppingConfig,
    observation_prefix: String,
    llm_prefix: String,
}

impl Agent {
    pub fn new(
        executor: llm_chain_openai::chatgpt::Executor,
        tools: ToolCollection<SearchTool>,
        early_stopping_config: EarlyStoppingConfig,
    ) -> Self {
        Self {
            executor,
            tools,
            early_stopping_config,
            observation_prefix: "Intermediate answer: ".to_string(),
            llm_prefix: "".to_string(),
        }
    }

    fn should_continue(&self, iterations_elapsed: u32, time_elapsed_seconds: f64) -> bool {
        match (
            self.early_stopping_config.max_iterations,
            self.early_stopping_config.max_time_elapsed_seconds,
        ) {
            (None, None) => true,
            (None, Some(max_time_elapsed_seconds)) => {
                max_time_elapsed_seconds >= time_elapsed_seconds
            }
            (Some(max_iterations), None) => max_iterations >= iterations_elapsed,
            (Some(max_iterations), Some(max_time_elapsed_seconds)) => {
                max_iterations >= iterations_elapsed
                    && max_time_elapsed_seconds >= time_elapsed_seconds
            }
        }
    }

    /// Ask a model for a decision on what to do next, e.x. which tool to use
    ///
    /// Perform the action
    async fn take_next_step(
        &self,
        intermediate_steps: &Vec<AgentIntermediateStep>,
        inputs: &Parameters,
    ) -> Result<AgentIntermediateStepOutput, MyError> {
        let decision = self.plan(intermediate_steps, inputs).await?;
        // TODO: Check if the model is finished

        let tool_invocation = self
            .tools
            .get_tool_invocation(&decision)
            .map_err(|_| MyError("Bad tool invocation".into()))?;
        let observation = self
            .tools
            .invoke(&tool_invocation.command, &tool_invocation.input)
            .await
            .map_err(|_| MyError("Tool invocation error".into()))?;
        Ok(AgentIntermediateStepOutput::Step(AgentIntermediateStep {
            action: AgentAction {
                tool: tool_invocation.command,
                tool_input: tool_invocation.input,
                log: decision,
            },
            observation,
        }))

        // Ok(match decision {
        //     AgentDecision::Action(action) => {
        //         AgentIntermediateStepOutput::Step(AgentIntermediateStep {
        //             observation: self
        //                 .tools
        //                 .invoke(&action.tool, &action.tool_input)
        //                 .await
        //                 .map_err(|_| MyError("Search tool error".into()))?,
        //             action,
        //         })
        //     }
        //     AgentDecision::Finish(finish) => AgentIntermediateStepOutput::Finish(finish),
        // })
    }

    /// Convert the intermediate steps into a single text to pass to the agent so he can continue his thought process
    fn stringify_thought_process(&self, intermediate_steps: &Vec<AgentIntermediateStep>) -> String {
        let mut thoughts = "".to_string();
        for intermediate_step in intermediate_steps {
            thoughts += &intermediate_step.action.log;
            thoughts += &format!(
                "\n{}{}\n{}",
                self.observation_prefix,
                intermediate_step.observation.as_str().unwrap_or_default(),
                self.llm_prefix
            );
        }
        thoughts
    }

    /// Ask a model for a decision on what to do next, e.x. which tool to use
    async fn plan(
        &self,
        intermediate_steps: &Vec<AgentIntermediateStep>,
        inputs: &Parameters,
    ) -> Result<String, MyError> {
        let thoughts = self.stringify_thought_process(intermediate_steps);
        let plan = self
            .executor
            .execute(None, &Prompt::Text(thoughts))
            .await
            .map_err(|_| MyError("Error while getting a response from the model".into()))?;
        plan.primary_textual_output().await.ok_or(MyError(
            "Could not get text output from model response".into(),
        ))
    }

    pub async fn run(&self, query: &str) -> Result<AgentFinish, MyError> {
        let inputs = Parameters::new_with_text(query); // TODO
        let mut intermediate_steps = vec![];

        let mut iterations = 0;
        let mut time_elapsed_seconds = 0.0;
        let start = Instant::now();
        let mut full_duration = Duration::from_nanos(0);
        while self.should_continue(iterations, time_elapsed_seconds) {
            let decision = self.take_next_step(&intermediate_steps, &inputs).await?;
            full_duration = start.elapsed();
            time_elapsed_seconds = full_duration.as_secs_f64();
            iterations += 1;
            match decision {
                AgentIntermediateStepOutput::Step(step) => intermediate_steps.push(step),
                AgentIntermediateStepOutput::Finish(finish) => return Ok(finish),
            }
        }
        Ok(AgentFinish {
            return_values: Parameters::new(),
            intermediate_steps,
        })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let executor = executor!().unwrap();
    let search_tool = SearchTool;
    let mut tools = ToolCollection::<SearchTool>::new();
    tools.add_tool(search_tool);
    let agent = Agent::new(
        executor,
        tools,
        EarlyStoppingConfig {
            max_iterations: Some(15),
            max_time_elapsed_seconds: Some(15.0),
        },
    );
    let res = agent
        .run("Give me a birth date of an associate of the inventor of Catan")
        .await
        .unwrap();
    println!("Agent response: {}", res.return_values.get_text().unwrap());
}
