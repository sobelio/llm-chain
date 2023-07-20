use thiserror::Error;

use llm_chain::traits::{EmbeddingsError, VectorStoreError};

#[derive(Debug, Error)]
pub enum MilvusError<E>
where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError,
{
    #[error(transparent)]
    Embeddings(#[from] E),
    #[error("Milvus Client Error")]
    Client,
    #[error("Serde Error")]
    Serde(serde_json::Error),
}

impl<E> VectorStoreError for MilvusError<E> where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError
{
}
