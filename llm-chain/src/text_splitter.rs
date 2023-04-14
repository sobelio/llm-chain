use std::cmp::max;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextSplitError {
    #[error("Error tokenizing input text")]
    TokenizationError,
    #[error("Error converting token chunk to string")]
    ChunkToStringError,
}

pub trait TextSplitter<TokenType: Clone> {
    fn tokenize(&self, text: &str) -> Result<Vec<TokenType>, TextSplitError>;
    fn chunk_to_string(&self, chunk_tokens: Vec<TokenType>) -> Result<String, TextSplitError>;
    fn split_text(
        &self,
        text: &str,
        max_tokens_per_chunk: usize,
        chunk_overlap: usize,
    ) -> Result<Vec<String>, TextSplitError> {
        let tokens = self.tokenize(text)?;
        let step_size = max(
            max_tokens_per_chunk.checked_sub(chunk_overlap).unwrap_or(1),
            1,
        );

        debug_assert_ne!(step_size, 0);

        (0..tokens.len())
            .step_by(step_size)
            .map(|start_idx| {
                let end_idx = usize::min(start_idx + max_tokens_per_chunk, tokens.len());
                self.chunk_to_string(tokens[start_idx..end_idx].to_vec())
            })
            .collect()
    }
}

pub struct NaiveWhitespaceSplitter;

impl TextSplitter<String> for NaiveWhitespaceSplitter {
    fn tokenize(&self, text: &str) -> Result<Vec<String>, TextSplitError> {
        Ok(text.split_whitespace().map(|w| w.to_string()).collect())
    }

    fn chunk_to_string(&self, chunk_tokens: Vec<String>) -> Result<String, TextSplitError> {
        Ok(chunk_tokens
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<String>>()
            .join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::{NaiveWhitespaceSplitter, TextSplitError, TextSplitter};

    #[test]
    fn whitespace_splitter_no_overlap() -> Result<(), TextSplitError> {
        let text = "This is a sample text that will be split into chunks based on tokens.";
        let max_tokens_per_chunk = 4;
        let chunk_overlap = 0;

        let splitter = NaiveWhitespaceSplitter;

        let chunks = splitter.split_text(text, max_tokens_per_chunk, chunk_overlap)?;

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
    fn whitespace_splitter_1_overlap() -> Result<(), TextSplitError> {
        let text = "This is a sample text that will be split into chunks based on tokens.";
        let max_tokens_per_chunk = 4;
        let chunk_overlap = 1;

        let splitter = NaiveWhitespaceSplitter;

        let chunks = splitter.split_text(text, max_tokens_per_chunk, chunk_overlap)?;

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
    fn whitespace_splitter_equal_overlap() -> Result<(), TextSplitError> {
        let text = "This is a sample text that will be split into chunks based on tokens.";
        let max_tokens_per_chunk = 4;
        let chunk_overlap = max_tokens_per_chunk;

        let splitter = NaiveWhitespaceSplitter;

        let chunks = splitter.split_text(text, max_tokens_per_chunk, chunk_overlap)?;

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
