use llm_chain::options::Options;
use llm_chain::output::Output;
use llm_chain::parameters;
use llm_chain::prompt;
use llm_chain::prompt::Prompt;
use llm_chain::step::Step;
use llm_chain::tokens::PromptTokensError;
use llm_chain::tokens::TokenCount;
use llm_chain::tokens::{Tokenizer, TokenizerError};
use llm_chain::traits::Executor;
use llm_chain::traits::{ExecutorCreationError, ExecutorError};
use llm_chain::Parameters;

use async_trait::async_trait;
struct MockExecutor {}
struct MockTokenizer {}

/// Mock Tokenizer implementation only for testing purposes
impl Tokenizer for MockTokenizer {
    fn split_text(
        &self,
        _: &str,
        _: usize,
        _: usize,
    ) -> Result<Vec<String>, llm_chain::tokens::TokenizerError> {
        Ok(vec!["hello,".to_string(), "world".to_string()])
    }
    fn to_string(
        &self,
        _: llm_chain::tokens::TokenCollection,
    ) -> Result<String, llm_chain::tokens::TokenizerError> {
        Ok("hello, world".to_string())
    }
    fn tokenize_str(
        &self,
        _: &str,
    ) -> Result<llm_chain::tokens::TokenCollection, llm_chain::tokens::TokenizerError> {
        Ok(vec![1, 2].into())
    }
}

/// Mock Executor implementation only for testing purposes
#[async_trait]
impl Executor for MockExecutor {
    type StepTokenizer<'a> = MockTokenizer;
    fn answer_prefix(&self, _: &llm_chain::prompt::Prompt) -> Option<String> {
        Some("answer".to_string())
    }
    async fn execute(&self, _: &Options, _: &Prompt) -> Result<Output, ExecutorError> {
        Ok(Output::new_immediate("hello, world".to_string().into()))
    }
    fn new_with_options(_: Options) -> Result<Self, ExecutorCreationError> {
        Ok(Self {})
    }
    fn tokens_used(&self, _: &Options, _: &Prompt) -> Result<TokenCount, PromptTokensError> {
        Ok(TokenCount::new(42, 1))
    }
    fn max_tokens_allowed(&self, _: &Options) -> i32 {
        42
    }
    fn get_tokenizer(&self, _: &Options) -> Result<MockTokenizer, TokenizerError> {
        Ok(MockTokenizer {})
    }
}

#[cfg(test)]
mod tests {
    // Test for step hooks
    use super::*;
    #[tokio::test]
    async fn test_step_hooks() {
        let exec = MockExecutor {};
        let mut step = Step::for_prompt_template(prompt!("Say something to {{name}}"));
        fn before(p: &Parameters) -> Result<(), String> {
            assert_eq!(p, &parameters!("Retep"));
            assert_ne!(p, &parameters!("Mary"));
            Ok(())
        }
        step.add_before_hook(before);
        let _ = step.run(&parameters!("Retep"), &exec).await;
    }
}
