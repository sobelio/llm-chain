use llm_chain::text_splitter::TextSplitter;
use llm_chain::tokens::{Tokenizer, TokenizerError};

use crate::Executor;
use crate::{
    context::LLamaContext,
    tokenizer::{embedding_to_output, llama_tokenize_helper},
};

pub struct LLamaTextSplitter<'a> {
    context: &'a LLamaContext,
}

impl<'a> LLamaTextSplitter<'a> {
    pub fn new(exec: &'a Executor) -> Self {
        Self {
            context: exec.get_context(),
        }
    }
}

impl<'a> Tokenizer<i32> for LLamaTextSplitter<'a> {
    fn tokenize_str(&self, doc: &str) -> Result<Vec<i32>, TokenizerError> {
        Ok(llama_tokenize_helper(self.context, doc, true))
    }

    fn to_string(&self, tokens: Vec<i32>) -> Result<String, TokenizerError> {
        let output = embedding_to_output(self.context, &tokens);
        Ok(output.to_string())
    }
}

impl<'a> TextSplitter<i32> for LLamaTextSplitter<'a> {}
