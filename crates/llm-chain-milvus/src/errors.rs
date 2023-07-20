use thiserror::Error;

use llm_chain::traits::{EmbeddingsError, VectorStoreError};

use milvus::error::Error as InnerError;

#[derive(Debug, Error)]
pub enum MilvusError<E>
where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError,
{
    #[error(transparent)]
    Embeddings(#[from] E),
    #[error("Milvus Client Error")]
    Client(InnerError),
    #[error("Milvus insertion Error")]
    InsertionError,
    #[error("Serde Error")]
    Serde(serde_json::Error),
}

impl<E> VectorStoreError for MilvusError<E> where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError
{
}
