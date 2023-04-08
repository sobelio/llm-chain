use llm_chain::chains::sequential::Chain;
use llm_chain_openai::chatgpt::{Executor, Model, Role, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let exec = Executor::new_default();
    let chain = Chain::new(vec![
        Step::new(
            Model::ChatGPT3_5Turbo,
            [
                (
                    Role::System,
                    "You are a bot for making personalized greetings",
                ),
                (
                    Role::User,
                    "Make personalized birthday e-mail to the whole company for {name} who has their birthday on {date}. Include their name",
                ),
            ],
        ),
        Step::new(
            Model::ChatGPT3_5Turbo,
            [
                (
                    Role::System,
                    "You are an assistant for managing social media accounts for a company",
                ),
                (
                    Role::User,
                    "Summarize this email into a tweet to be sent by the company, use emoji if you can. \n--\n{}",
                ),
            ],
        ),
    ]);
    let res = chain
        .run(
            vec![("name", "Emil"), ("date", "February 30th 2023")].into(),
            exec,
        )
        .await
        .unwrap();
    println!("{:?}", res);
}
