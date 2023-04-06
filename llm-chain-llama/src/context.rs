use std::ptr::null_mut;

use anyhow::Result;
use llama_sys::{
    llama_context, llama_context_default_params, llama_context_params, llama_eval, llama_free,
    llama_init_from_file, llama_sample_top_p_top_k, llama_token,
};

use crate::step::LlamaInvocation;

// Represents the configuration parameters for a LLamaContext.
#[derive(Debug, Clone)]
pub struct LlamaContextParams {
    n_ctx: i32,
    n_parts: i32,
    seed: i32,
    f16_kv: bool,
    logits_all: bool,
    vocab_only: bool,
    use_mlock: bool,
    embedding: bool,
}

impl LlamaContextParams {
    // Returns the default parameters or the user-specified parameters.
    pub(crate) fn or_default(params: &Option<LlamaContextParams>) -> llama_context_params {
        match params {
            Some(params) => params.clone().into(),
            None => unsafe { llama_context_default_params() },
        }
    }
}

impl From<LlamaContextParams> for llama_context_params {
    fn from(params: LlamaContextParams) -> Self {
        llama_context_params {
            n_ctx: params.n_ctx,
            n_parts: params.n_parts,
            seed: params.seed,
            f16_kv: params.f16_kv,
            logits_all: params.logits_all,
            vocab_only: params.vocab_only,
            use_mlock: params.use_mlock,
            embedding: params.embedding,
            progress_callback: None,
            progress_callback_user_data: null_mut(),
        }
    }
}

// Represents the LLamaContext which wraps FFI calls to the llama.cpp library.
pub(crate) struct LLamaContext {
    ctx: *mut llama_context,
}

impl LLamaContext {
    // Creates a new LLamaContext from the specified file and configuration parameters.
    pub fn from_file_and_params(path: &str, params: &Option<LlamaContextParams>) -> Self {
        let params = LlamaContextParams::or_default(params);
        let ctx = unsafe { llama_init_from_file(path.as_ptr() as *const i8, params) };
        Self { ctx }
    }

    // Executes the LLama sampling process with the specified configuration.
    pub fn llama_sample(
        &self,
        last_n_tokens_data: &[llama_token],
        last_n_tokens_size: i32,
        input: &LlamaInvocation,
    ) -> i32 {
        unsafe {
            llama_sample_top_p_top_k(
                self.ctx,
                last_n_tokens_data.as_ptr(),
                last_n_tokens_size,
                input.top_k,
                input.top_p,
                input.temp,
                input.repeat_penalty,
            )
        }
    }

    // Evaluates the given tokens with the specified configuration.
    pub fn llama_eval(
        &self,
        tokens: &[llama_token],
        n_tokens: i32,
        n_past: i32,
        input: &LlamaInvocation,
    ) -> Result<(), ()> {
        let res = unsafe {
            llama_eval(
                self.ctx,
                tokens.as_ptr(),
                n_tokens,
                n_past as i32,
                input.n_threads,
            )
        };
        if res == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

// Provides thread-safe behavior for LLamaContext.
unsafe impl Send for LLamaContext {}
unsafe impl Sync for LLamaContext {}

// Enables dereferencing LLamaContext to access the underlying *mut llama_context.
impl std::ops::Deref for LLamaContext {
    type Target = *mut llama_context;
    fn deref(&self) -> &*mut llama_context {
        &self.ctx
    }
}

// Handles proper cleanup of the llama_context when the LLamaContext is dropped.
impl Drop for LLamaContext {
    fn drop(&mut self) {
        unsafe { llama_free(self.ctx) };
    }
}
