use llm_chain::options;
use llm_chain::options::ModelRef;
use llm_chain::{executor, parameters, prompt};
use std::env::args;
use std::path::Path;

/// This example demonstrates how to use the llm-chain-gemma crate to generate text using a
/// Gemma.
///
/// Usage: cargo run --example simple path/to/model prompt
///
/// Note: gemma requires 2 files to load, one for the model itself and the other is for
/// sentencepiece.  Currently it assumes both resides in the same directory, and the
/// sentencepiece file name is tokenizer.sbm

fn get_model_type(model_path: &str) -> &str {
    let p = Path::new(model_path);
    if let Some(stem) = p.file_stem() {
        if let Some(model_type) = stem.to_str() {
            return model_type;
        }
    }
    "2b-it"
}

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
        ModelType: get_model_type(model_path),
        ModelType: "gemma",
        Temperature: 0.8
    );
    let exec = executor!(gemma, opts.clone())?;

    let res = prompt!(prompt).run(&parameters!(), &exec).await?;

    println!("{}", res.to_immediate().await?);
    Ok(())
}
