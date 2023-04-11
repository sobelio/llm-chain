use llm_chain::chains::map_reduce::Chain;
use llm_chain::Parameters;
use llm_chain_openai::chatgpt::{Executor, Model, Role, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let exec = Executor::new_default();
    let map_prompt = Step::new(
        Model::ChatGPT3_5Turbo,
        [
            (
                Role::System,
                "You are a bot for summarizing wikipedia articles, you are terse and focus on accuracy",
            ),
            (Role::User, "Summarize this article into bullet points:\n{}"),
        ],
    );
    let reduce_prompt = Step::new(
        Model::ChatGPT3_5Turbo,
        [
            (Role::System, "You are a diligent bot that summarizes text"),
            (
                Role::User,
                "Please combine the articles below into one summary as bullet points:\n{}",
            ),
        ],
    );
    let chain = Chain::new(map_prompt, reduce_prompt);
    let article = include_str!("article_to_summarize.md");
    let docs = vec![Parameters::new_with_text(article)];
    let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();
    println!("{:?}", res);
}
