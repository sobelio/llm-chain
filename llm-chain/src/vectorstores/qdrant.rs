use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use qdrant_client::{
    prelude::QdrantClient,
    qdrant::{PointStruct, SearchPoints, Vectors, ScoredPoint, PointId},
};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    schema::Document,
    traits::{Embeddings, EmbeddingsError, VectorStore},
};

const DEFAULT_CONTENT_PAYLOAD_KEY: &str = "page_content";
const DEFAULT_METADATA_PAYLOAD_KEY: &str = "metadata";

pub struct Qdrant<E>
where
    E: Embeddings,
{
    client: Arc<QdrantClient>,
    collection_name: String,
    embeddings: E,
    content_payload_key: String,
    metadata_payload_key: String,
}

impl<E> Qdrant<E>
where
    E: Embeddings,
{
    pub fn new(client: Arc<QdrantClient>, collection_name: String, embeddings: E, content_payload_key: Option<String>, metadata_payload_key: Option<String>) -> Self {
        Qdrant {
            client,
            collection_name,
            embeddings,
            content_payload_key: content_payload_key.unwrap_or(DEFAULT_CONTENT_PAYLOAD_KEY.to_string()),
            metadata_payload_key: metadata_payload_key.unwrap_or(DEFAULT_METADATA_PAYLOAD_KEY.to_string()),
        }
    }

    fn try_document_from_scored_point(&self, scored_point: ScoredPoint) -> Result<Document, QdrantError<E::Error>> {
        let metadata = match scored_point.payload.get(&self.metadata_payload_key).ok_or(QdrantError::PayloadKeyNotFound { payload_key: self.content_payload_key, point_id: scored_point.id })?.kind {
            Some(k) => match k {
                qdrant_client::qdrant::value::Kind::NullValue(_) => todo!(),
                qdrant_client::qdrant::value::Kind::DoubleValue(_) => todo!(),
                qdrant_client::qdrant::value::Kind::IntegerValue(_) => todo!(),
                qdrant_client::qdrant::value::Kind::StringValue(_) => todo!(),
                qdrant_client::qdrant::value::Kind::BoolValue(_) => todo!(),
                qdrant_client::qdrant::value::Kind::StructValue(_) => todo!(),
                qdrant_client::qdrant::value::Kind::ListValue(_) => todo!(),
            },
            None => todo!(),
        };
        Ok(Document {
            page_content: scored_point.payload.get(&self.content_payload_key).ok_or(QdrantError::PayloadKeyNotFound { payload_key: self.content_payload_key, point_id: scored_point.id })?,
            metadata,
        })
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
    #[error("Qdrant: Payload key {payload_key:?} not found in Scored Point with ID: {point_id:?}")]
    PayloadKeyNotFound {  payload_key: String, point_id: Option<PointId> },
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

    async fn similarity_search(
        &self,
        query: String,
        limit: u32,
    ) -> Result<Vec<Document>, Self::Error> {
        let embedded_query = self.embeddings.embed_query(query).await?;
        let res = self
            .client
            .search_points(&SearchPoints {
                collection_name: self.collection_name,
                vector: embedded_query,
                filter: None,
                limit: limit.into(),
                with_payload: None,
                params: None,
                score_threshold: None,
                offset: None,
                vector_name: None,
                with_vectors: None,
                read_consistency: None,
            })
            .await?;
        Ok(res.result.into_iter().map(|r| r.))
    }
}
