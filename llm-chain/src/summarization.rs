use crate::{chains::map_reduce, output::Output, parameters, prompt, traits};

/// A `TextSummarizer` takes a given text and summarizes it using an `Executor`.
///
/// The summarizer is built on top of a `map_reduce::Chain`, which takes care of the summarization process.
pub struct TextSummarizer<E: traits::Executor> {
    chain: map_reduce::Chain<E>,
}

impl<E: traits::Executor> Default for TextSummarizer<E> {
    fn default() -> Self {
        let map_prompt = prompt!(
            "You are a text summarizer. You will be given a text and you will have to summarize it",
            "Text:\n\n{{text}}\n\nPlease write a summary of the text above. Respond only with the summary."
        );
        let reduce_prompt = prompt!(
            "You are a text summarizer. You will be given a text and you will have to summarize it",
            "Text:\n\n{{text}}\n\nPlease write a combined summary of the segment summaries above. Respond only with the summary."
        );

        TextSummarizer {
            chain: map_reduce::Chain::new(map_prompt, reduce_prompt),
        }
    }
}

/// The error type returned by the `TextSummarizer` when summarizing text.
#[derive(thiserror::Error, Debug)]
pub enum TextSummarizerError<E: traits::Executor> {
    #[error("MapReduceChainError: {0}")]
    MapReduceChainError(#[from] map_reduce::MapReduceChainError<E::Error>),
    #[error("No output was produced")]
    NoOutput,
}

impl<E: traits::Executor> TextSummarizer<E> {
    /// Summarizes the given text using the provided `Executor`.
    ///
    /// Returns the summarized text, or an error if the summarization process fails.
    pub async fn summarize_text(
        &self,
        exec: &E,
        text: &str,
    ) -> Result<String, TextSummarizerError<E>> {
        let params = parameters! {
            "text" => text,
        };
        let chain_output = self.chain.run(vec![params], parameters!(), exec).await?;
        chain_output
            .primary_textual_output()
            .await
            .ok_or(TextSummarizerError::NoOutput)
    }
}

/// A convenience function to summarize text using the provided `Executor`.
///
/// Returns the summarized text, or an error if the summarization process fails.
pub async fn summarize_text<E: traits::Executor>(
    exec: &E,
    text: &str,
) -> Result<String, TextSummarizerError<E>> {
    TextSummarizer::<E>::default()
        .summarize_text(exec, text)
        .await
}
