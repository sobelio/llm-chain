use anyhow::Ok;
use llm_chain::{schema::EmptyMetadata, traits::VectorStore};
use llm_chain_milvus::Milvus;
use milvus::client::Client as MilvusClient;
use milvus::schema::CollectionSchemaBuilder;
use milvus::schema::FieldSchema;
use std::sync::Arc;
use std::vec;

use async_trait::async_trait;
use llm_chain::traits;
use rand::prelude::*;
use thiserror::Error;

struct RandomEmbedder {}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RandomEmbedderError {
    #[error("empty error")]
    Empty,
}

impl traits::EmbeddingsError for RandomEmbedderError {}

#[async_trait]
impl traits::Embeddings for RandomEmbedder {
    type Error = RandomEmbedderError;

    async fn embed_texts(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, Self::Error> {
        let mut rng = rand::thread_rng();
        let mut vecs = Vec::new();
        for _ in 0..texts.len() {
            let mut data: Vec<f32> = Vec::new();

            for _ in 1..=(256) {
                let val = rng.gen();
                data.push(val);
            }
            vecs.push(data);
        }

        Ok(vecs).map_err(|_| RandomEmbedderError::Empty)
    }

    async fn embed_query(&self, _query: String) -> Result<Vec<f32>, Self::Error> {
        let mut rng = rand::thread_rng();
        let mut query_vec: Vec<f32> = Vec::new();
        for _ in 1..=(256) {
            let val = rng.gen();
            query_vec.push(val);
        }
        Ok(query_vec).map_err(|_| RandomEmbedderError::Empty)
    }
}

#[tokio::main]
async fn main() {
    const URL: &str = "http://localhost:19530";

    let collection_name = "test_collection".to_string();
    let client = Arc::new(MilvusClient::new(URL).await.unwrap());
    let embedding_dim: i64 = 256;
    let default_vec_field: &str = "embedding";

    let schema = CollectionSchemaBuilder::new(&collection_name, "a test collection ")
        .add_field(FieldSchema::new_primary_int64(
            "id",
            "primary key field",
            true,
        ))
        .add_field(FieldSchema::new_float_vector(
            default_vec_field,
            "vector embedding field",
            embedding_dim,
        ))
        .build()
        .unwrap();

    if !client.has_collection(&collection_name).await.unwrap() {
        let _ = client
            .create_collection(schema.clone(), None)
            .await
            .unwrap();
    }

    let collection = client.get_collection(&collection_name).await.unwrap();

    // Inserting using llm-milvus
    // let embeddings = llm_chain_openai::embeddings::Embeddings::default();
    let embeddings = RandomEmbedder {};
    let milvus: Milvus<_, EmptyMetadata> = Milvus::new(
        client,
        collection_name.clone(),
        default_vec_field.to_string(),
        embeddings,
    );
    let doc_ids = milvus
        .add_texts(vec![
            "This is an amazing way of writing LLM-powered applications".to_string(),
        ])
        .await
        .unwrap();
    println!("{:?} vectors stored in milvus", doc_ids.len());
    collection.drop().await.unwrap();
}
