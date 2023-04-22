use async_openai::{error::OpenAIError, types::CreateChatCompletionStreamResponse};
use futures::stream::{Stream, StreamExt};
use std::{
    fmt::{self, Debug, Formatter},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::Mutex;

pub type ResponseStream =
    Pin<Box<dyn Stream<Item = Result<CreateChatCompletionStreamResponse, OpenAIError>> + Send>>;
pub type SharedResponseStream = Arc<Mutex<ResponseStream>>;

#[derive(Clone)]
pub struct StreamWrapper(SharedResponseStream);

impl StreamWrapper {
    pub fn new(stream: ResponseStream) -> Self {
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
