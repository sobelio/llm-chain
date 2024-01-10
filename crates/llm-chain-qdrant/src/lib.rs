use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use qdrant_client::{
    prelude::QdrantClient,
    qdrant::{
        value::Kind, with_payload_selector::SelectorOptions, Filter, PayloadIncludeSelector,
        PointId, PointStruct, ScoredPoint, SearchPoints, Value, Vectors, WithPayloadSelector,
    },
};
use thiserror::Error;
use uuid::Uuid;

use llm_chain::{
    schema::Document,
    traits::{Embeddings, EmbeddingsError, VectorStore, VectorStoreError},
};

use serde::{de::DeserializeOwned, Serialize};

const DEFAULT_CONTENT_PAYLOAD_KEY: &str = "page_content";
const DEFAULT_METADATA_PAYLOAD_KEY: &str = "metadata";

pub struct Qdrant<E, M>
where
    E: Embeddings,
    M: Serialize + DeserializeOwned + Send + Sync,
{
    client: Arc<QdrantClient>,
    collection_name: String,
    embeddings: E,
    content_payload_key: String,
    metadata_payload_key: String,
    filter: Option<Filter>,
    _marker: PhantomData<M>,
}

impl<E, M> Qdrant<E, M>
where
    E: Embeddings,
    M: Send + Sync + Serialize + DeserializeOwned,
{
    pub fn new(
        client: Arc<QdrantClient>,
        collection_name: String,
        embeddings: E,
        content_payload_key: Option<String>,
        metadata_payload_key: Option<String>,
        filter: Option<Filter>,
    ) -> Self {
        Qdrant {
            client,
            collection_name,
            embeddings,
            content_payload_key: content_payload_key
                .unwrap_or(DEFAULT_CONTENT_PAYLOAD_KEY.to_string()),
            metadata_payload_key: metadata_payload_key
                .unwrap_or(DEFAULT_METADATA_PAYLOAD_KEY.to_string()),
            filter,
            _marker: Default::default(),
        }
    }

    fn try_document_from_scored_point(
        &self,
        scored_point: ScoredPoint,
    ) -> Result<Document<M>, QdrantError<E::Error>> {
        let metadata = scored_point.payload.get(&self.metadata_payload_key);
        let metadata: Option<M> = match metadata.cloned() {
            Some(val) => {
                let j = serde_json::to_value(val).map_err(QdrantError::Serde)?;
                Some(serde_json::from_value(j).map_err(QdrantError::Serde)?)
            }
            None => None,
        };
        let page_content = scored_point
            .payload
            .get(&self.content_payload_key)
            .ok_or::<QdrantError<E::Error>>(
                ConversionError::PayloadKeyNotFound {
                    payload_key: self.content_payload_key.clone(),
                    point_id: scored_point.id.clone(),
                }
                .into(),
            )?
            .kind
            .clone()
            .ok_or::<QdrantError<E::Error>>(
                ConversionError::InvalidPageContent {
                    point_id: scored_point.id.clone(),
                }
                .into(),
            )?;
        if let Kind::StringValue(page_content) = page_content {
            Ok(Document {
                page_content,
                metadata,
            })
        } else {
            Err(ConversionError::InvalidPageContent {
                point_id: scored_point.id,
            }
            .into())
        }
    }
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Qdrant: Payload key {payload_key:?} not found in Scored Point with ID: {point_id:?}")]
    PayloadKeyNotFound {
        payload_key: String,
        point_id: Option<PointId>,
    },
    #[error("Page content was not a valid string. Point ID: {point_id:?}")]
    InvalidPageContent { point_id: Option<PointId> },
    #[error("Could not convert metadata. Point ID: {point_id:?}")]
    InvalidMetadata { point_id: Option<PointId> },
}

#[derive(Debug, Error)]
pub enum QdrantError<E>
where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError,
{
    #[error(transparent)]
    Embeddings(#[from] E),
    #[error("Qdrant Client Error")]
    Client(anyhow::Error),
    #[error(transparent)]
    ConversionError(#[from] ConversionError),
    #[error("Serde Error")]
    Serde(serde_json::Error),
}

impl<E> VectorStoreError for QdrantError<E> where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError
{
}

#[async_trait]
impl<E, M> VectorStore<E, M> for Qdrant<E, M>
where
    E: Embeddings + Send + Sync,
    M: Send + Sync + Serialize + DeserializeOwned,
{
    type Error = QdrantError<E::Error>;

    async fn add_texts(&self, texts: Vec<String>) -> Result<Vec<String>, Self::Error> {
        let embedding_vecs = self.embeddings.embed_texts(texts.clone()).await?;

        let ids = (0..embedding_vecs.len())
            .map(|_| Uuid::new_v4().to_string())
            .collect::<Vec<String>>();
        let points = embedding_vecs
            .into_iter()
            .zip(texts.into_iter())
            .zip(ids.iter())
            .map(|((vec, text), uuid)| {
                let mut payload = HashMap::new();
                payload.insert(self.content_payload_key.clone(), text.into());
                PointStruct {
                    id: Some(uuid.to_string().into()),
                    payload,
                    vectors: Some(Vectors::from(vec)),
                }
            })
            .collect();
        self.client
            .upsert_points(&self.collection_name, None, points, None)
            .await
            .map_err(QdrantError::Client)?;
        Ok(ids)
    }

    async fn add_documents(&self, documents: Vec<Document<M>>) -> Result<Vec<String>, Self::Error> {
        let texts = documents.iter().map(|d| d.page_content.clone()).collect();
        let embedding_vecs = self.embeddings.embed_texts(texts).await?;

        let ids = (0..embedding_vecs.len())
            .map(|_| Uuid::new_v4().to_string())
            .collect::<Vec<String>>();

        let points: Result<Vec<PointStruct>, Self::Error> = embedding_vecs
            .into_iter()
            .zip(documents.into_iter())
            .zip(ids.iter())
            .map(|((vec, document), uuid)| {
                let mut payload: HashMap<String, Value> = HashMap::new();

                if let Some(metadata) = document.metadata {
                    let val = serde_json::to_value(metadata).map_err(Self::Error::Serde)?;
                    payload.insert(self.metadata_payload_key.clone(), val.into());
                } else {
                    payload.insert(self.metadata_payload_key.clone(), Value { kind: None });
                }
                payload.insert(
                    self.content_payload_key.clone(),
                    document.page_content.clone().into(),
                );
                Ok(PointStruct {
                    id: Some(uuid.to_string().into()),
                    payload,
                    vectors: Some(Vectors::from(vec)),
                })
            })
            .collect();

        let points = points?;

        self.client
            .upsert_points(self.collection_name.clone(), None, points, None)
            .await
            .map_err(QdrantError::Client)?;

        Ok(ids)
    }

    async fn similarity_search(
        &self,
        query: String,
        limit: u32,
    ) -> Result<Vec<Document<M>>, Self::Error> {
        let embedded_query = self.embeddings.embed_query(query).await?;
        let res = self
            .client
            .search_points(&SearchPoints {
                timeout: None,
                shard_key_selector: None,
                sparse_indices: None,
                collection_name: self.collection_name.clone(),
                vector: embedded_query,
                filter: self.filter.clone(),
                limit: limit.into(),
                with_payload: Some(WithPayloadSelector {
                    selector_options: Some(SelectorOptions::Include(PayloadIncludeSelector {
                        fields: vec![
                            self.content_payload_key.clone(),
                            self.metadata_payload_key.clone(),
                        ],
                    })),
                }),
                params: None,
                score_threshold: None,
                offset: None,
                vector_name: None,
                with_vectors: None,
                read_consistency: None,
            })
            .await
            .map_err(QdrantError::Client)?;

        let mut out = vec![];
        for r in res.result.into_iter() {
            let val = self.try_document_from_scored_point(r)?;
            out.push(val);
        }
        Ok(out)
    }
}
