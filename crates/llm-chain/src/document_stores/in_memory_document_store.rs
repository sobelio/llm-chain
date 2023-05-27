use std::collections::HashMap;

use crate::document_stores::document_store::*;
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

impl<M> From<&InMemoryDocument<M>> for Document<M>
where
    M: serde::Serialize + serde::de::DeserializeOwned,
{
    fn from(val: &InMemoryDocument<M>) -> Self {
        let metadata = if let Some(m) = &val.metadata {
            let str = serde_json::to_string(&m).unwrap();
            let cloned = serde_json::from_str::<M>(&str).unwrap();
            Some(cloned)
        } else {
            None
        };

        Document {
            page_content: val.page_content.clone(),
            metadata,
        }
    }
}

impl<M> From<&Document<M>> for InMemoryDocument<M>
where
    M: serde::Serialize + serde::de::DeserializeOwned,
{
    fn from(val: &Document<M>) -> Self {
        let metadata = if let Some(m) = &val.metadata {
            let str = serde_json::to_string(&m).unwrap();
            let cloned = serde_json::from_str::<M>(&str).unwrap();
            Some(cloned)
        } else {
            None
        };

        InMemoryDocument {
            page_content: val.page_content.clone(),
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
    map: HashMap<usize, InMemoryDocument<M>>,
}

impl<M> InMemoryDocumentStore<M>
where
    M: Serialize + DeserializeOwned + Send + Sync,
{
    pub fn new() -> Self {
        InMemoryDocumentStore {
            map: HashMap::new(),
        }
    }
}

impl<M> Default for InMemoryDocumentStore<M>
where
    M: Serialize + DeserializeOwned + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<M> DocumentStore<usize, M> for InMemoryDocumentStore<M>
where
    M: Serialize + DeserializeOwned + Send + Sync,
{
    type Error = InMemoryDocumentStoreError;

    async fn get(&self, id: &usize) -> Result<Option<Document<M>>, Self::Error> {
        Ok(self.map.get(id).map(|m| m.into()))
    }

    async fn next_id(&self) -> Result<usize, Self::Error> {
        Ok(self.map.len())
    }

    async fn insert(&mut self, documents: &HashMap<usize, Document<M>>) -> Result<(), Self::Error> {
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
