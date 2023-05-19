use std::collections::HashMap;

use crate::document_store::document_store::*;
use crate::schema::Document;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct InMemoryDocument<M>
where
    M: serde::Serialize + serde::de::DeserializeOwned,
{
    page_content: String,
    metadata: Option<M>,
}

impl<M> Into<Document<M>> for &InMemoryDocument<M>
where
    M: serde::Serialize + serde::de::DeserializeOwned,
{
    fn into(self) -> Document<M> {
        let metadata = if let Some(m) = &self.metadata {
            let str = serde_json::to_string(&m).unwrap();
            let cloned = serde_json::from_str::<M>(&str).unwrap();
            Some(cloned)
        } else {
            None
        };

        Document {
            page_content: self.page_content.clone(),
            metadata,
        }
    }
}

impl<M> Into<InMemoryDocument<M>> for &Document<M>
where
    M: serde::Serialize + serde::de::DeserializeOwned,
{
    fn into(self) -> InMemoryDocument<M> {
        let metadata = if let Some(m) = &self.metadata {
            let str = serde_json::to_string(&m).unwrap();
            let cloned = serde_json::from_str::<M>(&str).unwrap();
            Some(cloned)
        } else {
            None
        };

        InMemoryDocument {
            page_content: self.page_content.clone(),
            metadata,
        }
    }
}

#[derive(Debug, Error)]
pub enum InMemoryDocumentStoreError {
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Key \"{0}\" already exists!")]
    KeyConflict(String),
}

impl DocumentStoreError for InMemoryDocumentStoreError {}

pub struct InMemoryDocumentStore<M>
where
    M: Serialize + DeserializeOwned + Send + Sync,
{
    map: HashMap<String, InMemoryDocument<M>>,
}

impl<M> InMemoryDocumentStore<M>
where
    M: Serialize + DeserializeOwned + Send + Sync,
{
    pub fn new() -> InMemoryDocumentStore<M> {
        InMemoryDocumentStore {
            map: HashMap::new(),
        }
    }
}

#[async_trait]
impl<M> DocumentStore<M> for InMemoryDocumentStore<M>
where
    M: Serialize + DeserializeOwned + Send + Sync,
{
    type Error = InMemoryDocumentStoreError;

    async fn get(&self, id: &str) -> Result<Option<Document<M>>, Self::Error> {
        Ok(self.map.get(id).map(|m| m.into()))
    }

    async fn len(&self) -> Result<usize, Self::Error> {
        Ok(self.map.len())
    }

    async fn insert(
        &mut self,
        documents: &HashMap<String, Document<M>>,
    ) -> Result<(), Self::Error> {
        for (key, value) in documents.iter() {
            if self.map.contains_key(key) {
                return Err(InMemoryDocumentStoreError::KeyConflict(key.to_string()));
            } else {
                self.map.insert(key.clone(), value.into());
            }
        }

        Ok(())
    }
}
