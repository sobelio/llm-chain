use llm_chain::{chains::sequential::Chain, prompt};
use llm_chain_openai::chatgpt::{Executor, Step};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let exec = Executor::new_default();
    let chain = Chain::new(vec![
        Step::for_prompt(
            prompt!("You are a bot for making personalized greetings", "Make personalized birthday e-mail to the whole company for {{name}} who has their birthday on {{date}}. Include their name")
        ),
        Step::for_prompt(
            prompt!( "You are an assistant for managing social media accounts for a company", "Summarize this email into a tweet to be sent by the company, use emoji if you can. \n--\n{{text}}")
        )
    ]);
    let res = chain
        .run(
            vec![("name", "Emil"), ("date", "February 30th 2023")].into(),
            &exec,
        )
        .await
        .unwrap();
    println!("{:?}", res);
}
