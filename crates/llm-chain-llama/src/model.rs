use llm_chain_llama_sys::{llama_model_default_params, llama_model_params, LLAMA_MAX_DEVICES};
use std::ptr::null_mut;

// Represents the configuration parameters for a LLama model.
#[derive(Debug, Clone)]
pub struct ModelParams {
    pub n_gpu_layers: i32,
    pub main_gpu: i32,
    pub tensor_split: Option<Vec<f32>>,
    pub vocab_only: bool,
    pub use_mmap: bool,
    pub use_mlock: bool,
}

impl ModelParams {
    pub fn new() -> ModelParams {
        unsafe { llama_model_default_params() }.into()
    }
    // Returns the default parameters or the user-specified parameters.
    pub(crate) fn or_default(params: Option<&ModelParams>) -> llama_model_params {
        match params {
            Some(params) => params.clone().into(),
            None => unsafe { llama_model_default_params() },
        }
    }
}

impl Default for ModelParams {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ModelParams> for llama_model_params {
    fn from(params: ModelParams) -> Self {
        let tensor_split = if let Some(tensor_split_vec) = params.tensor_split {
            tensor_split_vec.as_ptr() as *const f32
        } else {
            std::ptr::null()
        };
        llama_model_params {
            n_gpu_layers: params.n_gpu_layers,
            main_gpu: params.main_gpu,
            tensor_split,
            vocab_only: params.vocab_only,
            use_mmap: params.use_mmap,
            use_mlock: params.use_mlock,
            progress_callback: None,
            progress_callback_user_data: null_mut(),
        }
    }
}

impl From<llama_model_params> for ModelParams {
    fn from(params: llama_model_params) -> Self {
        let tensor_split = unsafe {
            if params.tensor_split.is_null() {
                None
            } else {
                let slice =
                    std::slice::from_raw_parts(params.tensor_split, LLAMA_MAX_DEVICES as usize);
                Some(slice.to_vec())
            }
        };
        ModelParams {
            n_gpu_layers: params.n_gpu_layers,
            main_gpu: params.main_gpu,
            tensor_split,
            vocab_only: params.vocab_only,
            use_mmap: params.use_mmap,
            use_mlock: params.use_mlock,
        }
    }
}
