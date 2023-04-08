use llm_chain::{PromptTemplate, Parameters};

/// Creates a new Alpaca prompt template for an instruction.
pub fn new_instruct_template<T: Into<String>>(instruction_template: T) -> PromptTemplate {
    let instruct: PromptTemplate = "
        Below is an instruction that describes a task. Write a response that appropriately completes the request.


        ### Instruction:


        {}


        ### Response:


        ".into();
        instruct.format(&Parameters::new_with_text(instruction_template)).into()
}