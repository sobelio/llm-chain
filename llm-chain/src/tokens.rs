//! # Tokens Module
//!
//! This module provides utilities for managing tokens in Language Learning Models (LLMs),
//! primarily focusing on measuring the sizes of prompts. This is useful for ensuring that
//! prompts stay within the context window size supported by a given model.

use crate::step::Step;
use crate::{traits, Parameters, TextSplitter};
use thiserror::Error;

/// Custom error type for handling prompt token-related errors.
#[derive(Clone, Debug, Error)]
pub enum PromptTokensError {
    /// Indicates that prompt tokens are not accessible for the given step.
    #[error("The prompt tokens are not accessible for this type of step.")]
    NotAvailable,
    /// Indicates that the prompt tokens could not be computed.
    #[error("The prompt tokens could not be computed.")]
    UnableToCompute,
    /// Indicates that the prompt tokens could not be computed because formatting the prompt failed.
    #[error("Formatting prompt failed: {0}")]
    PromptFormatFailed(#[from] crate::prompt::StringTemplateError),
    #[error("Tokenizer error: {0}")]
    TokenizerError(#[from] crate::tokens::TokenizerError),
}

/// An extension trait for the `Executor` trait that provides additional methods for working
/// with token counts.
pub trait ExecutorTokenCountExt<Output, Token: Clone, StepTokenizer>:
    traits::Executor<Output = Output, Token = Token>
{
    /// Splits a `Parameters` object into multiple smaller `Parameters` objects that fit within
    /// the context window size supported by the given model.
    ///
    /// # Arguments
    /// * `step` - The step that will process the Parameters. Has impact on tokenizer & text splitter used
    /// * `doc` - The parameter object to split into multiple, smaller, parameters
    /// * `chunk_overlap` - The amount of tokens each split part should overlap with previous & next chunk
    ///
    /// # Errors
    ///
    /// Returns a `PromptTokensError` if there is an issue computing the tokens.
    fn split_to_fit(
        &self,
        step: &Step<Self>,
        doc: &Parameters,
        chunk_overlap: Option<usize>,
    ) -> Result<Vec<Parameters>, PromptTokensError> {
        let splitter = self
            .get_text_splitter(step.options())
            .map_err(|_e| PromptTokensError::UnableToCompute)?;

        let text = doc.get_text().ok_or(PromptTokensError::UnableToCompute)?;

        let max_tokens = self
            .max_tokens_allowed(step.options())
            .try_into()
            .map_err(|_| PromptTokensError::UnableToCompute)?;

        let chunk_overlap = chunk_overlap.unwrap_or(0);

        let split_params = splitter
            .split_text(text, max_tokens, chunk_overlap)
            .map_err(|_e| PromptTokensError::UnableToCompute)?
            .into_iter()
            .map(Parameters::new_with_text)
            .collect();

        Ok(split_params)
    }
}

/// Struct representing token count information, including the maximum tokens allowed and the
/// total number of tokens used.
pub struct TokenCount {
    /// The maximum number of tokens allowed.
    max_tokens: i32,
    /// The total number of tokens used.
    tokens_used: i32,
}
impl TokenCount {
    /// Creates a new `TokenCount` instance with the given maximum tokens and tokens used.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - The maximum number of tokens allowed.
    /// * `tokens_used` - The total number of tokens used.
    pub fn new(max_tokens: i32, tokens_used: i32) -> Self {
        Self {
            max_tokens,
            tokens_used,
        }
    }

    /// Returns the number of tokens that could be added to the context window.
    pub fn tokens_remaining(&self) -> i32 {
        self.max_tokens - self.tokens_used
    }

    /// Returns true if there is still room in the context window.
    pub fn has_tokens_remaining(&self) -> bool {
        self.has_room_for(1)
    }

    /// Returns true if there is room for the given number of tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The number of tokens to check if there is room for.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_chain::tokens::TokenCount;
    /// let token_count = TokenCount::new(100, 50);
    /// assert!(token_count.has_room_for(49));
    /// ```
    pub fn has_room_for(&self, tokens: i32) -> bool {
        self.tokens_remaining() >= tokens
    }
}

/// An extension trait for the `Executor` trait that provides additional methods for working with tokens
impl<E, O, T, N> ExecutorTokenCountExt<O, T, N> for E
where
    E: traits::Executor<Output = O, Token = T>,
    T: Clone,
{
}

#[derive(Error, Debug, Clone)]
pub enum TokenizerError {
    #[error("Error tokenizing input text")]
    TokenizationError,
    #[error("Error stringifying tokens to text")]
    ToStringError,
    #[error("Error creating tokenizer")]
    TokenizerCreationError,
}

pub trait Tokenizer<TokenType: Clone> {
    /// Tokenizes a string.
    ///
    /// # Parameters
    ///
    /// * `doc`: The string to tokenize.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of tokens, or an error if there was a problem.
    fn tokenize_str(&self, doc: &str) -> Result<Vec<TokenType>, TokenizerError>;

    /// Converts a vector of tokens into a string.
    ///
    /// # Parameters
    ///
    /// * `tokens`: The slice of tokens to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing a string, or an error if there was a problem.
    fn to_string(&self, tokens: Vec<TokenType>) -> Result<String, TokenizerError>;
}
