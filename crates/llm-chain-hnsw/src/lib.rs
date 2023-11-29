use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use hnsw_rs::{hnsw::Hnsw, prelude::*};
use llm_chain::{
    document_stores::document_store::*,
    schema::Document,
    traits::{Embeddings, EmbeddingsError, VectorStore, VectorStoreError},
};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tokio::sync::Mutex;

pub struct HnswArgs {
    max_nb_connection: usize,
    max_elements: usize,
    max_layer: usize,
    ef_construction: usize,
}

impl Default for HnswArgs {
    fn default() -> Self {
        HnswArgs {
            max_nb_connection: 16,
            max_elements: 100,
            max_layer: 16,
            ef_construction: 200,
        }
    }
}

pub struct HnswVectorStore<'a, E, D, M>
where
    E: Embeddings,
    D: DocumentStore<usize, M> + Send + Sync,
    M: Serialize + DeserializeOwned + Send + Sync,
{
    hnsw: Arc<Hnsw<'a, f32, DistCosine>>,
    document_store: Arc<Mutex<D>>,
    embeddings: Arc<E>,
    _marker: PhantomData<M>,
}

impl<'a, E, D, M> HnswVectorStore<'a, E, D, M>
where
    E: Embeddings,
    D: DocumentStore<usize, M> + Send + Sync,
    M: Send + Sync + Serialize + DeserializeOwned,
{
    pub fn new(hnsw_args: HnswArgs, embeddings: Arc<E>, document_store: Arc<Mutex<D>>) -> Self {
        let hnsw = Hnsw::new(
            hnsw_args.max_nb_connection,
            hnsw_args.max_elements,
            hnsw_args.max_layer,
            hnsw_args.ef_construction,
            DistCosine {},
        );
        HnswVectorStore {
            hnsw: Arc::new(hnsw),
            document_store,
            embeddings,
            _marker: Default::default(),
        }
    }

    pub fn dump_to_file(
        &self,
        filename: String,
    ) -> Result<String, HnswVectorStoreError<E::Error, D::Error>> {
        self.hnsw
            .file_dump(&filename)
            .map_err(|e| HnswVectorStoreError::FileDumpError(e.to_string()))
    }

    pub fn load_from_file(
        hnsw: Hnsw<'a, f32, DistCosine>,
        embeddings: Arc<E>,
        document_store: Arc<Mutex<D>>,
    ) -> Result<Self, HnswVectorStoreError<E::Error, D::Error>>
where {
        Ok(HnswVectorStore {
            hnsw: Arc::new(hnsw),
            document_store,
            embeddings,
            _marker: Default::default(),
        })
    }
}

#[derive(Debug, Error)]
pub enum HnswVectorStoreError<E, D>
where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError,
    D: std::fmt::Debug + std::error::Error + DocumentStoreError,
{
    #[error(transparent)]
    EmbeddingsError(#[from] E),
    #[error(transparent)]
    DocumentStoreError(D),
    #[error("Document of index \"{0}\" not found!")]
    RelatedDocumentNotFound(usize),
    #[error("Unable to dump hnsw index to file: \"{0}\"")]
    FileDumpError(String),
    #[error("Unable to load hnsw index from file: \"{0}\"")]
    FileLoadError(String),
}

impl<E, D> VectorStoreError for HnswVectorStoreError<E, D>
where
    E: std::fmt::Debug + std::error::Error + EmbeddingsError,
    D: std::fmt::Debug + std::error::Error + DocumentStoreError,
{
}

#[async_trait]
impl<'a, E, D, M> VectorStore<E, M> for HnswVectorStore<'a, E, D, M>
where
    E: Embeddings + Send + Sync,
    D: DocumentStore<usize, M> + Send + Sync,
    M: Send + Sync + Serialize + DeserializeOwned,
{
    type Error = HnswVectorStoreError<E::Error, D::Error>;

    async fn add_texts(&self, texts: Vec<String>) -> Result<Vec<String>, Self::Error> {
        let document_store_arc = self.document_store.clone();
        let mut document_store = document_store_arc.lock().await;

        let embedding_vecs = self.embeddings.embed_texts(texts.clone()).await?;

        let next_id = document_store
            .next_id()
            .await
            .map_err(HnswVectorStoreError::DocumentStoreError)?;
        let ids = (0..embedding_vecs.len())
            .map(|i| next_id + i)
            .collect::<Vec<usize>>();

        let iter = embedding_vecs
            .into_iter()
            .zip(texts.into_iter())
            .zip(ids.iter());

        for ((vec, text), id) in iter {
            document_store
                .insert(&HashMap::from([(id.to_owned(), Document::new(text))]))
                .await
                .map_err(HnswVectorStoreError::DocumentStoreError)?;
            self.hnsw.insert((&vec, id.to_owned()));
        }

        let ids_str = ids
            .iter()
            .map(|&id| format!("{}", id))
            .collect::<Vec<String>>();
        Ok(ids_str)
    }

    async fn add_documents(&self, documents: Vec<Document<M>>) -> Result<Vec<String>, Self::Error> {
        let document_store_arc = self.document_store.clone();
        let mut document_store = document_store_arc.lock().await;

        let texts = documents.iter().map(|d| d.page_content.clone()).collect();
        let embedding_vecs = self.embeddings.embed_texts(texts).await?;

        let next_id = document_store
            .next_id()
            .await
            .map_err(HnswVectorStoreError::DocumentStoreError)?;
        let ids = (0..embedding_vecs.len())
            .map(|i| next_id + i)
            .collect::<Vec<usize>>();

        let iter = embedding_vecs
            .into_iter()
            .zip(documents.into_iter())
            .zip(ids.iter());

        for ((vec, document), id) in iter {
            document_store
                .insert(&HashMap::from([(id.to_owned(), document)]))
                .await
                .map_err(HnswVectorStoreError::DocumentStoreError)?;
            self.hnsw.insert((&vec, id.to_owned()));
        }

        let ids_str = ids
            .iter()
            .map(|&id| format!("{}", id))
            .collect::<Vec<String>>();
        Ok(ids_str)
    }

    async fn similarity_search(
        &self,
        query: String,
        limit: u32,
    ) -> Result<Vec<Document<M>>, Self::Error> {
        let document_store_arc = self.document_store.clone();
        let document_store = document_store_arc.lock().await;

        let embedded_query = self.embeddings.embed_query(query).await?;

        let ef_search = 30;
        let res = self.hnsw.search(&embedded_query, limit as usize, ef_search);

        let mut out = vec![];
        for r in res {
            let id = r.d_id;
            let doc = document_store
                .get(&id)
                .await
                .map_err(HnswVectorStoreError::DocumentStoreError)?
                .ok_or_else(|| HnswVectorStoreError::RelatedDocumentNotFound(r.d_id))?;
            out.push(doc);
        }

        Ok(out)
    }
}
