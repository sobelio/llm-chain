use crate::batch::LlamaBatch;
use crate::context::ContextParams;
use crate::context::LLamaContext;
use crate::model::ModelParams;
use crate::options::{LlamaInvocation, DEFAULT_OPTIONS};
use crate::tokenizer;
use async_trait::async_trait;
use futures::future::try_join_all;
use llm_chain::options::{options_from_env, Opt, OptDiscriminants, Options, OptionsCascade};
use llm_chain::prompt::Data;
use llm_chain::traits::EmbeddingsCreationError;
use llm_chain::traits::{self, EmbeddingsError};
use std::sync::Arc;
use std::{error::Error, fmt::Debug};
use tokio::sync::Mutex;

/// Generate embeddings using the llama.
///
/// This intended be similar to running the embedding example in llama.cpp:
/// ./embedding -m <path_to_model> --log-disable -p "Hello world" 2>/dev/null
///
pub struct Embeddings {
    context: Arc<Mutex<LLamaContext>>,
    options: Options,
}

#[async_trait]
impl traits::Embeddings for Embeddings {
    type Error = LlamaEmbeddingsError;

    async fn embed_texts(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, Self::Error> {
        let futures = texts.into_iter().map(|text| self.embed_query(text));
        let embeddings = try_join_all(futures).await?;
        Ok(embeddings)
    }

    async fn embed_query(&self, query: String) -> Result<Vec<f32>, Self::Error> {
        let options = vec![&DEFAULT_OPTIONS, &self.options];
        let invocation =
            LlamaInvocation::new(OptionsCascade::from_vec(options), &Data::Text(query)).unwrap();
        let embeddings = self.generate_embeddings(invocation).await?;
        Ok(embeddings)
    }
}

#[allow(dead_code)]
impl Embeddings {
    pub fn new_with_options(opt: Options) -> Result<Self, EmbeddingsCreationError> {
        //TODO(danbev) This is pretty much a duplication of the code in
        // llm_chain::executor::Executor::new_with_options. Find a good place
        // to share this code.
        let opts_from_env =
            options_from_env().map_err(|err| EmbeddingsCreationError::InnerError(err.into()))?;
        let cas = OptionsCascade::new()
            .with_options(&DEFAULT_OPTIONS)
            .with_options(&opts_from_env)
            .with_options(&opt);

        let Some(Opt::Model(model)) = cas.get(OptDiscriminants::Model) else {
            return Err(EmbeddingsCreationError::FieldRequiredError(
                "model_path".to_string(),
            ));
        };

        let mut mp = ModelParams::new();
        if let Some(Opt::NGpuLayers(value)) = cas.get(OptDiscriminants::NGpuLayers) {
            mp.n_gpu_layers = *value;
        }
        if let Some(Opt::MainGpu(value)) = cas.get(OptDiscriminants::MainGpu) {
            mp.main_gpu = *value;
        }
        if let Some(Opt::TensorSplit(values)) = cas.get(OptDiscriminants::TensorSplit) {
            mp.tensor_split = values.clone();
        }
        // Currently, the setting of vocab_only is not allowed as it will cause
        // a crash when using the llama executor which needs to have wieghts loaded
        // in order to work.
        mp.vocab_only = false;

        if let Some(Opt::UseMmap(value)) = cas.get(OptDiscriminants::UseMmap) {
            mp.use_mmap = *value;
        }
        if let Some(Opt::UseMlock(value)) = cas.get(OptDiscriminants::UseMlock) {
            mp.use_mlock = *value;
        }

        let mut cp = ContextParams::new();
        if let Some(Opt::NThreads(value)) = cas.get(OptDiscriminants::NThreads) {
            cp.n_threads = *value as u32;
        }

        if let Some(Opt::MaxContextSize(value)) = cas.get(OptDiscriminants::MaxContextSize) {
            cp.n_ctx = *value as u32;
        }

        if let Some(Opt::MaxBatchSize(value)) = cas.get(OptDiscriminants::MaxBatchSize) {
            cp.n_batch = *value as u32;
        }
        cp.embedding = true;

        Ok(Self {
            context: Arc::new(Mutex::new(LLamaContext::from_file_and_params(
                &model.to_path(),
                Some(&mp),
                Some(&cp),
            )?)),
            options: opt,
        })
    }

    fn get_model_path(options: &Options) -> Result<String, EmbeddingsCreationError> {
        let opts_from_env =
            options_from_env().map_err(|err| EmbeddingsCreationError::InnerError(err.into()))?;
        let cas = OptionsCascade::new()
            .with_options(&DEFAULT_OPTIONS)
            .with_options(&opts_from_env)
            .with_options(&options);
        let model_path = cas
            .get(OptDiscriminants::Model)
            .and_then(|x| match x {
                Opt::Model(m) => Some(m),
                _ => None,
            })
            .ok_or(EmbeddingsCreationError::FieldRequiredError(
                "model_path".to_string(),
            ))?;
        Ok(model_path.to_path())
    }

    async fn generate_embeddings(
        &self,
        input: LlamaInvocation,
    ) -> Result<Vec<f32>, LlamaEmbeddingsError> {
        let context = self.context.clone();
        let embeddings = tokio::task::spawn_blocking(move || {
            let context = context.blocking_lock();
            let prompt_text = input.prompt.to_text();
            let tokens = tokenizer::tokenize(&context, prompt_text.as_str(), true, false);
            //TODO(danbev) Handle the case where the number of tokens
            // are larger than the n_batch size.
            let batch = LlamaBatch::new_with_tokens(tokens.clone(), 1);
            let _ = context
                .llama_decode(&batch)
                .map_err(|e| LlamaEmbeddingsError::InnerError(e.into()));
            context.llama_get_embeddings()
        });
        embeddings
            .await
            .map_err(|e| LlamaEmbeddingsError::InnerError(e.into()))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LlamaEmbeddingsError {
    #[error("error when trying to generate embeddings: {0}")]
    InnerError(#[from] Box<dyn Error + Send + Sync>),
}

impl EmbeddingsError for LlamaEmbeddingsError {}
