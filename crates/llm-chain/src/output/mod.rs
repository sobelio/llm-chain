mod stream;

use core::fmt;

use crate::prompt::Data;
use futures::Stream;
use tokio::sync::mpsc;

pub use stream::{OutputStream, StreamSegment};

/// The `Output` enum provides a general interface for outputs of different types.
/// The `Immediate` variant represents data that is immediately available, while the `Stream` variant
/// represents data that may be produced over time.
pub enum Output {
    /// Represents immediately available data.
    Immediate(Immediate),

    /// Represents data that is produced over time.
    Stream(OutputStream),
}

impl Output {
    /// Converts the `Output` to its `Immediate` form.
    /// If the output is `Stream`, it will be consumed and turned into an `Immediate` output.
    /// This operation is asynchronous as it may need to wait for all data to be produced in the case of a `Stream`.
    pub async fn to_immediate(self) -> Immediate {
        match self {
            Output::Immediate(x) => x,
            Output::Stream(x) => Immediate(x.into_data().await),
        }
    }

    /// Creates a new `Stream` output along with a sender to produce data.
    pub fn new_stream() -> (mpsc::Sender<StreamSegment>, Self) {
        let (sender, stream) = OutputStream::new();

        (sender, Output::Stream(stream))
    }

    pub fn from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = StreamSegment> + Send + 'static,
    {
        Output::Stream(OutputStream::from_stream(stream))
    }

    /// Creates a new `Immediate` output from the given data.
    pub fn new_immediate(data: Data<String>) -> Self {
        Output::Immediate(Immediate(data))
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::Immediate(Immediate(data)) => data.fmt(f),
            Output::Stream(_) => write!(f, "<OutputStream>"),
        }
    }
}

pub struct Immediate(Data<String>);

impl Immediate {
    /// Returns a reference to the content if it is immediately available.
    pub fn get_content(&self) -> &Data<String> {
        &self.0
    }

    pub fn as_content(self) -> Data<String> {
        self.0
    }

    pub fn primary_textual_output(&self) -> Option<String> {
        self.get_content().extract_last_body().map(|x| x.clone())
    }
}

impl fmt::Display for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
