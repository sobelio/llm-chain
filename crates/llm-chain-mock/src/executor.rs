use async_trait::async_trait;
use llm_chain::options::Options;
use llm_chain::output::Output;
use llm_chain::prompt::Prompt;
use llm_chain::tokens::{
    PromptTokensError, TokenCollection, TokenCount, Tokenizer, TokenizerError,
};
use llm_chain::traits::{ExecutorCreationError, ExecutorError};

/// Executor is responsible for running the LLM and managing its context.
pub struct Executor {
    #[allow(dead_code)]
    options: Options,
}

#[async_trait]
impl llm_chain::traits::Executor for Executor {
    type StepTokenizer<'a> = MockTokenizer;

    fn new_with_options(options: Options) -> Result<Self, ExecutorCreationError> {
        Ok(Executor { options: options })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let output = format!( "As a mock large language model, I'm here to help you debug. I have received your prompt: \"{prompt}\" with options \"{options:?}\"");
        Ok(Output::new_immediate(Prompt::text(output)))
    }

    fn tokens_used(
        &self,
        options: &Options,
        prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        let tokenizer = self.get_tokenizer(options)?;
        let input = prompt.to_text();
        let mut tokens_used = tokenizer
            .tokenize_str(&input)
            .map_err(|_e| PromptTokensError::UnableToCompute)?
            .len() as i32;
        let answer_prefix = self.answer_prefix(prompt);
        if let Some(prefix) = answer_prefix {
            let answer_used = tokenizer
                .tokenize_str(&prefix)
                .map_err(|_e| PromptTokensError::UnableToCompute)?
                .len() as i32;
            tokens_used += answer_used
        }
        let max_tokens = self.max_tokens_allowed(options);
        Ok(TokenCount::new(max_tokens, tokens_used))
    }

    fn max_tokens_allowed(&self, _: &Options) -> i32 {
        i32::MAX
    }

    fn answer_prefix(&self, _prompt: &Prompt) -> Option<String> {
        None
    }

    fn get_tokenizer(&self, _: &Options) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
        Ok(MockTokenizer::new(self))
    }
}

pub struct MockTokenizer {}

impl MockTokenizer {
    pub fn new(_executor: &Executor) -> Self {
        MockTokenizer {}
    }
}

impl Tokenizer for MockTokenizer {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        let tokens: Vec<i32> = doc
            .as_bytes()
            .to_vec()
            .into_iter()
            .map(|c| c as i32)
            .collect();
        Ok(tokens.into())
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        let bytes: Vec<u8> = tokens
            .as_i32()
            .unwrap()
            .into_iter()
            .map(|c| c as u8)
            .collect();
        let doc = String::from_utf8(bytes).unwrap();
        Ok(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_chain::traits::Executor;
    #[test]
    fn test_mock_tokenizer() {
        let executor: crate::Executor =
            Executor::new_with_options(Options::empty().clone()).unwrap();
        let tokenizer = executor.get_tokenizer(&executor.options).unwrap();
        let tokens = tokenizer
            .tokenize_str("Héllo world") //Notice that the UTF8 character translates to x3 i32s
            .expect("failed to tokenize");
        println!("{:?}", tokens);
        assert_eq!(tokens.len(), 13);
        let doc = tokenizer
            .to_string(tokens)
            .expect("failed to convert back to string");
        assert_eq!(doc, "Héllo world");
    }
}
