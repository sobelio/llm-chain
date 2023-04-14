use llm_chain::text_splitter::{TextSplitError, TextSplitter};

use crate::{
    context::LLamaContext,
    tokenizer::{embedding_to_output, llama_tokenize_helper},
};

pub struct LLamaTextSplitter {
    context: LLamaContext,
}

impl TextSplitter<i32> for LLamaTextSplitter {
    fn tokenize(&self, text: &str) -> Result<Vec<i32>, TextSplitError> {
        Ok(llama_tokenize_helper(&self.context, text, true))
    }

    fn chunk_to_string(&self, chunk_tokens: Vec<i32>) -> Result<String, TextSplitError> {
        let output = embedding_to_output(&self.context, &chunk_tokens);
        Ok(output.to_string())
    }
}
