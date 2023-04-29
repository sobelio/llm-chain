use std::time::Duration;

use crate::chatgpt::PerInvocation;
use async_trait::async_trait;
use llm_chain::{
    executor,
    output::Output,
    parameters,
    prompt::{Prompt, PromptTemplate},
    step::Step,
    tools::{Format, Tool, ToolCollection, ToolDescription, ToolError, ToolUseError},
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
        ToolDescription {
            name: "Intermediate Answer".into(),
            description: "useful for when tou need to ask with search".into(),
            description_context: "useful for when tou need to ask with search".into(),
            input_format: Format::new(vec![]),
            output_format: Format::new(vec![]),
        }
    }
}

const PROMPT: &str = "Question: Who lived longer, Muhammad Ali or Alan Turing?
Are follow up questions needed here: Yes.
Follow up: How old was Muhammad Ali when he died?
Intermediate answer: Muhammad Ali was 74 years old when he died.
Follow up: How old was Alan Turing when he died?
Intermediate answer: Alan Turing was 41 years old when he died.
So the final answer is: Muhammad Ali

Question: When was the founder of craigslist born?
Are follow up questions needed here: Yes.
Follow up: Who was the founder of craigslist?
Intermediate answer: Craigslist was founded by Craig Newmark.
Follow up: When was Craig Newmark born?
Intermediate answer: Craig Newmark was born on December 6, 1952.
So the final answer is: December 6, 1952

Question: Who was the maternal grandfather of George Washington?
Are follow up questions needed here: Yes.
Follow up: Who was the mother of George Washington?
Intermediate answer: The mother of George Washington was Mary Ball Washington.
Follow up: Who was the father of Mary Ball Washington?
Intermediate answer: The father of Mary Ball Washington was Joseph Ball.
So the final answer is: Joseph Ball

Question: Are both the directors of Jaws and Casino Royale from the same country?
Are follow up questions needed here: Yes.
Follow up: Who is the director of Jaws?
Intermediate answer: The director of Jaws is Steven Spielberg.
Follow up: Where is Steven Spielberg from?
Intermediate answer: The United States.
Follow up: Who is the director of Casino Royale?
Intermediate answer: The director of Casino Royale is Martin Campbell.
Follow up: Where is Martin Campbell from?
Intermediate answer: New Zealand.
So the final answer is: No

Question: {input}
Are followup questions needed here:{agent_scratchpad}";

#[derive(Debug, PartialEq)]
pub struct AgentAction {
    tool: String,
    tool_input: serde_yaml::Value,
    log: String,
}
#[derive(Debug, PartialEq)]
pub struct AgentFinish {
    return_values: Parameters,
    log: String,
}

pub struct AgentIntermediateStep {
    action: AgentAction,
    observation: serde_yaml::Value,
}

pub enum AgentIntermediateStepOutput {
    Step(AgentIntermediateStep),
    Finish(AgentFinish),
}

#[derive(Debug, PartialEq)]
pub enum AgentDecision {
    Action(AgentAction),
    Finish(AgentFinish),
}

pub trait AgentOutputParser {
    fn parse(&self, text: String) -> Result<AgentDecision, MyError>;
}

pub struct DefaultAgentOutputParser;

impl AgentOutputParser for DefaultAgentOutputParser {
    fn parse(&self, text: String) -> Result<AgentDecision, MyError> {
        let followup_prefix = "Follow up:";
        let intermediate_answer_prefix = "Intermediate Answer";
        let finish_prefix = "So the final answer is:";

        let text_clone = text.clone();
        let last_line = text_clone
            .trim()
            .split("\n")
            .last()
            .ok_or(MyError("Empty text".to_owned() + &text.clone()))?
            .trim();
        if last_line.contains(followup_prefix) {
            let raw_follow_up_question = last_line.replace(followup_prefix, "");
            let follow_up_question = raw_follow_up_question.trim();

            Ok(AgentDecision::Action(AgentAction {
                tool: intermediate_answer_prefix.into(),
                tool_input: follow_up_question.into(),
                log: text,
            }))
        } else if last_line.contains(finish_prefix) {
            let raw_final_answer = last_line.replace(finish_prefix, "");
            let final_answer = raw_final_answer.trim();
            Ok(AgentDecision::Finish(AgentFinish {
                return_values: parameters!("output" => final_answer),
                log: text,
            }))
        } else {
            Err(MyError(
                "Neither finish line nor follow up line is last".to_owned() + &text,
            ))
        }
    }
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
    executor: crate::chatgpt::Executor,
    tools: ToolCollection<SearchTool>,
    early_stopping_config: EarlyStoppingConfig,
    observation_prefix: String,
    llm_prefix: String,
    output_parser: DefaultAgentOutputParser,
}

impl Agent {
    pub fn new(
        executor: crate::chatgpt::Executor,
        tools: ToolCollection<SearchTool>,
        early_stopping_config: EarlyStoppingConfig,
    ) -> Self {
        Self {
            executor,
            tools,
            early_stopping_config,
            observation_prefix: "Intermediate answer: ".to_string(),
            llm_prefix: "".to_string(),
            output_parser: DefaultAgentOutputParser,
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
        query: &str,
    ) -> Result<AgentIntermediateStepOutput, MyError> {
        let output = self.plan(intermediate_steps, query).await?;

        let decision = self.output_parser.parse(output)?;

        match decision {
            AgentDecision::Action(action) => {
                let observation = self
                    .tools
                    .invoke(&action.tool, &action.tool_input)
                    .await
                    .map_err(|_| MyError("Bad tool invocation".into()))?;

                Ok(AgentIntermediateStepOutput::Step(AgentIntermediateStep {
                    action,
                    observation,
                }))
            }
            AgentDecision::Finish(finish) => Ok(AgentIntermediateStepOutput::Finish(finish)),
        }
    }

    /// Convert the intermediate steps into a single text to pass to the agent so he can continue his thought process
    fn build_agent_scratchpad(&self, intermediate_steps: &Vec<AgentIntermediateStep>) -> String {
        let mut scratchpad = "".to_string();
        for intermediate_step in intermediate_steps {
            scratchpad += &intermediate_step.action.log;
            scratchpad += &format!(
                "\n{}{}\n{}",
                self.observation_prefix,
                intermediate_step.observation.as_str().unwrap_or_default(),
                self.llm_prefix
            );
        }
        scratchpad
    }

    /// Ask a model for a decision on what to do next, e.x. which tool to use
    ///
    /// Fills in the prompt template then calls the model to complete it
    async fn plan(
        &self,
        intermediate_steps: &Vec<AgentIntermediateStep>,
        query: &str,
    ) -> Result<String, MyError> {
        let scratchpad = self.build_agent_scratchpad(intermediate_steps);
        let template_parameters = parameters!("input" => query, "agent_scratchpad" => scratchpad);
        let prompt = PromptTemplate::Text(PROMPT.into())
            .format(&template_parameters)
            .map_err(|_| MyError("Template formatting error".into()))?;
        let plan = self
            .executor
            .execute(None, &prompt)
            .await
            .map_err(|_| MyError("Error while getting a response from the model".into()))?;
        plan.primary_textual_output().await.ok_or(MyError(
            "Could not get text output from model response".into(),
        ))
    }

    pub async fn run(&self, query: &str) -> Result<AgentFinish, MyError> {
        let mut intermediate_steps = vec![];

        let mut iterations = 0;
        let start = Instant::now();
        let mut full_duration = Duration::from_nanos(0);
        while self.should_continue(iterations, full_duration.as_secs_f64()) {
            let decision = self.take_next_step(&intermediate_steps, query).await?;
            full_duration = start.elapsed();
            iterations += 1;
            match decision {
                AgentIntermediateStepOutput::Step(step) => intermediate_steps.push(step),
                AgentIntermediateStepOutput::Finish(finish) => return Ok(finish),
            }
        }
        Err(MyError(
            "Agent exceeded max time or number of iterations".into(),
        ))
    }
}

#[cfg(test)]
mod tests {

    use llm_chain::parameters;

    use super::{
        AgentAction, AgentDecision, AgentFinish, AgentOutputParser, DefaultAgentOutputParser,
    };

    #[test]
    fn test_parses_followup() {
        let parser = DefaultAgentOutputParser;
        let text = "
        Whatever
        Whatever
        Follow up: my follow up question abc?";
        let decision = parser.parse(text.into()).unwrap();
        assert_eq!(
            decision,
            AgentDecision::Action(AgentAction {
                tool: "Intermediate Answer".into(),
                tool_input: "my follow up question abc?".into(),
                log: text.into()
            })
        );
    }

    #[test]
    fn test_parses_follow_up_ignores_trailing_whitespace() {
        let parser = DefaultAgentOutputParser;
        let text = "
        Whatever
        Whatever
        Follow up: my follow up question abc?
        ";
        let decision = parser.parse(text.into()).unwrap();
        assert_eq!(
            decision,
            AgentDecision::Action(AgentAction {
                tool: "Intermediate Answer".into(),
                tool_input: "my follow up question abc?".into(),
                log: text.into()
            })
        );
    }

    #[test]
    fn test_parses_final_answer() {
        let parser = DefaultAgentOutputParser;
        let text = "
        Whatever
        Whatever
        So the final answer is: yes abc!";
        let decision = parser.parse(text.into()).unwrap();
        assert_eq!(
            decision,
            AgentDecision::Finish(AgentFinish {
                return_values: parameters!("output" => "yes abc!"),
                log: text.into()
            })
        );
    }

    #[test]
    fn test_parses_final_answer_ignores_trailing_whitespace() {
        let parser = DefaultAgentOutputParser;
        let text = "
        Whatever
        Whatever
        So the final answer is: yes abc!
        ";
        let decision = parser.parse(text.into()).unwrap();
        assert_eq!(
            decision,
            AgentDecision::Finish(AgentFinish {
                return_values: parameters!("output" => "yes abc!"),
                log: text.into()
            })
        );
    }

    #[test]
    fn test_parses_final_answer_with_colons() {
        let parser = DefaultAgentOutputParser;
        let text = "
        Whatever
        Whatever
        So the final answer is: Mad Max: Fury road";
        let decision = parser.parse(text.into()).unwrap();
        assert_eq!(
            decision,
            AgentDecision::Finish(AgentFinish {
                return_values: parameters!("output" => "Mad Max: Fury road"),
                log: text.into()
            })
        );
    }

    // fn test_agent() {
    //     let executor = executor!().unwrap();
    //     let search_tool = SearchTool;
    //     let mut tools = ToolCollection::<SearchTool>::new();
    //     tools.add_tool(search_tool);
    //     let agent = Agent::new(
    //         executor,
    //         tools,
    //         EarlyStoppingConfig {
    //             max_iterations: Some(15),
    //             max_time_elapsed_seconds: Some(15.0),
    //         },
    //     );
    //     let res = agent
    //         .run("Give me a birth date of an associate of the inventor of Catan")
    //         .await
    //         .unwrap();
    //     println!("Agent response: {}", res.return_values.get_text().unwrap());
    // }
}
