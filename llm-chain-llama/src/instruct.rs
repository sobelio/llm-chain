use llm_chain::PromptTemplate;

/// Creates a new Alpaca prompt template for an instruction.
pub fn new_instruct_template<T: Into<String>>(instruction_template: T) -> PromptTemplate {
    format!("
        Below is an instruction that describes a task. Write a response that appropriately completes the request.


        ### Instruction:


        {}


        ### Response:


        ", instruction_template.into()).into()
}
