use llm_chain::chains::sequential::Chain;
use llm_chain::prompt;
use llm_chain::serialization::IoExt;
use llm_chain::traits::StepExt;
use llm_chain_openai::chatgpt::Executor;
use llm_chain_openai::chatgpt::Step;

#[cfg(feature = "serialization")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let chatgpt = Executor::new_default();
    let mut path = std::env::temp_dir();
    path.push("chain-from-yaml.yaml");
    let path = path.to_str().unwrap();

    let chain_to_write = Step::for_prompt(prompt!(
        "You are a bot for making personalized greetings",
        "Make a personalized greet for Joe"
    ))
    .to_chain();
    chain_to_write.write_file_sync(path).unwrap();
    println!("Wrote chain to {}", path);

    let chain = Chain::<Step>::read_file_sync(path).unwrap();
    let res = chain
        .run(llm_chain::Parameters::new(), &chatgpt)
        .await
        .unwrap();
    println!("{}", res);
}
