use llm_chain::options;
use llm_chain::options::{ModelRef, Options};
use llm_chain::{executor, parameters, prompt};
use std::{env::args, error::Error};
/// This example demonstrates how to use the llm-chain-llama crate to generate text using a
/// LLaMA model.
///
/// Usage: cargo run --example simple path/to/llama-or-alpaca-model
///
/// For example, if the model is located at "/models/llama"
/// cargo run --example simple /models/llama
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_args: Vec<String> = args().collect();
    let args = match &raw_args.len() {
        2 => (
            raw_args[1].as_str(),
            "Rust is a cool programming language because",
        ),
        3 => (raw_args[1].as_str(), raw_args[2].as_str()),
        _ => {
            panic!("Usage: cargo run --release --example simple <path to model> <optional prompt>")
        }
    };

    let model_path = args.0;
    let prompt = args.1;
    let opts = options!(
        Model: ModelRef::from_path(model_path),
        ModelType: "llama",
        MaxContextSize: 512_usize,
        NThreads: 4_usize,
        MaxTokens: 0_usize,
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
        StopSequence: vec!["\n".to_string()]
    );
    let exec = executor!(llama, opts.clone())?;

    let res = prompt!(prompt).run(&parameters!(), &exec).await?;

    println!("{}", res.to_immediate().await?);
    Ok(())
}
