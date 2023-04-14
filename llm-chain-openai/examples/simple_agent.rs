use llm_chain::tools::create_tool_prompt_segment;
use llm_chain::tools::tools::{ExitTool, PythonTool};
use llm_chain::tools::ToolCollection;
use llm_chain::PromptTemplate;
use llm_chain::{traits::StepExt, Parameters};
use llm_chain_openai::chatgpt::{
    ChatPromptTemplate, Executor, MessagePromptTemplate, Model, Role, Step,
};

// A simple example generating a prompt with some tools.

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut tool_collection = ToolCollection::new();
    tool_collection.add_tool(PythonTool::new());
    tool_collection.add_tool(ExitTool::new());
    let template = create_tool_prompt_segment(
        &tool_collection,
        "Please perform the following task: {}. Once you are done, type trigger ExitTool do not ask for more tasks.",
    );
    let task = "Figure out my IP address";
    let prompt = template.format(&Parameters::new_with_text(task)).unwrap();

    println!("Prompt: {}", prompt);
    let exec = Executor::new_default();

    let mut chat = ChatPromptTemplate::new(vec![
        (
            Role::System,
            "You are an automated agent for performing tasks. Your output must always be YAML.",
        )
            .into(),
        (Role::User, &prompt).into(),
    ]);
    for _ in 1..5 {
        let chain = Step::new(Model::ChatGPT3_5Turbo, chat.clone()).to_chain();
        let res = chain.run(Parameters::new(), &exec).await.unwrap();
        let message_text = res.choices.first().unwrap().message.content.clone();
        println!("Assistant: {}", message_text);
        println!("=============");
        chat.add(MessagePromptTemplate::new(
            Role::Assistant,
            message_text.clone().into(),
        ));
        let resp = tool_collection.process_chat_input(&message_text);
        match resp {
            Ok(x) => {
                chat.add(MessagePromptTemplate::new(
                    Role::User,
                    format!("```yaml\n{}```\nProceed with your next command.", x).into(),
                ));
                println!("LLMCHAIN: {}\n", x)
            }
            Err(e) => {
                let pt = template
                    .format(&Parameters::new_with_text(format!(
                        "Correct your output and perform the task - {}. Your task was: {}",
                        e, task
                    )))
                    .unwrap();
                let pt: PromptTemplate = pt.into();
                chat.add(MessagePromptTemplate::new(Role::User, pt));
                println!("Error: {}", e)
            }
        }
    }
}
