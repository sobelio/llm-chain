use llm_chain::chains::map_reduce::Chain;
use llm_chain::executor;
use llm_chain::{prompt, step::Step, Parameters};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!(llama)?;
    let map_prompt = Step::for_prompt_template(prompt!("== ARTICLE ==\n{{text}}== SUMMARY ==\n"));
    let reduce_prompt =
        Step::for_prompt_template(prompt!("== ARTICLE ==\n{{text}}== FINAL SUMMARY ==\n"));

    let chain = Chain::new(map_prompt, reduce_prompt);
    let article = include_str!("article_to_summarize.md");
    let docs = vec![Parameters::new_with_text(article)];
    let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();
    println!("{:?}", res);
    Ok(())
}
