use std::sync::Arc;

use llm_chain::{schema::EmptyMetadata, traits::VectorStore, vectorstores::qdrant::Qdrant};
use qdrant_client::{
    prelude::{QdrantClient, QdrantClientConfig},
    qdrant::{CreateCollection, Distance, VectorParams, VectorsConfig},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Qdrant prep
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = Arc::new(QdrantClient::new(Some(config)).await.unwrap());
    let collection_name = "my-collection".to_string();
    let embedding_size = 1536;

    if !client
        .has_collection(collection_name.clone())
        .await
        .unwrap()
    {
        client
            .create_collection(&CreateCollection {
                collection_name: collection_name.clone(),
                vectors_config: Some(VectorsConfig {
                    config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                        VectorParams {
                            size: embedding_size,
                            distance: Distance::Cosine.into(),
                            hnsw_config: None,
                            quantization_config: None,
                        },
                    )),
                }),
                ..Default::default()
            })
            .await
            .unwrap();
    }

    let embeddings = llm_chain_openai::embeddings::Embeddings::default();

    // Storing documents
    let qdrant: Qdrant<llm_chain_openai::embeddings::Embeddings, EmptyMetadata> = Qdrant::new(
        client.clone(),
        collection_name.clone(),
        embeddings,
        None,
        None,
    );
    let doc_ids = qdrant
        .add_texts(vec![
            "This is an amazing way of writing LLM-powered applications".to_string(),
        ])
        .await
        .unwrap();

    println!("Vectors stored under IDs: {:?}", doc_ids);

    let response = client
        .get_points(
            collection_name,
            &doc_ids.into_iter().map(|id| id.into()).collect(),
            Some(true),
            Some(true),
            None,
        )
        .await
        .unwrap();

    println!("Retrieved stored vectors: {:?}", response.result);
}
