

use llm_chain::executor;
use llm_chain::{prompt, Parameters};


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!(llama)?;
    let res = prompt!("Write a hypothetical weather report for {season} in {location}.")
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
