use llm_chain::chains::sequential::Chain;
use llm_chain::serialization::StorableEntity;
use llm_chain::step::Step;
use llm_chain::traits::Executor as ExecutorTrait;
use llm_chain::{parameters, prompt};
use llm_chain_openai::chatgpt::Executor;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chatgpt = Executor::new()?;
    let mut path = std::env::temp_dir();
    path.push("chain-from-yaml.yaml");
    let path = path.to_str().unwrap();

    let chain_to_write: Chain<Executor> = Step::for_prompt_template(prompt!(
        "You are a bot for making personalized greetings",
        "Make a personalized greet for Joe"
    ))
    .to_chain();
    chain_to_write.write_file_sync(path)?;
    println!("Wrote chain to {}", path);

    let chain = Chain::<Executor>::read_file_sync(path).unwrap();
    let res = chain.run(parameters!(), &chatgpt).await.unwrap();
    println!("{}", res);
    Ok(())
}
