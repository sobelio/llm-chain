use std::ffi::{CStr, CString};

use crate::batch;
use crate::model::ModelParams;
use crate::options::LlamaInvocation;
use anyhow::Result;
use llm_chain_llama_sys::{
    llama_context, llama_context_default_params, llama_context_params, llama_decode, llama_eval,
    llama_free, llama_get_logits, llama_get_logits_ith, llama_load_model_from_file, llama_model,
    llama_n_vocab, llama_new_context_with_model, llama_sample_repetition_penalties,
    llama_sample_tail_free, llama_sample_temperature, llama_sample_token,
    llama_sample_token_greedy, llama_sample_token_mirostat, llama_sample_token_mirostat_v2,
    llama_sample_top_k, llama_sample_top_p, llama_sample_typical, llama_token_data,
    llama_token_data_array, llama_token_eos, llama_token_get_text, llama_token_nl,
    llama_token_to_piece,
};

pub use batch::LlamaBatch;

#[derive(Debug, thiserror::Error)]
#[error("LLAMA.cpp returned error-code {0}")]
pub struct LLAMACPPErrorCode(i32);

// Represents the configuration parameters for a LLamaContext.
#[derive(Debug, Clone)]
pub struct ContextParams {
    pub seed: u32,
    pub n_ctx: u32,
    pub n_batch: u32,
    pub n_threads: u32,
    pub n_threads_batch: u32,
    pub rope_scaling_type: i8,
    pub rope_freq_base: f32,
    pub rope_freq_scale: f32,
    pub yarn_ext_factor: f32,
    pub yarn_attn_factor: f32,
    pub yarn_beta_fast: f32,
    pub yarn_beta_slow: f32,
    pub yarn_orig_ctx: u32,
    pub mul_mat_q: bool,
    pub f16_kv: bool,
    pub logits_all: bool,
    pub embedding: bool,
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
            seed: params.seed,
            n_ctx: params.n_ctx,
            n_batch: params.n_batch,
            n_threads: params.n_threads,
            n_threads_batch: params.n_threads_batch,
            rope_scaling_type: params.rope_scaling_type,
            rope_freq_base: params.rope_freq_base,
            rope_freq_scale: params.rope_freq_scale,
            yarn_ext_factor: params.yarn_ext_factor,
            yarn_attn_factor: params.yarn_attn_factor,
            yarn_beta_fast: params.yarn_beta_fast,
            yarn_beta_slow: params.yarn_beta_slow,
            yarn_orig_ctx: params.yarn_orig_ctx,
            mul_mat_q: params.mul_mat_q,
            f16_kv: params.f16_kv,
            logits_all: false,
            embedding: params.embedding,
        }
    }
}

impl From<llama_context_params> for ContextParams {
    fn from(params: llama_context_params) -> Self {
        ContextParams {
            seed: params.seed,
            n_ctx: params.n_ctx,
            n_batch: params.n_batch,
            n_threads: params.n_threads,
            n_threads_batch: params.n_threads_batch,
            rope_scaling_type: params.rope_scaling_type,
            rope_freq_base: params.rope_freq_base,
            rope_freq_scale: params.rope_freq_scale,
            yarn_ext_factor: params.yarn_ext_factor,
            yarn_attn_factor: params.yarn_attn_factor,
            yarn_beta_fast: params.yarn_beta_fast,
            yarn_beta_slow: params.yarn_beta_slow,
            yarn_orig_ctx: params.yarn_orig_ctx,
            mul_mat_q: params.mul_mat_q,
            f16_kv: params.f16_kv,
            logits_all: params.logits_all,
            embedding: params.embedding,
        }
    }
}

// Represents the LLamaContext which wraps FFI calls to the llama.cpp library.
pub(crate) struct LLamaContext {
    ctx: *mut llama_context,
    pub model: *mut llama_model,
}

#[allow(dead_code)]
impl LLamaContext {
    // Creates a new LLamaContext from the specified file and configuration parameters.
    pub fn from_file_and_params(
        path: &str,
        model_params: Option<&ModelParams>,
        context_params: Option<&ContextParams>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let path = CString::new(path).expect("could not convert to CString");
        let model_params = ModelParams::or_default(model_params);
        let model =
            unsafe { llama_load_model_from_file(path.into_raw() as *const i8, model_params) };
        if model.is_null() {
            return Err("Initializing llama model returned nullptr".into());
        }

        let context_params = ContextParams::or_default(context_params);
        let ctx = unsafe { llama_new_context_with_model(model, context_params) };
        if ctx.is_null() {
            return Err("Initializing llama context returned nullptr".into());
        }
        Ok(Self { ctx, model })
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
        unsafe { llama_n_vocab(self.model) }
    }

    pub fn llama_get_logits_ith(&self, index: usize) -> Vec<f32> {
        let float_ptr = unsafe { llama_get_logits_ith(self.ctx, index as i32) };
        Vec::from(unsafe { std::slice::from_raw_parts(float_ptr, self.llama_n_vocab() as usize) })
    }

    // Executes the LLama sampling process with the specified configuration.
    pub fn llama_sample(
        &self,
        n_ctx: i32,
        last_n_tokens_data: &[i32],
        last_n_tokens_size: i32,
        input: &LlamaInvocation,
        batch_n_tokens: i32,
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
        let mut logits = self.llama_get_logits_ith((batch_n_tokens - 1) as usize);

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
        let nl_logit = logits[unsafe { llama_token_nl(self.model) } as usize];
        let last_n_repeat = i32::min(i32::min(last_n_tokens_size, repeat_last_n), n_ctx) as usize;

        unsafe {
            llama_sample_repetition_penalties(
                self.ctx,
                &mut candidates_p,
                last_n_tokens_data
                    .as_ptr()
                    .add((last_n_tokens_size - last_n_repeat as i32) as usize),
                last_n_repeat,
                input.repeat_penalty,
                input.frequency_penalty,
                input.presence_penalty,
            )
        };
        if !input.penalize_nl {
            logits[unsafe { llama_token_nl(self.model) as usize }] = nl_logit;
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
        let c_ptr = unsafe { llama_token_get_text(self.model, *token) };
        unsafe { CStr::from_ptr(c_ptr) }.to_bytes().to_vec()
    }

    // Evaluates the given tokens with the specified configuration.
    pub fn llama_eval(
        &self,
        tokens: &mut [i32],
        n_tokens: i32,
        n_past: i32,
        _input: &LlamaInvocation,
    ) -> Result<(), LLAMACPPErrorCode> {
        let res = unsafe { llama_eval(self.ctx, tokens.as_mut_ptr(), n_tokens, n_past) };
        if res == 0 {
            Ok(())
        } else {
            Err(LLAMACPPErrorCode(res))
        }
    }

    // Evaluates the provided batch.
    pub fn llama_decode(&self, batch: &LlamaBatch) -> Result<(), LLAMACPPErrorCode> {
        let res = unsafe { llama_decode(self.ctx, batch.into()) };
        if res == 0 {
            Ok(())
        } else {
            Err(LLAMACPPErrorCode(res))
        }
    }

    pub fn llama_token_eos(&self) -> i32 {
        unsafe { llama_token_eos(self.model) }
    }

    pub fn llama_token_nl(&self) -> i32 {
        unsafe { llama_token_nl(self.model) }
    }

    pub fn llama_token_to_piece(
        &self,
        token_id: i32,
    ) -> Result<String, std::string::FromUtf8Error> {
        let mut result = vec![0 as i8; 8];
        let n_tokens = unsafe {
            llama_token_to_piece(
                self.model,
                token_id,
                result.as_mut_ptr(),
                result.len() as i32,
            )
        };
        if n_tokens < 0 {
            result.resize(-n_tokens as usize, 0 as i8);
            let check = unsafe {
                llama_token_to_piece(
                    self.model,
                    token_id,
                    result.as_mut_ptr(),
                    result.len() as i32,
                )
            };
            assert_eq!(check, -n_tokens);
        } else {
            result.resize(n_tokens as usize, 0 as i8);
        }
        let result_bytes: Vec<u8> = result.into_iter().map(|b| b as u8).collect();
        String::from_utf8(result_bytes)
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
