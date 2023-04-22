use llm_chain::executor;
use llm_chain::multitool;
use llm_chain::output::Output;
use llm_chain::parameters;
use llm_chain::prompt::chat::{ChatMessage, ChatPrompt, ChatRole};
use llm_chain::step::Step;
use llm_chain::tools::tools::{BashTool, BashToolError, ExitTool, ExitToolError};
use llm_chain::tools::ToolCollection;
use llm_chain::tools::{Tool, ToolDescription, ToolError};
use llm_chain::traits::Executor as ExecutorTrait;
use llm_chain::Parameters;
use llm_chain::PromptTemplate;
use thiserror::Error;

// A simple example generating a prompt with some tools.
multitool!(
    MyMultitool,
    MyMultitoolError,
    BashTool,
    BashToolError,
    ExitTool,
    ExitToolError
);

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tool_collection = ToolCollection::<MyMultitool>::new();
    tool_collection.add_tool(BashTool::new().into());
    tool_collection.add_tool(ExitTool::new().into());
    let tool_prompt = tool_collection.to_prompt_template().unwrap();
    let template = PromptTemplate::combine(vec![
        tool_prompt,
        PromptTemplate::tera("You may ONLY use one tool at a time. Please perform the following task: {{task}}. Once you have read the IP Address you may trigger ExitTool. -- Do not do this before you know the ip address. do not ask for more tasks."),
    ]);
    let task = "Figure out my IP address";
    let exec = executor!()?;

    let mut chat = ChatPrompt::builder()
        .system("You are an automated agent for performing tasks. Your output must always be YAML.")
        .add_message(ChatMessage::from_template(ChatRole::User, template))
        .build()
        .unwrap();
    let params = parameters!("task" => task);
    for _ in 1..5 {
        let res = Step::for_prompt(chat.clone().into())
            .run(&params, &exec)
            .await?;
        let message_text = res.primary_textual_output().await.unwrap();
        println!("Assistant: {}", message_text);
        println!("=============");
        let next_step = match tool_collection.process_chat_input(&message_text) {
            Ok(x) => PromptTemplate::static_string(format!(
                "```yaml
                    {}
                    ```
                    Proceed with your next command.",
                x
            )),
            Err(e) => PromptTemplate::static_string(format!(
                "Correct your output and perform the task - {}. Your task was: {}",
                e, task
            )),
        };
        println!("User: {}", next_step);
        chat = chat
            .to_builder()
            .add_message(ChatMessage::from_template(
                ChatRole::System,
                PromptTemplate::static_string(message_text),
            ))
            .add_message(ChatMessage::from_template(ChatRole::User, next_step))
            .build()
            .unwrap();
    }
    Ok(())
}
