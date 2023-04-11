//! # Tokens Module
//!
//! This module provides utilities for managing tokens in Language Learning Models (LLMs),
//! primarily focusing on measuring the sizes of prompts. This is useful for ensuring that
//! prompts stay within the context window size supported by a given model.

use crate::traits::Executor;
use crate::Parameters;
use thiserror::Error;

/// Custom error type for handling prompt token-related errors.
#[derive(Clone, Debug, Error)]
pub enum PromptTokensError {
    /// Indicates that prompt tokens are not accessible for the given step.
    #[error("The prompt tokens are accessible for this type of step.")]
    NotAvailable,
    /// Indicates that the prompt tokens could not be computed.
    #[error("The prompt tokens could not be computed.")]
    UnableToCompute,
}

/// An extension trait for the `Executor` trait that provides additional methods for working
/// with token counts.
pub trait ExecutorTokenCountExt<Step, Output, Token>:
    Executor<Step = Step, Output = Output, Token = Token>
{
    /// Splits a `Parameters` object at the token limit.
    ///
    /// This method takes a `Step` and a `Parameters` object, and returns a tuple of `Parameters`
    /// objects. The first element of the tuple contains the input `Parameters` object, and the
    /// second element contains an `Option<Parameters>` that represents the remainder if the input
    /// text exceeded the token limit.
    ///
    /// # Errors
    ///
    /// Returns a `PromptTokensError` if there is an issue computing the tokens.
    fn split_at_tokens(
        &self,
        step: &Step,
        doc: &Parameters,
    ) -> Result<(Parameters, Option<Parameters>), PromptTokensError> {
        let tokens_used = self.tokens_used(step, doc)?;
        let text = doc.get_text().ok_or(PromptTokensError::UnableToCompute)?;
        if tokens_used.has_tokens_remaining() {
            Ok((doc.clone(), None))
        } else {
            let tokens = self.tokenize_str(step, text)?;
            let idx: usize = (tokens_used.max_tokens - tokens_used.template_tokens_used) as usize;
            let (a, b) = tokens.split_at(idx);
            let a = doc.with_text(self.to_string(step, a)?);
            let b = self.to_string(step, b)?;
            let b = if b.is_empty() {
                None
            } else {
                Some(doc.with_text(b))
            };
            Ok((a, b))
        }
    }
    /// Splits a `Parameters` object into multiple smaller `Parameters` objects that fit within
    /// the context window size supported by the given model.
    ///
    /// # Errors
    ///
    /// Returns a `PromptTokensError` if there is an issue computing the tokens.
    fn split_to_fit(
        &self,
        step: &Step,
        doc: &Parameters,
    ) -> Result<Vec<Parameters>, PromptTokensError> {
        let mut res = Vec::new();
        let mut remainder = doc.clone();
        loop {
            let (a, b) = self.split_at_tokens(step, &remainder)?;
            res.push(a);
            if let Some(new_remainder) = b {
                remainder = new_remainder;
            } else {
                break;
            }
        }
        Ok(res)
    }
}

/// Struct representing token count information, including the maximum tokens allowed and the
/// total number of tokens used.
pub struct TokenCount {
    /// The maximum number of tokens allowed.
    max_tokens: i32,
    /// The total number of tokens used.
    tokens_used: i32,
    /// Template tokens used
    template_tokens_used: i32,
}

impl TokenCount {
    /// Creates a new `TokenCount` instance with the given maximum tokens and tokens used.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - The maximum number of tokens allowed.
    /// * `tokens_used` - The total number of tokens used.
    pub fn new(max_tokens: i32, tokens_used: i32, template_tokens_used: i32) -> Self {
        Self {
            max_tokens,
            tokens_used,
            template_tokens_used,
        }
    }

    /// Returns the number of tokens that could be added to the context window.
    fn tokens_remaining(&self) -> i32 {
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
    /// let token_count = TokenCount::new(100, 50, 10);
    /// assert!(token_count.has_room_for(49));
    /// ```
    pub fn has_room_for(&self, tokens: i32) -> bool {
        self.tokens_remaining() >= tokens
    }
}

/// An extension trait for the `Executor` trait that provides additional methods for working with tokens
impl<E, S, O, T> ExecutorTokenCountExt<S, O, T> for E where
    E: Executor<Step = S, Output = O, Token = T>
{
}
