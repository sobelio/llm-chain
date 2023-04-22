use async_openai::types::ChatCompletionResponseStream;
use futures::stream::StreamExt;
use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};
use tokio::sync::Mutex;

pub type SharedResponseStream = Arc<Mutex<ChatCompletionResponseStream>>;

#[derive(Clone)]
pub struct StreamWrapper(SharedResponseStream);

impl StreamWrapper {
    pub fn new(stream: ChatCompletionResponseStream) -> Self {
        Self(Arc::new(Mutex::new(stream)))
    }

    pub async fn primary_textual_output_choices(&self) -> Vec<String> {
        let mut output = vec![];
        let stream = self.0.clone();
        let mut stream = stream.lock().await;
        while let Some(result) = stream.next().await {
            if let Ok(response) = result {
                for chat_choice in &response.choices {
                    if let Some(content) = &chat_choice.delta.content {
                        output.push(content.clone());
                    }
                }
            }
        }
        vec![output.join("")]
    }

    pub fn inner(&self) -> SharedResponseStream {
        self.0.clone()
    }
}

impl Debug for StreamWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamWrapper")
            .field("stream", &"ResponseStream")
            .finish()
    }
}
