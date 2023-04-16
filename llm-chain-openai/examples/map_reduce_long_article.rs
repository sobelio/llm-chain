use llm_chain::chains::map_reduce::Chain;
use llm_chain::{prompt, Parameters};
use llm_chain_openai::chatgpt::{Executor, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let exec = Executor::new_default();
    let map_prompt = Step::for_prompt(prompt!(
        "You are a bot for summarizing wikipedia articles, you are terse and focus on accuracy",
        "Summarize this article into bullet points:\n{{text}}"
    ));
    let reduce_prompt = Step::for_prompt(prompt!(
        "You are a diligent bot that summarizes text",
        "Please combine the articles below into one summary as bullet points:\n{{text}}"
    ));
    let chain = Chain::new(map_prompt, reduce_prompt);
    let article = include_str!("article_to_summarize.md");
    let docs = vec![Parameters::new_with_text(article)];
    let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();
    println!("{}", res);
}
