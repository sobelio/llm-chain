use std::ffi;
use std::path::Path;
use llm_chain::output::StreamSegment;
use llm_chain_gemma_sys::{
    gcpp_Gemma, gcpp_Gemma_Decode, gcpp_Gemma_Decodes, gcpp_Gemma_Encode, gcpp_Gemma_Gemma, gcpp_Gemma_destructor, gcpp_GenerateGemma, gcpp_InferenceArgs, gcpp_InferenceArgs_InferenceArgs, gcpp_InferenceArgs_MaxGeneratedTokens, gcpp_InferenceArgs_Multiturn, gcpp_InferenceArgs_SetMaxTokens, gcpp_InferenceArgs_SetTemperature, gcpp_InferenceArgs_Validate, gcpp_InferenceArgs_destructor, gcpp_LoaderArgs_LoaderArgs, gcpp_LoaderArgs_ModelTraining, gcpp_LoaderArgs_SetCache, gcpp_LoaderArgs_SetModelTypeValue, gcpp_LoaderArgs_SetTokenizer, gcpp_LoaderArgs_Validate, gcpp_LoaderArgs_destructor, gcpp_ModelTraining, gcpp_ModelTraining_GEMMA_IT, hwy_ThreadPool, hwy_ThreadPool_ThreadPool, hwy_ThreadPool_destructor, std_mt19937, std_mt19937_destructor, std_mt19937_mt19937, std_mt19937_random_seed, std_string_c_str, std_string_destructor, std_string_string, std_vector_int_destructor, std_vector_int_iter, std_vector_int_size, std_vector_int_vector, EOS_ID
};
use llm_chain::options::{Opt, Options, OptDiscriminants};
use llm_chain::tokens::{TokenCollection, Tokenizer, TokenizerError};
use llm_chain::traits::ExecutorCreationError;
use tokio::sync::mpsc;

pub struct GemmaContext {
    gemma: *mut gcpp_Gemma,
    model_training: gcpp_ModelTraining,
    gen: *mut std_mt19937,
    pub iargs: *mut gcpp_InferenceArgs,
    pool: *mut hwy_ThreadPool,
    inner_pool: *mut hwy_ThreadPool,
    pos: u32,
}

impl GemmaContext {
    pub fn new(options: &Options) -> Result<GemmaContext, ExecutorCreationError> {
        unsafe {
            let largs = gcpp_LoaderArgs_LoaderArgs(0, std::ptr::null_mut());
            if let Some(Opt::ModelType(mt)) = options.get(OptDiscriminants::ModelType) {
                gcpp_LoaderArgs_SetModelTypeValue(largs, mt.clone().into_bytes().as_ptr() as *const i8);
            }
            if let Some(Opt::Model(m)) = options.get(OptDiscriminants::Model) {
                // Typically the downloaded model data is compressed and set as cache.
                // TODO: consider the case of non-compressed one?
                let path = m.to_path();
                gcpp_LoaderArgs_SetCache(largs, path.as_ptr() as *const i8);
                // TODO: consider adding the option for tokenizer file.
                let parent = Path::new(&path).parent();
                if parent.is_none() {
                    return Err(ExecutorCreationError::InvalidValue(String::from("no parent for path")));
                }
                if let Some(tokenizer_path) = parent.unwrap().join("tokenizer.spm").to_str() {
                    gcpp_LoaderArgs_SetTokenizer(largs, tokenizer_path.as_ptr() as *const i8);
                } else {
                    return Err(ExecutorCreationError::InvalidValue(String::from("conversion from path to str for tokenizer")));
                }
            }

            let err = gcpp_LoaderArgs_Validate(largs);
            if err != std::ptr::null_mut() {
                let msg = ffi::CString::from_raw(err as *mut ffi::c_char).into_string();
                if msg.is_err() {
                    return Err(ExecutorCreationError::InnerError(Box::new(msg.unwrap_err())));
                }
                gcpp_LoaderArgs_destructor(largs);
                return Err(ExecutorCreationError::InvalidValue(msg.unwrap()));
            }

            let iargs = gcpp_InferenceArgs_InferenceArgs(0, std::ptr::null_mut());
            if let Some(Opt::Temperature(t)) = options.get(OptDiscriminants::Temperature) {
                gcpp_InferenceArgs_SetTemperature(iargs, *t);
            }
            if let Some(Opt::MaxTokens(m)) = options.get(OptDiscriminants::MaxTokens) {
                gcpp_InferenceArgs_SetMaxTokens(iargs, *m as ffi::c_uint);
            }

            let err = gcpp_InferenceArgs_Validate(iargs);
            if err != std::ptr::null_mut() {
                let msg = ffi::CString::from_raw(err as *mut ffi::c_char).into_string();
                if msg.is_err() {
                    return Err(ExecutorCreationError::InnerError(Box::new(msg.unwrap_err())));
                }
                gcpp_LoaderArgs_destructor(largs);
                gcpp_InferenceArgs_destructor(iargs);
                return Err(ExecutorCreationError::InvalidValue(msg.unwrap()));
            }

            let pool = hwy_ThreadPool_ThreadPool(
                if let Some(Opt::NThreads(nt)) = options.get(OptDiscriminants::NThreads) {
                    *nt as ffi::c_uint
                } else {
                    0
                });
            let inner_pool = hwy_ThreadPool_ThreadPool(1);

            let gemma = gcpp_Gemma_Gemma(largs, pool);
            let gen = std_mt19937_mt19937();
            std_mt19937_random_seed(gen);

            let model_training = gcpp_LoaderArgs_ModelTraining(largs);

            gcpp_LoaderArgs_destructor(largs);

            Ok(GemmaContext{
                gemma: gemma,
                gen: gen,
                model_training: model_training as gcpp_ModelTraining,
                iargs: iargs,
                pool: pool,
                inner_pool: inner_pool,
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
            gcpp_InferenceArgs_destructor(self.iargs);
            hwy_ThreadPool_destructor(self.pool);
            hwy_ThreadPool_destructor(self.inner_pool);
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

extern fn stream_token(ctx: *mut ffi::c_void, token: ffi::c_int, _: ffi::c_float) -> ffi::c_char {
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
        (*gctx).out.send(StreamSegment::Content(decoded.unwrap())).is_ok() as ffi::c_char
    }
}

extern fn accept_token(_ctx: *mut ffi::c_void, _token: ffi::c_int) -> ffi::c_char {
    true as ffi::c_char
}

impl GemmaContext {
    pub fn generate<'a>(&mut self, prompt: String, out: mpsc::UnboundedSender<StreamSegment>) {
        unsafe {
            if gcpp_InferenceArgs_Multiturn(self.iargs) != 0 {
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
            gcpp_Gemma_Encode(self.gemma, prompt_text.as_mut_ptr() as *mut ffi::c_char, prompt_text.len() as ffi::c_uint, tokens);
            let mut genctx = GenerateContext{
                gemma: self.gemma,
                pos: self.pos,
                tokens_processed: 0,
                input_tokens: std_vector_int_size(tokens) as u32,
                out: out,
            };
            gcpp_GenerateGemma(
                self.gemma, self.iargs,
                tokens, self.pos, self.pool, self.inner_pool,
                (&mut genctx as *mut GenerateContext) as *mut ffi::c_void, stream_token,
                std::ptr::null_mut(), accept_token, self.gen, 0);
            self.pos = genctx.pos;
            std_vector_int_destructor(tokens);
        }
    }

    pub fn max_generated_tokens(&self) -> u32 {
        unsafe {
            gcpp_InferenceArgs_MaxGeneratedTokens(self.iargs)
        }
    }
}

impl Tokenizer for GemmaContext {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        unsafe {
            let mut doc_copied = String::from(doc);
            let tokens = std_vector_int_vector();
            let result = gcpp_Gemma_Encode(self.gemma, doc_copied.as_mut_ptr() as *mut ffi::c_char, doc.len() as ffi::c_uint, tokens);
            if result == 0 {
                return Err(TokenizerError::ToStringError);
            }
            Ok(TokenCollection::from(Vec::from_iter(std_vector_int_iter::new(tokens))))
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