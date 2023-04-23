//! Schema for Documents that can be stored in vector stores.
//!
//! This schema is used to store documents in vector stores. It is used to store the document's content and metadata.
#[derive(Debug)]
pub struct Document<M = EmptyMetadata> {
    pub page_content: String,
    pub metadata: Option<M>,
}

impl<M> Document<M> {
    pub fn new(page_content: String) -> Self {
        Document {
            page_content,
            metadata: None,
        }
    }
}

#[derive(Debug)]
pub struct EmptyMetadata;

impl From<()> for EmptyMetadata {
    fn from(_: ()) -> Self {
        EmptyMetadata
    }
}
