use async_openai::{error::OpenAIError, types::ChatCompletionResponseStream};
use futures::stream::StreamExt;
use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};
use tokio::sync::Mutex;

pub type ResponseStream = Arc<Mutex<ChatCompletionResponseStream>>;

#[derive(Clone)]
pub struct StreamWrapper(ResponseStream);

impl StreamWrapper {
    pub fn new(stream: ChatCompletionResponseStream) -> Self {
        Self(Arc::new(Mutex::new(stream)))
    }

    pub async fn primary_textual_output_choices(&self) -> Vec<String> {
        let stream = self.inner();
        let temp_cache = Arc::new(Mutex::new(Vec::new()));
        let shared_temp_cache = temp_cache.clone();
        let stream_clone = futures::stream::unfold(stream.clone(), move |state| {
            let shared_temp_cache = shared_temp_cache.clone();
            let state = state.clone();
            async move {
                let mut state_guard = state.lock().await;
                while let Some(result) = state_guard.next().await {
                    if let Ok(response) = result {
                        for chat_choice in &response.choices {
                            if let Some(content) = &chat_choice.delta.content {
                                let mut cache = shared_temp_cache.lock().await;
                                cache.push(content.clone());
                            }
                        }
                        return Some((Ok::<_, OpenAIError>(response), state.clone()));
                    }
                }
                None
            }
        });
        let stream_clone_results = stream_clone.collect::<Vec<_>>().await;
        let new_stream = Box::pin(futures::stream::iter(stream_clone_results));
        let stream = self.inner();
        {
            let mut stream_guard = stream.lock().await;
            *stream_guard = new_stream;
        }
        {
            let temp_cache = temp_cache.lock().await;
            vec![temp_cache.join("")]
        }
    }

    pub fn inner(&self) -> ResponseStream {
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
