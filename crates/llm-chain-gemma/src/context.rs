use llm_chain::options::{Opt, OptDiscriminants, Options};
use llm_chain::output::StreamSegment;
use llm_chain::tokens::{TokenCollection, Tokenizer, TokenizerError};
use llm_chain::traits::ExecutorCreationError;
use llm_chain_gemma_sys::{
    gcpp_CreateKVCache, gcpp_Gemma, gcpp_Gemma_Decode, gcpp_Gemma_Decodes, gcpp_Gemma_Encode,
    gcpp_Gemma_Gemma, gcpp_Gemma_SetModelTraining, gcpp_Gemma_destructor, gcpp_GenerateGemma,
    gcpp_KVCache, gcpp_KVCache_destructor, gcpp_Model, gcpp_ModelTraining,
    gcpp_ModelTraining_GEMMA_IT, gcpp_ModelTraining_GEMMA_PT, gcpp_Model_GEMMA_2B,
    gcpp_Model_GEMMA_7B, gcpp_RuntimeConfig, hwy_ThreadPool, hwy_ThreadPool_ThreadPool,
    hwy_ThreadPool_destructor, std_mt19937, std_mt19937_destructor, std_mt19937_mt19937,
    std_mt19937_random_seed, std_string_c_str, std_string_destructor, std_string_string,
    std_vector_int_destructor, std_vector_int_iter, std_vector_int_size, std_vector_int_vector,
    EOS_ID,
};
use std::ffi;
use std::path::Path;
use tokio::sync::mpsc;

#[derive(thiserror::Error, Debug)]
#[error("Gemma.cpp is not supported")]
pub struct GemmaNotSupportedError {}

pub struct GemmaContext {
    gemma: *mut gcpp_Gemma,
    model_training: gcpp_ModelTraining,
    gen: *mut std_mt19937,
    pub config: gcpp_RuntimeConfig,
    kvcache: *mut gcpp_KVCache,
    pool: *mut hwy_ThreadPool,
    pos: u32,
}

impl GemmaContext {
    pub fn new(options: &Options) -> Result<GemmaContext, ExecutorCreationError> {
        let mut model_type: gcpp_Model = gcpp_Model_GEMMA_2B;
        let mut model_training: gcpp_ModelTraining = gcpp_ModelTraining_GEMMA_IT;
        let mut tokenizer_path = String::new();
        let mut compressed_weights_path = String::new();
        let mut config = gcpp_RuntimeConfig {
            max_tokens: 3072,
            max_generated_tokens: 2048,
            temperature: 1.0,
            verbosity: 0,
        };
        if let Some(Opt::ModelType(mt)) = options.get(OptDiscriminants::ModelType) {
            let parts = Vec::from_iter(mt.split("-").into_iter());
            if parts.len() != 2 {
                return Err(ExecutorCreationError::InvalidValue(format!(
                    "model type {} is invalid",
                    mt
                )));
            }
            match parts[0] {
                "2b" => {}
                "7b" => {
                    model_type = gcpp_Model_GEMMA_7B;
                }
                _ => {
                    return Err(ExecutorCreationError::InvalidValue(format!(
                        "model type {} must be 2b or 7b",
                        parts[0]
                    )));
                }
            }
            match parts[1] {
                "it" => {}
                "pt" => {
                    model_training = gcpp_ModelTraining_GEMMA_PT;
                }
                _ => {
                    return Err(ExecutorCreationError::InvalidValue(format!(
                        "model training {} must be it or pt",
                        parts[1]
                    )));
                }
            }
        }
        if let Some(Opt::Model(m)) = options.get(OptDiscriminants::Model) {
            compressed_weights_path = m.to_path();
            let parent = Path::new(&compressed_weights_path).parent();
            if parent.is_none() {
                return Err(ExecutorCreationError::InvalidValue(String::from(
                    "no parent for path",
                )));
            }
            if let Some(tpath) = parent.unwrap().join("tokenizer.spm").to_str() {
                tokenizer_path = String::from(tpath);
            }
        }
        if let Some(Opt::Temperature(t)) = options.get(OptDiscriminants::Temperature) {
            config.temperature = *t;
        }
        if let Some(Opt::MaxTokens(m)) = options.get(OptDiscriminants::MaxTokens) {
            config.max_tokens = *m as ffi::c_uint;
        }
        unsafe {
            let pool = hwy_ThreadPool_ThreadPool(
                if let Some(Opt::NThreads(nt)) = options.get(OptDiscriminants::NThreads) {
                    *nt as ffi::c_uint
                } else {
                    0
                },
            );
            if pool == std::ptr::null_mut() {
                return Err(ExecutorCreationError::InnerError(Box::new(
                    GemmaNotSupportedError {},
                )));
            }

            let gemma = gcpp_Gemma_Gemma(
                tokenizer_path.as_ptr() as *const i8,
                tokenizer_path.len() as ffi::c_uint,
                compressed_weights_path.as_ptr() as *const i8,
                compressed_weights_path.len() as ffi::c_uint,
                std::ptr::null(),
                0,
                model_type,
                pool,
            );
            if gemma == std::ptr::null_mut() {
                return Err(ExecutorCreationError::InnerError(Box::new(
                    GemmaNotSupportedError {},
                )));
            }
            gcpp_Gemma_SetModelTraining(gemma, model_training);

            let gen = std_mt19937_mt19937();
            if gen == std::ptr::null_mut() {
                return Err(ExecutorCreationError::InnerError(Box::new(
                    GemmaNotSupportedError {},
                )));
            }
            std_mt19937_random_seed(gen);

            Ok(GemmaContext {
                gemma: gemma,
                gen: gen,
                model_training: model_training as gcpp_ModelTraining,
                config: config,
                kvcache: gcpp_CreateKVCache(model_type),
                pool: pool,
                pos: 0,
            })
        }
    }
}

impl Drop for GemmaContext {
    fn drop(&mut self) {
        unsafe {
            gcpp_Gemma_destructor(self.gemma);
            std_mt19937_destructor(self.gen);
            gcpp_KVCache_destructor(self.kvcache);
            hwy_ThreadPool_destructor(self.pool);
        }
    }
}

#[repr(C)]
struct GenerateContext {
    gemma: *mut gcpp_Gemma,
    pos: u32,
    tokens_processed: u32,
    input_tokens: u32,
    out: mpsc::UnboundedSender<StreamSegment>,
}

extern "C" fn stream_token(
    ctx: *mut ffi::c_void,
    token: ffi::c_int,
    _: ffi::c_float,
) -> ffi::c_char {
    unsafe {
        let gctx = ctx as *mut GenerateContext;
        (*gctx).pos += 1;
        (*gctx).tokens_processed += 1;
        if (*gctx).tokens_processed < (*gctx).input_tokens {
            return true as ffi::c_char;
        }
        if token == EOS_ID {
            return true as ffi::c_char;
        }
        let s = std_string_string();
        if gcpp_Gemma_Decode((*gctx).gemma, token, s) == 0 {
            return false as ffi::c_char;
        }
        let decoded = ffi::CString::from_raw(std_string_c_str(s)).into_string();
        if decoded.is_err() {
            return false as ffi::c_char;
        }
        (*gctx)
            .out
            .send(StreamSegment::Content(decoded.unwrap()))
            .is_ok() as ffi::c_char
    }
}

impl GemmaContext {
    pub fn generate<'a>(&mut self, prompt: String, out: mpsc::UnboundedSender<StreamSegment>) {
        unsafe {
            if self.model_training != gcpp_ModelTraining_GEMMA_IT {
                self.pos = 0
            }
            let mut prompt_text = if self.model_training == gcpp_ModelTraining_GEMMA_IT {
                format!("<start_of_turn>{prompt}<end_of_turn><start_of_turn>model\n")
            } else {
                prompt
            };
            if self.pos > 0 {
                prompt_text = format!("<end_of_turn>{prompt_text}");
            }
            let tokens = std_vector_int_vector();
            gcpp_Gemma_Encode(
                self.gemma,
                prompt_text.as_mut_ptr() as *mut ffi::c_char,
                prompt_text.len() as ffi::c_uint,
                tokens,
            );
            let mut genctx = GenerateContext {
                gemma: self.gemma,
                pos: self.pos,
                tokens_processed: 0,
                input_tokens: std_vector_int_size(tokens) as u32,
                out: out,
            };
            gcpp_GenerateGemma(
                self.gemma,
                &mut self.config,
                tokens,
                self.pos,
                self.kvcache,
                self.pool,
                (&mut genctx as *mut GenerateContext) as *mut ffi::c_void,
                stream_token,
                self.gen,
            );
            self.pos = genctx.pos;
            std_vector_int_destructor(tokens);
        }
    }

    pub fn max_generated_tokens(&self) -> u32 {
        self.config.max_generated_tokens as u32
    }
}

impl Tokenizer for GemmaContext {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        unsafe {
            let mut doc_copied = String::from(doc);
            let tokens = std_vector_int_vector();
            let result = gcpp_Gemma_Encode(
                self.gemma,
                doc_copied.as_mut_ptr() as *mut ffi::c_char,
                doc.len() as ffi::c_uint,
                tokens,
            );
            if result == 0 {
                return Err(TokenizerError::ToStringError);
            }
            Ok(TokenCollection::from(Vec::from_iter(
                std_vector_int_iter::new(tokens),
            )))
        }
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        let ts = tokens.as_i32()?;
        unsafe {
            let out = std_string_string();
            let ok = gcpp_Gemma_Decodes(self.gemma, ts.as_ptr(), ts.len() as ffi::c_int, out);
            if ok == 0 {
                std_string_destructor(out);
                return Err(TokenizerError::ToStringError);
            }
            let out_str = ffi::CString::from_raw(std_string_c_str(out)).into_string();
            std_string_destructor(out);
            out_str.map_err(|_| TokenizerError::ToStringError)
        }
    }
}

unsafe impl Sync for GemmaContext {}
unsafe impl Send for GemmaContext {}
