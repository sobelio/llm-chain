use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use qdrant_client::{
    prelude::QdrantClient,
    qdrant::{
        value::Kind, with_payload_selector::SelectorOptions, PayloadIncludeSelector, PointId,
        PointStruct, ScoredPoint, SearchPoints, Value, Vectors, WithPayloadSelector,
    },
};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    schema::{Document, EmptyMetadata},
    traits::{Embeddings, EmbeddingsError, VectorStore, VectorStoreError},
};

const DEFAULT_CONTENT_PAYLOAD_KEY: &str = "page_content";
const DEFAULT_METADATA_PAYLOAD_KEY: &str = "metadata";

impl TryFrom<Value> for EmptyMetadata {
    type Error = ();

    fn try_from(_: Value) -> Result<Self, Self::Error> {
        Ok(EmptyMetadata)
    }
}

impl From<EmptyMetadata> for Value {
    fn from(_val: EmptyMetadata) -> Self {
        Value {
            kind: Some(qdrant_client::qdrant::value::Kind::NullValue(0)),
        }
    }
}

pub struct Qdrant<E, M>
where
    E: Embeddings,
    M: TryFrom<Value> + Into<Value> + Send + Sync,
{
    client: Arc<QdrantClient>,
    collection_name: String,
    embeddings: E,
    content_payload_key: String,
    metadata_payload_key: String,
    _marker: PhantomData<M>,
}

impl<E, M> Qdrant<E, M>
where
    E: Embeddings,
    M: TryFrom<Value> + Into<Value> + Send + Sync,
{
    pub fn new(
        client: Arc<QdrantClient>,
        collection_name: String,
        embeddings: E,
        content_payload_key: Option<String>,
        metadata_payload_key: Option<String>,
    ) -> Self {
        Qdrant {
            client,
            collection_name,
            embeddings,
            content_payload_key: content_payload_key
                .unwrap_or(DEFAULT_CONTENT_PAYLOAD_KEY.to_string()),
            metadata_payload_key: metadata_payload_key
                .unwrap_or(DEFAULT_METADATA_PAYLOAD_KEY.to_string()),
            _marker: Default::default(),
        }
    }

    fn try_document_from_scored_point(
        &self,
        scored_point: ScoredPoint,
    ) -> Result<Document<M>, QdrantError<E::Error>> {
        let metadata = scored_point.payload.get(&self.metadata_payload_key);
        let metadata: Option<M> = match metadata.cloned() {
            Some(val) => match M::try_from(val) {
                Ok(m) => Ok::<std::option::Option<M>, QdrantError<E::Error>>(Some(m)),
                Err(_) => Err(ConversionError::InvalidMetadata {
                    point_id: scored_point.id.clone(),
                }
                .into()),
            },
            None => Ok(None),
        }?;
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
    Client(#[from] anyhow::Error),
    #[error(transparent)]
    ConversionError(#[from] ConversionError),
}

impl<E> VectorStoreError for QdrantError<E> where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError
{
}

#[async_trait]
impl<E, M> VectorStore<E, M> for Qdrant<E, M>
where
    E: Embeddings + Send + Sync,
    M: TryFrom<Value> + Into<Value> + Send + Sync,
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
            .upsert_points(self.collection_name.clone(), points, None)
            .await?;
        Ok(ids)
    }

    async fn add_documents(&self, documents: Vec<Document<M>>) -> Result<Vec<String>, Self::Error> {
        let texts = documents.iter().map(|d| d.page_content.clone()).collect();
        let embedding_vecs = self.embeddings.embed_texts(texts).await?;

        let ids = (0..embedding_vecs.len())
            .map(|_| Uuid::new_v4().to_string())
            .collect::<Vec<String>>();

        let points = embedding_vecs
            .into_iter()
            .zip(documents.into_iter())
            .zip(ids.iter())
            .map(|((vec, document), uuid)| {
                let mut payload: HashMap<String, Value> = HashMap::new();

                if let Some(metadata) = document.metadata {
                    payload.insert(self.metadata_payload_key.clone(), metadata.into());
                } else {
                    payload.insert(self.metadata_payload_key.clone(), Value { kind: None });
                }
                payload.insert(
                    self.content_payload_key.clone(),
                    document.page_content.clone().into(),
                );
                PointStruct {
                    id: Some(uuid.to_string().into()),
                    payload,
                    vectors: Some(Vectors::from(vec)),
                }
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
    ) -> Result<Vec<Document<M>>, Self::Error> {
        let embedded_query = self.embeddings.embed_query(query).await?;
        let res = self
            .client
            .search_points(&SearchPoints {
                collection_name: self.collection_name.clone(),
                vector: embedded_query,
                filter: None,
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
            .await?;

        let mut out = vec![];
        for r in res.result.into_iter() {
            let val = self.try_document_from_scored_point(r)?;
            out.push(val);
        }
        Ok(out)
    }
}
