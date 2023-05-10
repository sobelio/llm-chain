use llm_chain::parsing::extract_labeled_text;

fn main() {
    let text = r"
- Title: The Matrix
- Actor: Keanu Reeves
- Director: The Wachowskis
";

    let result = extract_labeled_text(text);
    println!("{:?}", result);
}
