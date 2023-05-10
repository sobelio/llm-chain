//! TextSplitters break text small enough parts to be fed to the model.
//!
//! TextSplitters are responsible for breaking text into small enough parts to be fed to the model. This means that they work with the token stream of the model.
use llm_chain::{
    text_splitter::TextSplitter,
    tokens::{Tokenizer, TokenizerError},
};
use tiktoken_rs::CoreBPE;

use super::Model;

pub struct OpenAITextSplitter {
    model: Model,
}

impl OpenAITextSplitter {
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    fn get_bpe_from_model(&self) -> Result<CoreBPE, TokenizerError> {
        use tiktoken_rs::get_bpe_from_model;
        get_bpe_from_model(&self.model.to_string()).map_err(|_| TokenizerError::TokenizationError)
    }
}

impl Tokenizer<usize> for OpenAITextSplitter {
    fn tokenize_str(&self, doc: &str) -> Result<Vec<usize>, TokenizerError> {
        Ok(self.get_bpe_from_model()?.encode_ordinary(doc))
    }

    fn to_string(&self, tokens: Vec<usize>) -> Result<String, TokenizerError> {
        self.get_bpe_from_model()?
            .decode(tokens)
            .map_err(|_| TokenizerError::ToStringError)
    }
}

impl TextSplitter<usize> for OpenAITextSplitter {}

#[cfg(test)]
mod tests {
    use super::{OpenAITextSplitter, TextSplitter, TokenizerError};

    #[test]
    fn openai_splitter_no_overlap() -> Result<(), TokenizerError> {
        let doc = "This is a sample text that will be split into chunks based on tokens.";
        let max_tokens_per_chunk = 4;
        let chunk_overlap = 0;

        let splitter = OpenAITextSplitter {
            model: crate::chatgpt::Model::ChatGPT3_5Turbo,
        };

        let chunks = splitter.split_text(doc, max_tokens_per_chunk, chunk_overlap)?;

        assert_eq!(
            chunks,
            vec![
                "This is a sample",
                " text that will be",
                " split into chunks based",
                " on tokens."
            ]
        );

        Ok(())
    }

    #[test]
    fn openai_splitter_1_overlap() -> Result<(), TokenizerError> {
        let doc = "This is a sample text that will be split into chunks based on tokens.";
        let max_tokens_per_chunk = 4;
        let chunk_overlap = 1;

        let splitter = OpenAITextSplitter {
            model: crate::chatgpt::Model::ChatGPT3_5Turbo,
        };

        let chunks = splitter.split_text(doc, max_tokens_per_chunk, chunk_overlap)?;

        assert_eq!(
            chunks,
            vec![
                "This is a sample",
                " sample text that will",
                " will be split into",
                " into chunks based on",
                " on tokens."
            ]
        );

        Ok(())
    }

    #[test]
    fn openai_splitter_equal_overlap() -> Result<(), TokenizerError> {
        let doc = "This is a sample text that will be split into chunks based on tokens.";
        let max_tokens_per_chunk = 4;
        let chunk_overlap = max_tokens_per_chunk;

        let splitter = OpenAITextSplitter {
            model: crate::chatgpt::Model::ChatGPT3_5Turbo,
        };

        let chunks = splitter.split_text(doc, max_tokens_per_chunk, chunk_overlap)?;

        assert_eq!(
            chunks,
            vec![
                "This is a sample",
                " is a sample text",
                " a sample text that",
                " sample text that will",
                " text that will be",
                " that will be split",
                " will be split into",
                " be split into chunks",
                " split into chunks based",
                " into chunks based on",
                " chunks based on tokens",
                " based on tokens.",
                " on tokens.",
                " tokens.",
                "."
            ]
        );

        Ok(())
    }
}
