use llm_chain::chains::map_reduce::Chain;
use llm_chain::executor;
use llm_chain::options;
use llm_chain::{prompt, step::Step, Parameters};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = options!(
        ModelType: "llama",
        MaxContextSize: 4096_usize,
        NThreads: 4_usize,
        MaxTokens: 2048_usize,
        MaxBatchSize: 4096_usize,
        TopK: 40_i32,
        TopP: 0.95,
        TfsZ: 1.0,
        TypicalP: 1.0,
        Temperature: 0.8,
        RepeatPenalty: 1.1,
        RepeatPenaltyLastN: 64_usize,
        FrequencyPenalty: 0.0,
        PresencePenalty: 0.0,
        Mirostat: 0_i32,
        MirostatTau: 5.0,
        MirostatEta: 0.1,
        PenalizeNl: true,
        StopSequence: vec!["\n\n".to_string()]
    );
    let exec = executor!(llama, opts.clone())?;
    let map_prompt = Step::for_prompt_template(prompt!("== ARTICLE ==\n{{text}}== SUMMARY ==\n"));
    let reduce_prompt =
        Step::for_prompt_template(prompt!("== ARTICLE ==\n{{text}}== FINAL SUMMARY ==\n"));
    let chain = Chain::new(map_prompt, reduce_prompt);
    let article = include_str!("article_to_summarize.md");
    let docs = vec![Parameters::new_with_text(article)];
    let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();
    println!("{:?}", res.to_immediate().await?.get_content());
    Ok(())
}
