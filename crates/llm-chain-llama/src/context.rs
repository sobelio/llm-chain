use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

use crate::options::LlamaInvocation;
use anyhow::Result;
use llm_chain_llama_sys::{
    llama_context, llama_context_default_params, llama_context_params, llama_eval, llama_free,
    llama_get_logits, llama_init_from_file, llama_n_vocab,
    llama_sample_frequency_and_presence_penalties, llama_sample_repetition_penalty,
    llama_sample_tail_free, llama_sample_temperature, llama_sample_token,
    llama_sample_token_greedy, llama_sample_token_mirostat, llama_sample_token_mirostat_v2,
    llama_sample_top_k, llama_sample_top_p, llama_sample_typical, llama_token_data,
    llama_token_data_array, llama_token_nl, llama_token_to_str,
};

#[derive(Debug, thiserror::Error)]
#[error("LLAMA.cpp returned error-code {0}")]
pub struct LLAMACPPErrorCode(i32);

// Represents the configuration parameters for a LLamaContext.
#[derive(Debug, Clone)]
pub struct ContextParams {
    pub n_ctx: i32,
    pub n_batch: i32,
    pub n_gpu_layers: i32,
    pub main_gpu: i32,
    pub tensor_split: *const f32,
    pub seed: u32,
    pub f16_kv: bool,
    pub vocab_only: bool,
    pub use_mlock: bool,
    pub use_mmap: bool,
    pub embedding: bool,
    pub low_vram: bool,
    pub rope_freq_base: f32,
    pub rope_freq_scale: f32,
    pub mul_mat_q: bool,
    pub n_gqa: i32,
    pub rms_norm_eps: f32,
}

unsafe impl Sync for ContextParams {}
unsafe impl Send for ContextParams {}

impl ContextParams {
    pub fn new() -> ContextParams {
        unsafe { llama_context_default_params() }.into()
    }
    // Returns the default parameters or the user-specified parameters.
    pub(crate) fn or_default(params: Option<&ContextParams>) -> llama_context_params {
        match params {
            Some(params) => params.clone().into(),
            None => unsafe { llama_context_default_params() },
        }
    }
}

impl Default for ContextParams {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ContextParams> for llama_context_params {
    fn from(params: ContextParams) -> Self {
        llama_context_params {
            n_ctx: params.n_ctx,
            n_batch: params.n_batch,
            n_gpu_layers: params.n_gpu_layers,
            main_gpu: params.main_gpu,
            tensor_split: params.tensor_split,
            seed: params.seed,
            f16_kv: params.f16_kv,
            logits_all: false,
            vocab_only: params.vocab_only,
            use_mlock: params.use_mlock,
            use_mmap: params.use_mmap,
            embedding: params.embedding,
            progress_callback: None,
            progress_callback_user_data: null_mut(),
            low_vram: params.low_vram,
            rope_freq_base: params.rope_freq_base,
            rope_freq_scale: params.rope_freq_scale,
            mul_mat_q: params.mul_mat_q,
            n_gqa: params.n_gqa,
            rms_norm_eps: params.rms_norm_eps,
        }
    }
}

impl From<llama_context_params> for ContextParams {
    fn from(params: llama_context_params) -> Self {
        ContextParams {
            n_ctx: params.n_ctx,
            n_batch: params.n_batch,
            n_gpu_layers: params.n_gpu_layers,
            main_gpu: params.main_gpu,
            tensor_split: params.tensor_split,
            seed: params.seed,
            f16_kv: params.f16_kv,
            vocab_only: params.vocab_only,
            use_mlock: params.use_mlock,
            use_mmap: params.use_mmap,
            embedding: params.embedding,
            low_vram: params.low_vram,
            rope_freq_base: params.rope_freq_base,
            rope_freq_scale: params.rope_freq_scale,
            mul_mat_q: params.mul_mat_q,
            n_gqa: params.n_gqa,
            rms_norm_eps: params.rms_norm_eps,
        }
    }
}

// Represents the LLamaContext which wraps FFI calls to the llama.cpp library.
pub(crate) struct LLamaContext {
    ctx: *mut llama_context,
}

impl LLamaContext {
    // Creates a new LLamaContext from the specified file and configuration parameters.
    pub fn from_file_and_params(
        path: &str,
        params: Option<&ContextParams>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let path = CString::new(path).expect("could not convert to CString");
        let params = ContextParams::or_default(params);
        let ctx = unsafe { llama_init_from_file(path.into_raw() as *const i8, params) };
        if ctx.is_null() {
            return Err("Initializing llama context returned nullptr".into());
        }
        Ok(Self { ctx })
    }

    // Token logits obtained from the last call to llama_eval()
    // The logits for the last token are stored in the last row
    // Can be mutated in order to change the probabilities of the next token
    // Rows: n_tokens
    // Cols: n_vocab
    pub fn llama_get_logits_as_slice(&self, n_tokens: usize, n_vocab: usize) -> Vec<f32> {
        let len = n_tokens * n_vocab;
        unsafe { std::slice::from_raw_parts_mut(llama_get_logits(self.ctx), len) }.to_vec()
    }
    pub fn llama_n_vocab(&self) -> i32 {
        unsafe { llama_n_vocab(self.ctx) }
    }

    // Executes the LLama sampling process with the specified configuration.
    pub fn llama_sample(
        &self,
        n_ctx: i32,
        last_n_tokens_data: &[i32],
        last_n_tokens_size: i32,
        input: &LlamaInvocation,
    ) -> i32 {
        let top_k = if input.top_k <= 0 {
            self.llama_n_vocab()
        } else {
            input.top_k
        };
        let repeat_last_n = if input.repeat_last_n < 0 {
            n_ctx
        } else {
            input.repeat_last_n
        };
        let n_vocab = self.llama_n_vocab() as usize;
        // only get the last row, as the sample only requires this.
        let mut logits = self.llama_get_logits_as_slice(1, n_vocab);

        // let id : llama_token = 0;
        input
            .logit_bias
            .iter()
            .for_each(|(k, v)| logits[*k as usize] += v);
        let mut candidates: Vec<llama_token_data> = Vec::with_capacity(n_vocab);
        (0..n_vocab).for_each(|i| {
            candidates.push(llama_token_data {
                id: i as i32,
                logit: logits[i],
                p: input.top_p,
            })
        });
        let mut candidates_p = llama_token_data_array {
            data: candidates.as_mut_ptr(),
            size: candidates.len(),
            sorted: false,
        };
        let nl_logit = logits[unsafe { llama_token_nl() } as usize];
        let last_n_repeat = i32::min(i32::min(last_n_tokens_size, repeat_last_n), n_ctx) as usize;

        unsafe {
            llama_sample_repetition_penalty(
                self.ctx,
                &mut candidates_p,
                last_n_tokens_data
                    .as_ptr()
                    .add((last_n_tokens_size - last_n_repeat as i32) as usize),
                last_n_repeat,
                input.repeat_penalty,
            )
        };
        unsafe {
            llama_sample_frequency_and_presence_penalties(
                self.ctx,
                &mut candidates_p,
                last_n_tokens_data
                    .as_ptr()
                    .add((last_n_tokens_size - last_n_repeat as i32) as usize),
                last_n_repeat,
                input.frequency_penalty,
                input.presence_penalty,
            )
        };
        if !input.penalize_nl {
            logits[unsafe { llama_token_nl() as usize }] = nl_logit;
        }

        if input.temp <= 0.0 {
            // Greedy sampling
            unsafe { llama_sample_token_greedy(self.ctx, &mut candidates_p) }
        } else if input.mirostat == 1 {
            let mut mirostat_mu = 2.0 * input.mirostat_tau;
            let mirostat_m = 100_i32;
            unsafe { llama_sample_temperature(self.ctx, &mut candidates_p, input.temp) };
            unsafe {
                llama_sample_token_mirostat(
                    self.ctx,
                    &mut candidates_p,
                    input.mirostat_tau,
                    input.mirostat_eta,
                    mirostat_m,
                    &mut mirostat_mu,
                )
            }
        } else if input.mirostat == 2 {
            let mut mirostat_mu = 2.0 * input.mirostat_tau;
            unsafe { llama_sample_temperature(self.ctx, &mut candidates_p, input.temp) };
            unsafe {
                llama_sample_token_mirostat_v2(
                    self.ctx,
                    &mut candidates_p,
                    input.mirostat_tau,
                    input.mirostat_eta,
                    &mut mirostat_mu,
                )
            }
        } else {
            // Temperature sampling
            unsafe { llama_sample_top_k(self.ctx, &mut candidates_p, top_k, 1) };
            unsafe { llama_sample_tail_free(self.ctx, &mut candidates_p, input.tfs_z, 1) };
            unsafe { llama_sample_typical(self.ctx, &mut candidates_p, input.typical_p, 1) };
            unsafe { llama_sample_top_p(self.ctx, &mut candidates_p, input.top_p, 1) };
            unsafe { llama_sample_temperature(self.ctx, &mut candidates_p, input.temp) };
            unsafe { llama_sample_token(self.ctx, &mut candidates_p) }
        }
    }

    pub fn llama_token_to_bytes(&self, token: &i32) -> Vec<u8> {
        let c_ptr = unsafe { llama_token_to_str(self.ctx, *token) };
        unsafe { CStr::from_ptr(c_ptr) }.to_bytes().to_vec()
    }

    // Evaluates the given tokens with the specified configuration.
    pub fn llama_eval(
        &self,
        tokens: &[i32],
        n_tokens: i32,
        n_past: i32,
        input: &LlamaInvocation,
    ) -> Result<(), LLAMACPPErrorCode> {
        let res =
            unsafe { llama_eval(self.ctx, tokens.as_ptr(), n_tokens, n_past, input.n_threads) };
        if res == 0 {
            Ok(())
        } else {
            Err(LLAMACPPErrorCode(res))
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
