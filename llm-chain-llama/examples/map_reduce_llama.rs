use llm_chain::chains::map_reduce::Chain;
use llm_chain::Parameters;
use llm_chain_llama::{Executor, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let exec = Executor::new("model.bin".to_string());
    let map_prompt = Step::new("== ARTICLE ==\n{}== SUMMARY ==\n".into());
    let reduce_prompt = Step::new("== ARTICLE ==\n{}== SUMMARY ==\n".into());
    let chain = Chain::new(map_prompt, reduce_prompt);
    let article = include_str!("article_to_summarize.md");
    let docs = vec![Parameters::new_with_text(article)];
    let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();
    println!("{:?}", res);
}
