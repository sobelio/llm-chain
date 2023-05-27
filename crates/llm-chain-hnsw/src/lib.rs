use std::{
    collections::HashMap, fs::OpenOptions, io::BufReader, marker::PhantomData, path::PathBuf,
    sync::Arc,
};

use async_trait::async_trait;
use hnsw_rs::{hnsw::Hnsw, hnswio::*, prelude::*};
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

pub struct HnswVectorStore<E, D, M>
where
    E: Embeddings,
    D: DocumentStore<usize, M> + Send + Sync,
    M: Serialize + DeserializeOwned + Send + Sync,
{
    hnsw: Arc<Hnsw<f32, DistCosine>>,
    document_store: Arc<Mutex<D>>,
    embeddings: Arc<E>,
    _marker: PhantomData<M>,
}

impl<E, D, M> HnswVectorStore<E, D, M>
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
    ) -> Result<i32, HnswVectorStoreError<E::Error, D::Error>> {
        self.hnsw
            .file_dump(&filename)
            .map_err(HnswVectorStoreError::FileDumpError)
    }

    pub fn load_from_file(
        filename: String,
        embeddings: Arc<E>,
        document_store: Arc<Mutex<D>>,
    ) -> Result<Self, HnswVectorStoreError<E::Error, D::Error>> {
        let graph_fn = format!("{}.hnsw.graph", &filename);
        let graph_path = PathBuf::from(graph_fn);
        let graph_file_res = OpenOptions::new().read(true).open(&graph_path);
        if graph_file_res.is_err() {
            return Err(HnswVectorStoreError::FileLoadError(format!(
                "could not open file {:?}",
                graph_path.as_os_str()
            )));
        }
        let graph_file = graph_file_res.unwrap();
        let data_fn = format!("{}.hnsw.data", &filename);
        let data_path = PathBuf::from(data_fn);
        let data_file_res = OpenOptions::new().read(true).open(&data_path);
        if data_file_res.is_err() {
            return Err(HnswVectorStoreError::FileLoadError(format!(
                "could not open file {:?}",
                data_path.as_os_str()
            )));
        }
        let data_file = data_file_res.unwrap();

        let mut graph_in = BufReader::new(graph_file);
        let mut data_in = BufReader::new(data_file);

        let hnsw_description = load_description(&mut graph_in).unwrap();
        let hnsw_loaded: Hnsw<f32, DistCosine> =
            load_hnsw(&mut graph_in, &hnsw_description, &mut data_in).unwrap();

        Ok(HnswVectorStore {
            hnsw: Arc::new(hnsw_loaded),
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
impl<E, D, M> VectorStore<E, M> for HnswVectorStore<E, D, M>
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
