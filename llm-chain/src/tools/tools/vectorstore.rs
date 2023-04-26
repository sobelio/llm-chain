//! The vector store tool accessses information from vector stores.
//!
//! Use it to give your LLM memory or access to semantically searchable information.
use std::marker::PhantomData;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

use crate::{
    tools::{Describe, Format, Tool, ToolDescription, ToolError},
    traits::{Embeddings, EmbeddingsError, VectorStore, VectorStoreError},
};

pub struct VectorStoreTool<E, M, V>
where
    E: Embeddings,
    V: VectorStore<E, M>,
    M: Serialize + DeserializeOwned,
{
    pub store: V,
    pub topic: String,
    pub topic_context: String,
    _data1: PhantomData<E>,
    _data2: PhantomData<M>,
}

impl<E, M, V> VectorStoreTool<E, M, V>
where
    E: Embeddings,
    M: Serialize + DeserializeOwned,
    V: VectorStore<E, M>,
{
    pub fn new(store: V, topic: &str, topic_context: &str) -> Self {
        Self {
            store,
            topic: topic.to_string(),
            topic_context: topic_context.to_string(),
            _data1: Default::default(),
            _data2: Default::default(),
        }
    }
}

#[derive(Debug, Error)]
pub enum VectorStoreToolError<V, E>
where
    V: std::fmt::Debug + std::error::Error + VectorStoreError,
    E: std::fmt::Debug + std::error::Error + EmbeddingsError,
{
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    VectorStoreError(#[from] V),
    #[error(transparent)]
    Embeddings(E),
}

impl<V, E> ToolError for VectorStoreToolError<V, E>
where
    V: std::fmt::Debug + std::error::Error + VectorStoreError,
    E: std::fmt::Debug + std::error::Error + EmbeddingsError,
{
}

#[derive(Serialize, Deserialize)]
pub struct VectorStoreToolInput {
    query: String,
    limit: u32,
}

#[derive(Serialize, Deserialize)]
pub struct VectorStoreToolOutput {
    texts: Vec<String>,
}

impl Describe for VectorStoreToolInput {
    fn describe() -> Format {
        vec![
            (
                "query",
                "You can search for texts similar to this one in the vector database.",
            )
                .into(),
            (
                "limit",
                "The number of texts that will be returned from the vector database.",
            )
                .into(),
        ]
        .into()
    }
}

impl Describe for VectorStoreToolOutput {
    fn describe() -> Format {
        vec![
            ("texts", "List of texts similar to the query.").into(),
            (
                "error_msg",
                "Error message received when trying to search in the vector database.",
            )
                .into(),
        ]
        .into()
    }
}

#[async_trait]
impl<E, M, V> Tool for VectorStoreTool<E, M, V>
where
    E: Embeddings + Sync + Send,
    V: VectorStore<E, M> + Sync + Send,
    M: Sync + Send + serde::Serialize + serde::de::DeserializeOwned,
    Self: 'static,
{
    type Input = VectorStoreToolInput;
    type Output = VectorStoreToolOutput;
    type Error = VectorStoreToolError<<V as VectorStore<E, M>>::Error, <E as Embeddings>::Error>;

    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        match self
            .store
            .similarity_search(input.query.clone(), input.limit)
            .await
        {
            Ok(o) => Ok(VectorStoreToolOutput {
                texts: o.into_iter().map(|d| d.page_content).collect(),
            }),
            Err(e) => Err(<<V as VectorStore<E, M>>::Error as Into<Self::Error>>::into(e)),
        }
    }

    fn description(&self) -> crate::tools::ToolDescription {
        ToolDescription::new(
            "VectorStoreTool",
            "A tool that retrieves documents based on similarity to a given query.",
            &format!(
                "Useful for when you need to answer questions about {}. 
            Whenever you need information about {} 
            you should ALWAYS use this. 
            Input should be a fully formed question.",
                self.topic, self.topic_context
            ),
            Self::Input::describe(),
            Self::Output::describe(),
        )
    }
}
