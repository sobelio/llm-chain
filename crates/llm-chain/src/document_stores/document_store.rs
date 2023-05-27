use std::collections::HashMap;

use crate::schema::Document;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait DocumentStore<T, M>
where
    T: Send + Sync,
    M: Serialize + DeserializeOwned + Send + Sync,
{
    type Error: std::fmt::Debug + std::error::Error + DocumentStoreError;

    async fn get(&self, id: &T) -> Result<Option<Document<M>>, Self::Error>;

    async fn next_id(&self) -> Result<T, Self::Error>;

    /// Checks if the document store is empty
    async fn is_empty(&self) -> Result<bool, Self::Error> {
        match self.len().await {
            Ok(length) => Ok(length == 0),
            Err(e) => Err(e),
        }
    }

    async fn insert(&mut self, documents: &HashMap<T, Document<M>>) -> Result<(), Self::Error>;
}

pub trait DocumentStoreError {}
