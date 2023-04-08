use std::{env, path::Path};

use llm_chain::{traits::StepExt, Parameters};
use llm_chain_llama::{new_instruct_template, Executor, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: myprogram path/to/alpaca-model");
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let exec = Executor::new(path.to_str().unwrap().to_string());

    // This is the Alpaca templated used to instruct the user to write a response. As with all LLM-chain templates, {} is used to indicate the main/default parameter.
    let instr =
        new_instruct_template("Write a hypothetical weather report for {season} in {location}.");
    let chain = Step::new(instr).to_chain();

    let res = chain
        .run(
            Parameters::new()
                .with("season", "summer")
                .with("location", "the moon"),
            exec,
        )
        .await
        .unwrap();
    println!("{:?}", res.to_string());
}
