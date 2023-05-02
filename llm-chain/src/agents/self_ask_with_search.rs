use crate::{
    output::Output,
    parameters,
    prompt::{PromptTemplate, StringTemplateError},
    tools::{Tool, ToolError},
    traits::{Executor, ExecutorError},
    Parameters,
};
use std::time::{Duration, Instant};
use thiserror::Error;

/// TODO: This prompt has some issues:
///
/// - models do not always format their output correctly, e.x. respond with "So the final answer could be: ..." instead of "So the final answer is: ..."
/// - some models have safety measures against asking about events which are in the future (from the point of view of the model); they will not even attempt to use the search tool
/// - models sometimes finish on "Intermediate answer: ..." if it contains the final answer to the question
/// - models sometimes immediately answer with "Yes, ..." or "No, ..."; they should always structure their final answer with "So the final answer is: ..." (or equivalent)
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

Question: {{input}}
Are followup questions needed here:{{agent_scratchpad}}";

#[derive(Debug, PartialEq)]
pub struct AgentAction {
    pub tool: String,
    pub tool_input: serde_yaml::Value,
    pub log: String,
}
#[derive(Debug, PartialEq)]
pub struct AgentFinish {
    pub return_values: Parameters,
    pub log: String,
}

#[derive(Debug)]
pub struct AgentIntermediateStep {
    pub action: AgentAction,
    pub observation: serde_yaml::Value,
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
    type Error;
    fn parse(&self, text: String) -> Result<AgentDecision, Self::Error>;
}

#[derive(Debug, Error)]
pub enum SelfAskWithSearchAgentError<T, E>
where
    T: std::fmt::Debug + std::error::Error + ToolError,
    E: std::fmt::Debug + std::error::Error + ExecutorError,
{
    #[error("Search tool input yaml was not of type string: {0:?}")]
    ToolInputNotString(serde_yaml::Value),
    #[error(transparent)]
    SearchToolError(T),
    #[error(transparent)]
    ExecutorError(E),
    #[error(transparent)]
    ParserError(#[from] ParserError),
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    StringTemplateError(#[from] StringTemplateError),
    #[error("Model response was empty or contained no choices")]
    NoChoicesReturned,
    #[error("Max number of iterations or timeout exceeded. Elapsed: {time_elapsed_seconds}s, {iterations_elapsed} iterations")]
    RuntimeExceeded {
        time_elapsed_seconds: f64,
        iterations_elapsed: u32,
    },
}

pub struct SelfAskWithSearchAgentOutputParser {
    followup_prefix: String,
    intermediate_answer_prefix: String,
    acceptable_finish_prefixes: Vec<String>,
}

impl SelfAskWithSearchAgentOutputParser {
    pub fn new(
        followup_prefix: &str,
        intermediate_answer_prefix: &str,
        acceptable_finish_prefixes: &[&str],
    ) -> Self {
        Self {
            followup_prefix: followup_prefix.into(),
            intermediate_answer_prefix: intermediate_answer_prefix.into(),
            acceptable_finish_prefixes: acceptable_finish_prefixes
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

impl Default for SelfAskWithSearchAgentOutputParser {
    fn default() -> Self {
        Self::new(
            "Follow up:",
            "Intermediate Answer:",
            &[
                "Final answer:",
                "So the final answer is:",
                "So the final answer could be:",
            ],
        )
    }
}

#[derive(Debug, Error)]
#[error("No finish line or follow up question was returned by the model: {0}")]
pub struct ParserError(String);

impl AgentOutputParser for SelfAskWithSearchAgentOutputParser {
    type Error = ParserError;
    fn parse(&self, text: String) -> Result<AgentDecision, Self::Error> {
        if let Some(followup_idx) = text.find(&self.followup_prefix) {
            let (followup_question, log) = if let Some(intermediate_answer_idx) =
                text.find(&self.intermediate_answer_prefix)
            {
                let followup_question = text
                    .chars()
                    .skip(followup_idx + self.followup_prefix.len())
                    .take(intermediate_answer_idx - (followup_idx + self.followup_prefix.len()))
                    .collect::<String>()
                    .trim()
                    .to_owned();

                let log = text.chars().take(intermediate_answer_idx).collect();
                (followup_question, log)
            } else {
                let followup_question = text
                    .chars()
                    .skip(followup_idx + self.followup_prefix.len())
                    .take_while(|&c| c != '\n')
                    .collect::<String>()
                    .trim()
                    .to_owned();

                let log = text
                    .char_indices()
                    .map_while(|(idx, c)| {
                        if c != '\n' || idx < followup_idx {
                            Some(c)
                        } else {
                            None
                        }
                    })
                    .collect();
                (followup_question, log)
            };
            Ok(AgentDecision::Action(AgentAction {
                tool: "Intermediate Answer".into(),
                tool_input: followup_question.into(),
                log,
            }))
        } else if let Some((idx, prefix)) = self
            .acceptable_finish_prefixes
            .iter()
            .find_map(|prefix| text.find(prefix).map(|idx| (idx, prefix)))
        {
            let final_answer = text.chars().skip(idx + prefix.len()).collect::<String>();
            Ok(AgentDecision::Finish(AgentFinish {
                return_values: parameters!("output" => final_answer.trim()),
                log: text,
            }))
        } else {
            Err(ParserError(text))
        }
    }
}

pub struct EarlyStoppingConfig {
    pub max_iterations: Option<u32>,
    pub max_time_elapsed_seconds: Option<f64>,
}

impl Default for EarlyStoppingConfig {
    fn default() -> Self {
        Self {
            max_iterations: Default::default(),
            max_time_elapsed_seconds: Default::default(),
        }
    }
}

pub struct Agent<E, T>
where
    E: Executor,
    T: Tool,
    T::Input: From<String>,
    T::Output: Into<String>,
{
    executor: E,
    search_tool: T,
    early_stopping_config: EarlyStoppingConfig,
    observation_prefix: String,
    llm_prefix: String,
    output_parser: SelfAskWithSearchAgentOutputParser,
}

impl<E, T> Agent<E, T>
where
    E: Executor,
    T: Tool,
    T::Input: From<String>,
    T::Output: Into<String>,
{
    pub fn new(executor: E, search_tool: T, early_stopping_config: EarlyStoppingConfig) -> Self {
        Self {
            executor,
            search_tool,
            early_stopping_config,
            observation_prefix: "Intermediate answer: ".to_string(),
            llm_prefix: "".to_string(),
            output_parser: SelfAskWithSearchAgentOutputParser::default(),
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
    ) -> Result<
        AgentIntermediateStepOutput,
        SelfAskWithSearchAgentError<<T as Tool>::Error, <E as Executor>::Error>,
    > {
        let output = self.plan(intermediate_steps, query).await?;

        let decision = self.output_parser.parse(output)?;
        match decision {
            AgentDecision::Action(action) => {
                let observation = self
                    .search_tool
                    .invoke_typed(
                        &action
                            .tool_input
                            .as_str()
                            .ok_or(SelfAskWithSearchAgentError::ToolInputNotString(
                                action.tool_input.clone(),
                            ))?
                            .to_string()
                            .into(),
                    )
                    .await
                    .map_err(SelfAskWithSearchAgentError::SearchToolError)?;

                Ok(AgentIntermediateStepOutput::Step(AgentIntermediateStep {
                    action,
                    observation: serde_yaml::to_value(Into::<String>::into(observation))?,
                }))
            }
            AgentDecision::Finish(finish) => Ok(AgentIntermediateStepOutput::Finish(finish)),
        }
    }

    /// Convert the intermediate steps into a single text to pass to the agent so he can continue his thought process
    pub fn build_agent_scratchpad(
        &self,
        intermediate_steps: &Vec<AgentIntermediateStep>,
    ) -> String {
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
    ) -> Result<String, SelfAskWithSearchAgentError<<T as Tool>::Error, <E as Executor>::Error>>
    {
        let scratchpad = self.build_agent_scratchpad(intermediate_steps);
        let template_parameters = parameters!("input" => query, "agent_scratchpad" => scratchpad);
        let prompt = PromptTemplate::Text(PROMPT.into()).format(&template_parameters)?;
        let plan = self
            .executor
            .execute(None, &prompt)
            .await
            .map_err(SelfAskWithSearchAgentError::ExecutorError)?;
        plan.primary_textual_output()
            .await
            .ok_or(SelfAskWithSearchAgentError::NoChoicesReturned)
    }

    pub async fn run(
        &self,
        query: &str,
    ) -> Result<
        (AgentFinish, Vec<AgentIntermediateStep>),
        SelfAskWithSearchAgentError<<T as Tool>::Error, <E as Executor>::Error>,
    > {
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
                AgentIntermediateStepOutput::Finish(finish) => {
                    return Ok((finish, intermediate_steps))
                }
            }
        }
        Err(SelfAskWithSearchAgentError::RuntimeExceeded {
            time_elapsed_seconds: full_duration.as_secs_f64(),
            iterations_elapsed: iterations,
        })
    }
}

#[cfg(test)]
mod tests {

    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    use crate::{
        agents::self_ask_with_search::{AgentIntermediateStep, EarlyStoppingConfig},
        output::Output,
        parameters,
        tokens::Tokenizer,
        tools::{Tool, ToolError},
        traits::{Executor, ExecutorError, Options},
        TextSplitter,
    };

    use super::{
        Agent, AgentAction, AgentDecision, AgentFinish, AgentOutputParser,
        SelfAskWithSearchAgentOutputParser,
    };

    #[test]
    fn test_parses_followup() {
        let parser = SelfAskWithSearchAgentOutputParser::default();
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
    fn test_parses_follow_up_trims_trailing_whitespace() {
        let parser = SelfAskWithSearchAgentOutputParser::default();
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
                log: text.trim_end().into()
            })
        );
    }

    #[test]
    fn test_parses_final_answer() {
        let parser = SelfAskWithSearchAgentOutputParser::default();
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
        let parser = SelfAskWithSearchAgentOutputParser::default();
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
        let parser = SelfAskWithSearchAgentOutputParser::default();
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

    #[test]
    fn test_builds_agent_sratchpad() {
        #[derive(Debug, Serialize, Deserialize, Clone)]
        struct MockOptions;

        impl Options for MockOptions {}

        #[derive(Clone)]
        struct MockOutput;

        #[async_trait]
        impl Output for MockOutput {
            async fn primary_textual_output_choices(&self) -> Vec<String> {
                todo!()
            }
        }

        #[derive(Debug, Error)]
        #[error("Mocked executor error")]
        struct MockError;

        impl ExecutorError for MockError {}

        impl ToolError for MockError {}

        impl From<serde_yaml::Error> for MockError {
            fn from(_: serde_yaml::Error) -> Self {
                Self
            }
        }

        struct MockTextSplitter;

        impl<T: Clone> Tokenizer<T> for MockTextSplitter {
            fn tokenize_str(&self, _: &str) -> Result<Vec<T>, crate::tokens::TokenizerError> {
                todo!()
            }

            fn to_string(&self, _: Vec<T>) -> Result<String, crate::tokens::TokenizerError> {
                todo!()
            }
        }

        impl<T: Clone> TextSplitter<T> for MockTextSplitter {
            fn split_text(
                &self,
                _: &str,
                _: usize,
                _: usize,
            ) -> Result<Vec<String>, crate::tokens::TokenizerError> {
                todo!()
            }
        }

        struct MockExecutor;

        #[async_trait]
        impl Executor for MockExecutor {
            type PerInvocationOptions = MockOptions;

            type PerExecutorOptions = MockOptions;

            type Output = MockOutput;

            type Error = MockError;

            type Token = ();

            type StepTokenizer<'a> = MockTextSplitter;

            type TextSplitter<'a> = MockTextSplitter;

            fn new_with_options(
                _: Option<Self::PerExecutorOptions>,
                _: Option<Self::PerInvocationOptions>,
            ) -> Result<Self, crate::traits::ExecutorCreationError> {
                todo!()
            }

            async fn execute(
                &self,
                _: Option<&Self::PerInvocationOptions>,
                _: &crate::prompt::Prompt,
            ) -> Result<Self::Output, Self::Error> {
                todo!()
            }

            fn tokens_used(
                &self,
                _: Option<&Self::PerInvocationOptions>,
                _: &crate::prompt::Prompt,
            ) -> Result<crate::tokens::TokenCount, crate::tokens::PromptTokensError> {
                todo!()
            }

            fn max_tokens_allowed(&self, _: Option<&Self::PerInvocationOptions>) -> i32 {
                todo!()
            }

            fn get_tokenizer(
                &self,
                _: Option<&Self::PerInvocationOptions>,
            ) -> Result<Self::StepTokenizer<'_>, crate::tokens::TokenizerError> {
                todo!()
            }

            fn get_text_splitter(
                &self,
                _: Option<&Self::PerInvocationOptions>,
            ) -> Result<Self::TextSplitter<'_>, Self::Error> {
                todo!()
            }

            fn new() -> Result<Self, crate::traits::ExecutorCreationError> {
                Self::new_with_options(None, None)
            }
        }
        struct MockSearch;

        #[async_trait]
        impl Tool for MockSearch {
            type Input = String;

            type Output = String;

            type Error = MockError;

            async fn invoke_typed(&self, _: &Self::Input) -> Result<Self::Output, Self::Error> {
                todo!()
            }

            fn description(&self) -> crate::tools::ToolDescription {
                todo!()
            }
        }
        let mock_executor = MockExecutor;
        let mock_search = MockSearch;
        let agent = Agent::new(
            mock_executor,
            mock_search,
            EarlyStoppingConfig {
                max_iterations: None,
                max_time_elapsed_seconds: None,
            },
        );
        let intermediate_steps = vec![
            AgentIntermediateStep {
                action: AgentAction {
                    tool: "Intermediate Answer".into(),
                    tool_input: "How old was Muhammad Ali when he died?".into(),
                    log: "Yes.
Follow up: How old was Muhammad Ali when he died?"
                        .into(),
                },
                observation: "Muhammad Ali was 74 years old when he died.".into(),
            },
            AgentIntermediateStep {
                action: AgentAction {
                    tool: "Intermediate Answer".into(),
                    tool_input: "How old was Alan Turing when he died?".into(),
                    log: "Follow up: How old was Alan Turing when he died?".into(),
                },
                observation: "Alan Turing was 41 years old when he died.".into(),
            },
        ];

        let expected_scratchpad = "Yes.
Follow up: How old was Muhammad Ali when he died?
Intermediate answer: Muhammad Ali was 74 years old when he died.
Follow up: How old was Alan Turing when he died?
Intermediate answer: Alan Turing was 41 years old when he died.\n";

        let scratchpad = agent.build_agent_scratchpad(&intermediate_steps);

        assert_eq!(scratchpad, expected_scratchpad);
    }
}
