use crate::prompt::{ChatRole, Data};
use futures::StreamExt;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc::{self, Receiver};
use tokio_stream::Stream;

use crate::prompt::{ChatMessage, ChatMessageCollection};
pub enum StreamSegment {
    Role(ChatRole),
    Content(String),
}

pub struct OutputStream {
    receiver: Receiver<StreamSegment>,
}

impl OutputStream {
    pub(super) fn new() -> (mpsc::Sender<StreamSegment>, Self) {
        let (sender, receiver) = mpsc::channel(100);
        (sender, Self { receiver })
    }

    pub(super) fn from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = StreamSegment> + Send + 'static,
    {
        let (sender, receiver) = mpsc::channel(100);
        let sender_clone = sender.clone();
        let mut stream = Box::pin(stream);

        tokio::spawn(async move {
            while let Some(segment) = stream.next().await {
                if sender_clone.send(segment).await.is_err() {
                    break;
                }
            }
        });

        Self { receiver }
    }

    pub(super) async fn into_data(self) -> Data<String> {
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
            }
        }

        let body = current_body.join("");
        // Handle any remaining message
        if let Some(role) = current_role {
            if !current_body.is_empty() {
                messages.add_message(ChatMessage::new(role, body));
            }
            messages.into()
        } else {
            Data::text(body)
        }
    }
}

impl Stream for OutputStream {
    type Item = StreamSegment;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_recv(cx)
    }
}
