use llm_chain::chains::sequential::Chain;
use llm_chain::serialization::IoExt;
use llm_chain_openai::chatgpt::Executor;
use llm_chain_openai::chatgpt::Step;

#[cfg(feature = "serialization")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use llm_chain_openai::chatgpt::Model;

    let chatgpt = Executor::new_default();

    let chain_to_write = Chain::<Step>::of_one(Step::new(
        Model::ChatGPT3_5Turbo,
        [
            (
                llm_chain_openai::chatgpt::Role::System,
                "You are a bot for making personalized greetings",
            ),
            (
                llm_chain_openai::chatgpt::Role::User,
                "Make a personalized greet for Joe",
            ),
        ],
    ));
    chain_to_write
        .write_file_sync("chain-from-yaml-2.yaml")
        .unwrap();

    let chain = Chain::<Step>::read_file_sync("chain-from-yaml.yaml").unwrap();
    let res = chain
        .run(llm_chain::Parameters::new(), &chatgpt)
        .await
        .unwrap();
    println!("{:?}", res);
}
