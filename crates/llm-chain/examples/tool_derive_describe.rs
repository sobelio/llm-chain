use llm_chain::tools::Describe;
use llm_chain_macros::Describe;
use llm_chain::tools::{Format, FormatPart};



#[derive(Describe)]
struct MyToolInput {
    #[purpose("Person's name")]
    name: String,
    #[purpose("Person's age")]
    age: u8
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("{:#?}", MyToolInput::describe());
}
