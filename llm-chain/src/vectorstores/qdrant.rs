use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use qdrant_client::{
    prelude::QdrantClient,
    qdrant::{PointStruct, Vectors},
};
use thiserror::Error;
use uuid::Uuid;

use crate::traits::{Embeddings, EmbeddingsError, VectorStore};

pub struct Qdrant<E>
where
    E: Embeddings,
{
    client: Arc<QdrantClient>,
    collection_name: String,
    embeddings: E,
}

impl<E> Qdrant<E>
where
    E: Embeddings,
{
    pub fn new(client: Arc<QdrantClient>, collection_name: String, embeddings: E) -> Self {
        Qdrant {
            client,
            collection_name,
            embeddings,
        }
    }
}

#[derive(Debug, Error)]
pub enum QdrantError<E>
where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError,
{
    #[error(transparent)]
    Embeddings(#[from] E),
    #[error("Qdrant Client Error")]
    Client(#[from] anyhow::Error),
}

#[async_trait]
impl<E> VectorStore<E> for Qdrant<E>
where
    E: Embeddings + Send + Sync,
{
    type Error = QdrantError<E::Error>;

    async fn add_texts(&self, texts: Vec<String>) -> Result<Vec<String>, Self::Error> {
        let embedding_vecs = self.embeddings.embed_texts(texts.clone()).await?;

        let ids = (0..embedding_vecs.len())
            .into_iter()
            .map(|_| Uuid::new_v4().to_string())
            .collect::<Vec<String>>();

        let points = ids
            .clone()
            .into_iter()
            .zip(texts.into_iter())
            .zip(embedding_vecs.into_iter())
            .map(|((uuid, txt), vec)| PointStruct {
                id: Some(uuid.into()),
                payload: HashMap::from([("text".into(), txt.into())]),
                vectors: Some(Vectors::from(vec)),
            })
            .collect();

        self.client
            .upsert_points(self.collection_name.clone(), points, None)
            .await?;

        Ok(ids)
    }
}
