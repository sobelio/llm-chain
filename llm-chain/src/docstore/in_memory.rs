use std::collections::HashMap;

use async_trait::async_trait;
use thiserror::Error;

use crate::{schema::Document, traits::Docstore};

pub struct InMemoryDocstore {
    data: HashMap<String, Document<String>>,
}

#[derive(Debug, Error)]
pub enum InMemoryDocstoreError {
    #[error("No document stored under key: {0}")]
    NotFound(String),
}

#[async_trait]
impl Docstore for InMemoryDocstore {
    type PageContent = String;

    type Error = InMemoryDocstoreError;

    async fn search(&self, search: String) -> Result<Document<Self::PageContent>, Self::Error> {
        if let Some(document) = self.data.get(&search).cloned() {
            Ok(document)
        } else {
            Err(InMemoryDocstoreError::NotFound(search))
        }
    }
}
