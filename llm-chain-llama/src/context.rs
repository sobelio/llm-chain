use std::{ffi::CStr, ptr::null_mut};

use anyhow::Result;
use llm_chain::traits;
use llm_chain_llama_sys::{
    llama_context, llama_context_default_params, llama_context_params, llama_eval, llama_free,
    llama_init_from_file, llama_sample_top_p_top_k, llama_token, llama_token_to_str,
};
use serde::{Deserialize, Serialize};

use crate::options::LlamaInvocation;

// Represents the configuration parameters for a LLamaContext.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextParams {
    n_ctx: i32,
    n_parts: i32,
    seed: i32,
    f16_kv: bool,
    logits_all: bool,
    vocab_only: bool,
    use_mlock: bool,
    use_mmap: bool,
    embedding: bool,
}

impl ContextParams {
    // Returns the default parameters or the user-specified parameters.
    pub(crate) fn or_default(params: Option<&ContextParams>) -> llama_context_params {
        match params {
            Some(params) => params.clone().into(),
            None => unsafe { llama_context_default_params() },
        }
    }
}

impl From<ContextParams> for llama_context_params {
    fn from(params: ContextParams) -> Self {
        llama_context_params {
            n_ctx: params.n_ctx,
            n_parts: params.n_parts,
            seed: params.seed,
            f16_kv: params.f16_kv,
            logits_all: params.logits_all,
            vocab_only: params.vocab_only,
            use_mlock: params.use_mlock,
            use_mmap: params.use_mmap,
            embedding: params.embedding,
            progress_callback: None,
            progress_callback_user_data: null_mut(),
        }
    }
}

impl traits::Options for ContextParams {}

// Represents the LLamaContext which wraps FFI calls to the llama.cpp library.
pub(crate) struct LLamaContext {
    ctx: *mut llama_context,
}

impl LLamaContext {
    // Creates a new LLamaContext from the specified file and configuration parameters.
    pub fn from_file_and_params(path: &str, params: Option<&ContextParams>) -> Self {
        let params = ContextParams::or_default(params);
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

    pub fn llama_token_to_str(&self, token: &i32) -> String {
        let c_ptr = unsafe { llama_token_to_str(self.ctx, *token) };
        let native_string = unsafe { CStr::from_ptr(c_ptr) }
            .to_str()
            .unwrap()
            .to_owned();
        native_string
    }

    // Evaluates the given tokens with the specified configuration.
    pub fn llama_eval(
        &self,
        tokens: &[llama_token],
        n_tokens: i32,
        n_past: i32,
        input: &LlamaInvocation,
    ) -> Result<(), ()> {
        let res =
            unsafe { llama_eval(self.ctx, tokens.as_ptr(), n_tokens, n_past, input.n_threads) };
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
