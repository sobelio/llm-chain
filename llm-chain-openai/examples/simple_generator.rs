use llm_chain::{traits::StepExt, Parameters};
use llm_chain_openai::chatgpt::{Executor, Model, Role, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let exec = Executor::new_default();
    let chain = Step::new(
        Model::ChatGPT3_5Turbo,
        [
            (
                Role::System,
                "You are a bot for making personalized greetings",
            ),
            (Role::User, "Make a personalized greet for Joe"),
        ],
    )
    .to_chain();
    let res = chain.run(Parameters::new(), &exec).await.unwrap();
    println!("{:?}", res);
}
