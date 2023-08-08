use async_trait::async_trait;
use errors::MilvusError;
use llm_chain::{
    schema::Document,
    traits::{Embeddings, VectorStore},
};
use milvus::{
    client::Client as MilvusClient,
    collection::SearchOption,
    data::FieldColumn,
    proto::{milvus::MutationResult, schema::i_ds::IdField},
    value::{Value, ValueVec},
};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, marker::PhantomData, sync::Arc};

pub mod errors;
const DEFAULT_CONTENT_PAYLOAD_KEY: &str = "page_content";
const DEFAULT_METADATA_PAYLOAD_KEY: &str = "metadata";

pub struct Milvus<E, M>
where
    E: Embeddings,
    M: Serialize + DeserializeOwned + Send + Sync,
{
    client: Arc<MilvusClient>,
    collection_name: String,
    vector_field_name: String,
    payload_field_name: Option<String>,
    content_payload_key: String,
    metadata_payload_key: String,
    embeddings: E,
    _marker: PhantomData<M>,
}

impl<E, M> Milvus<E, M>
where
    E: Embeddings,
    M: Serialize + DeserializeOwned + Send + Sync,
{
    pub fn new(
        client: Arc<MilvusClient>,
        collection_name: String,
        vector_field_name: String,
        payload_field_name: Option<String>,
        content_payload_key: Option<String>,
        metadata_payload_key: Option<String>,
        embeddings: E,
    ) -> Self {
        Self {
            client,
            collection_name,
            vector_field_name,
            payload_field_name,
            embeddings,
            content_payload_key: content_payload_key
                .unwrap_or(DEFAULT_CONTENT_PAYLOAD_KEY.to_string()),
            metadata_payload_key: metadata_payload_key
                .unwrap_or(DEFAULT_METADATA_PAYLOAD_KEY.to_string()),
            _marker: Default::default(),
        }
    }

    fn ids_from_milvus_results(
        &self,
        res: MutationResult,
    ) -> Result<Vec<String>, MilvusError<E::Error>> {
        let ids = res.i_ds.ok_or(errors::MilvusError::InsertionError)?;
        match ids.id_field {
            Some(IdField::IntId(arr)) => Ok(arr
                .data
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()),
            Some(IdField::StrId(string_arr)) => Ok(string_arr.data),
            None => Err(errors::MilvusError::InsertionError),
        }
    }
}

#[async_trait]
impl<E, M> VectorStore<E, M> for Milvus<E, M>
where
    E: Embeddings + Send + Sync,
    M: Send + Sync + Serialize + DeserializeOwned,
{
    type Error = errors::MilvusError<E::Error>;

    async fn add_texts(&self, texts: Vec<String>) -> Result<Vec<String>, Self::Error> {
        let embedding_vecs = self.embeddings.embed_texts(texts.clone()).await?;
        let collection = self
            .client
            .get_collection(&self.collection_name)
            .await
            .map_err(errors::MilvusError::Client)?;

        let embed_column = FieldColumn::new(
            collection
                .schema()
                .get_field(&self.vector_field_name)
                .unwrap(),
            embedding_vecs.into_iter().flatten().collect::<Vec<_>>(),
        );

        let milvus_results = collection.insert(vec![embed_column], None).await.unwrap();
        collection
            .flush()
            .await
            .map_err(|_| errors::MilvusError::InsertionError)?;
        self.ids_from_milvus_results(milvus_results)
    }

    async fn add_documents(&self, documents: Vec<Document<M>>) -> Result<Vec<String>, Self::Error> {
        let collection = self
            .client
            .get_collection(&self.collection_name)
            .await
            .map_err(errors::MilvusError::Client)?;

        // Embedding documents' text
        let texts = documents.iter().map(|d| d.page_content.clone()).collect();
        let embedding_vecs = self.embeddings.embed_texts(texts).await?;

        // Construct Milvus vector column
        let embed_column = FieldColumn::new(
            collection
                .schema()
                .get_field(&self.vector_field_name)
                .unwrap(),
            embedding_vecs.into_iter().flatten().collect::<Vec<_>>(),
        );
        // Inserting document in Milvus collection
        // Note: To insert document metadata we need to be sure that
        // the collection has a column `Datatype.JSON`
        match &self.payload_field_name {
            Some(payload_field_name) => {
                let payload_column_name = collection
                    .schema()
                    .get_field(&payload_field_name)
                    .ok_or(errors::MilvusError::InvalidColumnName)?;
                let payloads: Vec<String> = documents
                    .into_iter()
                    .map(|document| {
                        // Construct the
                        let mut payload: HashMap<String, Option<String>> = HashMap::new();

                        if let Some(metadata) = document.metadata {
                            let val =
                                serde_json::to_string(&metadata).map_err(Self::Error::Serde)?;

                            payload.insert(self.metadata_payload_key.clone(), val.into());
                        } else {
                            payload.insert(self.metadata_payload_key.clone(), None);
                        }
                        payload.insert(
                            self.content_payload_key.clone(),
                            document.page_content.clone().into(),
                        );
                        let payload =
                            serde_json::to_string(&payload).map_err(Self::Error::Serde)?;
                        Ok(payload)
                    })
                    .collect::<Result<Vec<_>, errors::MilvusError<_>>>()?;
                let payload_column = FieldColumn::new(payload_column_name, payloads);
                let milvus_results = collection
                    .insert(vec![embed_column, payload_column], None)
                    .await
                    .unwrap();

                collection
                    .flush()
                    .await
                    .map_err(|_| errors::MilvusError::InsertionError)?;

                self.ids_from_milvus_results(milvus_results)
            }
            None => {
                let milvus_results = collection.insert(vec![embed_column], None).await.unwrap();
                self.ids_from_milvus_results(milvus_results)
            }
        }
    }

    async fn similarity_search(
        &self,
        query: String,
        limit: u32,
    ) -> Result<Vec<Document<M>>, Self::Error> {
        let collection = self
            .client
            .get_collection(&self.collection_name)
            .await
            .map_err(errors::MilvusError::Client)?;

        let embedded_query = self.embeddings.embed_query(query).await?;

        let indexes = collection
            .describe_index(self.vector_field_name.clone())
            .await
            .unwrap();

        // Take the first index for now
        let index = indexes
            .first()
            .ok_or(errors::MilvusError::EmptyIndexError)?;

        match &self.payload_field_name {
            Some(out_field) => {
                let results = collection
                    .search(
                        vec![embedded_query.into()],
                        self.vector_field_name.clone(),
                        limit as i32,
                        index.params().metric_type(),
                        vec![out_field],
                        &SearchOption::default(),
                    )
                    .await
                    .map_err(Self::Error::Client)?;

                // Convert Results to docs
                let mut docs: Vec<Document<M>> = Vec::new();
                for res in results {
                    for field in res.field.iter().filter(|f| &f.name == out_field) {
                        match &field.value {
                            ValueVec::String(val) => {
                                let payload: HashMap<String, Option<String>> =
                                    serde_json::from_str(&val[0])
                                        .map_err(errors::MilvusError::Serde)?;

                                let _metadata: Option<String> = payload // XXX: temp fix since the
                                                                       // var is not used rn
                                    .get(&self.metadata_payload_key)
                                    .unwrap()
                                    .clone()
                                    .into();

                                let page_content = payload
                                    .get(&self.content_payload_key)
                                    .unwrap()
                                    .clone()
                                    .unwrap_or("".to_string());

                                docs.push(Document {
                                    page_content: page_content,
                                    metadata: None,
                                });
                            }
                            _ => return Err(errors::MilvusError::QueryError),
                        }
                    }
                }
                Ok(docs)
            }
            None => return Err(errors::MilvusError::QueryError),
        }
    }
}
