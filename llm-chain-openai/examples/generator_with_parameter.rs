use llm_chain::traits::StepExt;
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
            (Role::User, "Make a personalized greeting tweet for {}"),
        ],
    )
    .to_chain();
    let res = chain.run("Emil".into(), &exec).await.unwrap();
    println!("{:?}", res);
}
