//! Module modeling the outputs from LLMs
//!
//! The `output` module contains the `Output` trait, which represents the output of a Large Language Model (LLM). It provides methods for retrieving and combining textual outputs from different models.
use async_trait::async_trait;
use futures::stream::StreamExt;

use crate::prompt::ChatRole;

/// Separator string used when joining primary textual outputs.
const OUTPUT_JOINER_SEQUENCE: &str = "\n";

/// The `Output` trait represents the output of a Large Language Model (LLM). It provides
/// methods for retrieving and combining textual outputs from different models.
#[async_trait]
pub trait Output: Send + Clone + Sync {
    /// Gets the primary textual output of the model. This method returns a vector of strings
    /// containing zero to many outputs depending on how many "choices" were generated.
    async fn primary_textual_output_choices(&self) -> Vec<String>;

    /// Gets the primary textual output of the model, if any. If there are multiple choices,
    /// it returns the first one. If no choices are available, it returns `None`.
    async fn primary_textual_output(&self) -> Option<String> {
        let outputs = self.primary_textual_output_choices().await;
        if outputs.is_empty() {
            None
        } else {
            Some(outputs[0].clone())
        }
    }

    /// Gets the `ChatRole` of the output, if any. If no role is available, it returns `None`.
    /// Automatically guesses assistant when not implemented.
    async fn get_chat_role(&self) -> Option<ChatRole> {
        Some(ChatRole::Assistant)
    }

    /// Combines the primary textual outputs from multiple instances implementing the `Output` trait.
    /// The outputs are joined using the `OUTPUT_JOINER_SEQUENCE` separator.
    async fn combine_primary_textual_outputs(outputs: &[&Self]) -> String {
        let primary_outputs = futures::stream::iter(outputs)
            .then(|output| output.primary_textual_output())
            .filter_map(|opt_output| async move { opt_output })
            .collect::<Vec<String>>()
            .await;

        primary_outputs.join(OUTPUT_JOINER_SEQUENCE)
    }

    /// Combines the primary textual outputs of a pair of instances implementing the `Output` trait.
    /// The outputs are joined using the `OUTPUT_JOINER_SEQUENCE` separator.
    async fn combine_primary_textual_outputs_for_pair(output1: &Self, output2: &Self) -> String {
        let (output1, output2) = futures::join!(
            output1.primary_textual_output(),
            output2.primary_textual_output()
        );

        match (output1, output2) {
            (Some(output1), Some(output2)) => {
                format!("{}{}{}", output1, OUTPUT_JOINER_SEQUENCE, output2)
            }
            (Some(output1), None) => output1,
            (None, Some(output2)) => output2,
            (None, None) => String::new(),
        }
    }
}
