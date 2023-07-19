use crate::prompt::{ChatRole, Data};
use crate::traits::ExecutorError;
use futures::StreamExt;
use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio_stream::Stream;

use crate::prompt::{ChatMessage, ChatMessageCollection};
#[derive(Debug)]
pub enum StreamSegment {
    Role(ChatRole),
    Content(String),
    Err(ExecutorError),
}

impl fmt::Display for StreamSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StreamSegment::Role(chat_role) => write!(f, "{}", chat_role),
            StreamSegment::Content(content) => write!(f, "{}", content),
            StreamSegment::Err(executor_error) => write!(f, "{}", executor_error),
        }
    }
}

pub struct OutputStream {
    receiver: UnboundedReceiver<StreamSegment>,
}

impl OutputStream {
    pub(super) fn new() -> (mpsc::UnboundedSender<StreamSegment>, Self) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (sender, Self { receiver })
    }

    pub(super) fn from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = StreamSegment> + Send + 'static,
    {
        let (sender, receiver) = mpsc::unbounded_channel();
        let sender_clone = sender;
        let mut stream = Box::pin(stream);

        tokio::spawn(async move {
            while let Some(segment) = stream.next().await {
                if sender_clone.send(segment).is_err() {
                    break;
                }
            }
        });

        Self { receiver }
    }

    pub(super) async fn into_data(self) -> Result<Data<String>, ExecutorError> {
        let mut messages = ChatMessageCollection::new();
        let mut current_role = None;
        let mut current_body = Vec::new();

        let mut stream = self.receiver;

        while let Some(segment) = stream.recv().await {
            match segment {
                StreamSegment::Role(role) => {
                    if let Some(role) = current_role {
                        if !current_body.is_empty() {
                            let body = current_body.join("");
                            messages.add_message(ChatMessage::new(role, body));
                            current_body.clear();
                        }
                    }
                    current_role = Some(role);
                }
                StreamSegment::Content(text) => {
                    current_body.push(text);
                }
                StreamSegment::Err(err) => return Err(err),
            }
        }

        let body = current_body.join("");
        // Handle any remaining message
        if let Some(role) = current_role {
            if !current_body.is_empty() {
                messages.add_message(ChatMessage::new(role, body));
            }
            Ok(messages.into())
        } else {
            Ok(Data::text(body))
        }
    }
}

impl Stream for OutputStream {
    type Item = StreamSegment;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}
