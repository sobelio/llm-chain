use std::{env, path::Path};

use llm_chain::{prompt, traits::StepExt, Parameters};
use llm_chain_llama::{Executor, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: myprogram path/to/alpaca-model");
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let exec = Executor::new(path.to_str().unwrap());

    // This is the Alpaca templated used to instruct the user to write a response. As with all LLM-chain templates, {} is used to indicate the main/default parameter.
    let res = Step::for_prompt(prompt!(
        "Write a hypothetical weather report for {season} in {location}."
    ))
    .run(
        &Parameters::new()
            .with("season", "summer")
            .with("location", "the moon"),
        &exec,
    )
    .await?;
    println!("{}", res);
    Ok(())
}
