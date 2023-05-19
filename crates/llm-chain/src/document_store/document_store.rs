use std::collections::HashMap;

use crate::schema::Document;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait DocumentStore<M>
where
    M: Serialize + DeserializeOwned + Send + Sync,
{
    type Error: std::fmt::Debug + std::error::Error + DocumentStoreError;

    async fn get(&self, id: &str) -> Result<Option<Document<M>>, Self::Error>;

    async fn len(&self) -> Result<usize, Self::Error>;

    async fn insert(&mut self, documents: &HashMap<String, Document<M>>)
        -> Result<(), Self::Error>;
}

pub trait DocumentStoreError {}
