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
    #[error("Milvus payload column doesn't exist")]
    InvalidColumnName,
    #[error("Milvus insertion error")]
    InsertionError,
    #[error("No indexes on collection vector field")]
    EmptyIndexError,
    #[error("Milvus query error")]
    QueryError,
    #[error("Serde Error")]
    Serde(serde_json::Error),
}

impl<E> VectorStoreError for MilvusError<E> where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError
{
}
