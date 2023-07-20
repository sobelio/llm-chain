use async_trait::async_trait;
use llm_chain::{
    schema::Document,
    traits::{Embeddings, VectorStore},
};
use milvus::{
    client::Client as MilvusClient,
    data::FieldColumn,
    proto::{
        milvus::QueryResults,
        schema::{i_ds::IdField, vector_field},
    },
};
use serde::{de::DeserializeOwned, Serialize};
use std::{marker::PhantomData, sync::Arc};

pub mod errors;

pub struct Milvus<E, M>
where
    E: Embeddings,
    M: Serialize + DeserializeOwned + Send + Sync,
{
    client: Arc<MilvusClient>,
    collection_name: String,
    vector_field_name: String,
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
        embeddings: E,
    ) -> Self {
        Self {
            client,
            collection_name,
            vector_field_name,
            embeddings,
            _marker: Default::default(),
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
        let ids = milvus_results
            .i_ds
            .ok_or(errors::MilvusError::InsertionError)?;
        match ids.id_field {
            Some(IdField::IntId(arr)) => Ok(arr.data.into_iter().map(|x| x.to_string()).collect()),
            Some(IdField::StrId(string_arr)) => Ok(string_arr.data),
            None => Err(errors::MilvusError::InsertionError),
        }
    }

    async fn add_documents(&self, documents: Vec<Document<M>>) -> Result<Vec<String>, Self::Error> {
        todo!()
    }
    async fn similarity_search(
        &self,
        query: String,
        limit: u32,
    ) -> Result<Vec<Document<M>>, Self::Error> {
        todo!()
    }
}
