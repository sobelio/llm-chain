//! TextSplitters break text small enough parts to be fed to the model.
//!
//! TextSplitters are responsible for breaking text into small enough parts to be fed to the model. This means that they work with the token stream of the model.
use crate::tokens::{Tokenizer, TokenizerError};
use std::cmp::max;

pub trait TextSplitter<TokenType>: Tokenizer<TokenType>
where
    TokenType: Clone,
{
    fn split_text(
        &self,
        doc: &str,
        max_tokens_per_chunk: usize,
        chunk_overlap: usize,
    ) -> Result<Vec<String>, TokenizerError> {
        let tokens = self.tokenize_str(doc)?;
        let step_size = max(
            max_tokens_per_chunk.checked_sub(chunk_overlap).unwrap_or(1),
            1,
        );

        debug_assert_ne!(step_size, 0);

        (0..tokens.len())
            .step_by(step_size)
            .map(|start_idx| {
                let end_idx = usize::min(start_idx + max_tokens_per_chunk, tokens.len());
                self.to_string(tokens[start_idx..end_idx].to_vec())
            })
            .collect()
    }
}

pub struct NaiveWhitespaceSplitter;

impl Tokenizer<String> for NaiveWhitespaceSplitter {
    fn tokenize_str(&self, doc: &str) -> Result<Vec<String>, TokenizerError> {
        Ok(doc.split_whitespace().map(|t| t.to_string()).collect())
    }

    fn to_string(&self, tokens: Vec<String>) -> Result<String, TokenizerError> {
        Ok(tokens
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<String>>()
            .join(" "))
    }
}

impl TextSplitter<String> for NaiveWhitespaceSplitter {}

#[cfg(test)]
mod tests {
    use super::{NaiveWhitespaceSplitter, TextSplitter, TokenizerError};

    #[test]
    fn whitespace_splitter_no_overlap() -> Result<(), TokenizerError> {
        let doc = "This is a sample text that will be split into chunks based on tokens.";
        let max_tokens_per_chunk = 4;
        let chunk_overlap = 0;

        let splitter = NaiveWhitespaceSplitter;

        let chunks = splitter.split_text(doc, max_tokens_per_chunk, chunk_overlap)?;

        assert_eq!(
            chunks,
            vec![
                "This is a sample",
                "text that will be",
                "split into chunks based",
                "on tokens."
            ]
        );

        Ok(())
    }

    #[test]
    fn whitespace_splitter_1_overlap() -> Result<(), TokenizerError> {
        let doc = "This is a sample text that will be split into chunks based on tokens.";
        let max_tokens_per_chunk = 4;
        let chunk_overlap = 1;

        let splitter = NaiveWhitespaceSplitter;

        let chunks = splitter.split_text(doc, max_tokens_per_chunk, chunk_overlap)?;

        assert_eq!(
            chunks,
            vec![
                "This is a sample",
                "sample text that will",
                "will be split into",
                "into chunks based on",
                "on tokens."
            ]
        );

        Ok(())
    }

    #[test]
    fn whitespace_splitter_equal_overlap() -> Result<(), TokenizerError> {
        let doc = "This is a sample text that will be split into chunks based on tokens.";
        let max_tokens_per_chunk = 4;
        let chunk_overlap = max_tokens_per_chunk;

        let splitter = NaiveWhitespaceSplitter;

        let chunks = splitter.split_text(doc, max_tokens_per_chunk, chunk_overlap)?;

        assert_eq!(
            chunks,
            vec![
                "This is a sample",
                "is a sample text",
                "a sample text that",
                "sample text that will",
                "text that will be",
                "that will be split",
                "will be split into",
                "be split into chunks",
                "split into chunks based",
                "into chunks based on",
                "chunks based on tokens.",
                "based on tokens.",
                "on tokens.",
                "tokens."
            ]
        );

        Ok(())
    }
}
