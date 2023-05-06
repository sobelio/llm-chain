use llm_chain::text_splitter::TextSplitter;
use llm_chain::tokens::{Tokenizer, TokenizerError};

use crate::Executor;

pub struct LocalLlmTextSplitter<'a> {
    llm: &'a dyn llm::Model,
}

impl<'a> LocalLlmTextSplitter<'a> {
    pub fn new(exec: &'a Executor) -> Self {
        Self {
            llm: exec.get_llm(),
        }
    }
}

impl<'a> Tokenizer<llm::TokenId> for LocalLlmTextSplitter<'a> {
    fn tokenize_str(&self, doc: &str) -> Result<Vec<llm::TokenId>, TokenizerError> {
        match &self.llm.vocabulary().tokenize(doc, false) {
            Ok(tokens) => Ok(tokens.into_iter().map(|t| t.1).collect()),
            Err(_) => Err(TokenizerError::TokenizationError),
        }
    }

    fn to_string(&self, tokens: Vec<llm::TokenId>) -> Result<String, TokenizerError> {
        let mut res = String::new();
        let mut token_utf8_buf = llm::TokenUtf8Buffer::new();
        for token_id in tokens {
            // Buffer the token until it's valid UTF-8, then call the callback.
            if let Some(tokens) =
                token_utf8_buf.push(self.llm.vocabulary().token(token_id as usize))
            {
                res.push_str(&tokens)
            }
        }

        Ok(res)
    }
}

impl<'a> TextSplitter<llm::TokenId> for LocalLlmTextSplitter<'a> {}
