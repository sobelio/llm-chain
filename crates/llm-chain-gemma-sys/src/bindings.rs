use std::ffi;

pub type gcpp_Model = ffi::c_int;
pub const gcpp_Model_GEMMA_2B: gcpp_Model = 0;
pub const gcpp_Model_GEMMA_7B: gcpp_Model = 1;

pub type gcpp_ModelTraining = ffi::c_int;
pub const gcpp_ModelTraining_GEMMA_IT: gcpp_ModelTraining = 0;
pub const gcpp_ModelTraining_GEMMA_PT: gcpp_ModelTraining = 1;

pub const EOS_ID: i32 = 1;

#[repr(C)]
pub struct gcpp_RuntimeConfig {
    pub max_tokens: ffi::c_uint,
    pub max_generated_tokens: ffi::c_uint,
    pub temperature: ffi::c_float,
    pub verbosity: ffi::c_int,
}

#[repr(C)]
pub struct hwy_ThreadPool {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    pub fn hwy_ThreadPool_ThreadPool(num_threads: ffi::c_uint) -> *mut hwy_ThreadPool;
    pub fn hwy_ThreadPool_destructor(pool: *mut hwy_ThreadPool);
}

#[repr(C)]
pub struct gcpp_Gemma {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    pub fn gcpp_Gemma_Gemma(
        tokenizer_path: *const ffi::c_char, tokenizer_path_len: ffi::c_uint,
        compressed_weights_path: *const ffi::c_char, compressed_weights_path_len: ffi::c_uint,
        weights_path: *const ffi::c_char, weights_path_len: ffi::c_uint,
        model_type: gcpp_Model, pool: *mut hwy_ThreadPool) -> *mut gcpp_Gemma;
    pub fn gcpp_Gemma_destructor(gemma: *mut gcpp_Gemma);
    pub fn gcpp_Gemma_SetModelTraining(gemma: *mut gcpp_Gemma, training: gcpp_ModelTraining);
    pub fn gcpp_Gemma_Encode(gemma: *mut gcpp_Gemma, input: *mut ffi::c_char, len: ffi::c_uint, out: *mut std_vector_int) -> ffi::c_char;
    pub fn gcpp_Gemma_Decode(gemma: *mut gcpp_Gemma, token: ffi::c_int, out: *mut std_string) -> ffi::c_char;
    pub fn gcpp_Gemma_Decodes(gemma: *mut gcpp_Gemma, tokens: *const ffi::c_int, num_tokens: ffi::c_int, out: *mut std_string) -> ffi::c_char;

    pub fn gcpp_GenerateGemma(
        gemma: *mut gcpp_Gemma, config: *const gcpp_RuntimeConfig,
        prompt: *const std_vector_int, start_pos: ffi::c_uint,
        kvcache: *mut gcpp_KVCache, pool: *mut hwy_ThreadPool,
        stream_context: *mut ffi::c_void,
        stream_token: extern fn(*mut ffi::c_void, ffi::c_int, ffi::c_float) -> ffi::c_char,
        gen: *mut std_mt19937,
    );
}

#[repr(C)]
pub struct gcpp_KVCache {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    pub fn gcpp_CreateKVCache(model_type: gcpp_Model) -> *mut gcpp_KVCache;
    pub fn gcpp_KVCache_destructor(cache: *mut gcpp_KVCache);
}

#[repr(C)]
pub struct std_vector_int {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    pub fn std_vector_int_vector() -> *mut std_vector_int;
    pub fn std_vector_int_destructor(v: *mut std_vector_int);
    pub fn std_vector_int_size(v: *const std_vector_int) -> ffi::c_uint;
    pub fn std_vector_int_at(v: *const std_vector_int, i: ffi::c_uint) -> ffi::c_int;
}

pub struct std_vector_int_iter {
    v: *mut std_vector_int,
    i: ffi::c_uint,
}

impl std_vector_int_iter {
    pub fn new(v: *mut std_vector_int) -> std_vector_int_iter {
        std_vector_int_iter{
            v: v,
            i: 0,
        }
    }
}

impl ExactSizeIterator for std_vector_int_iter {
    fn len(&self) -> usize {
        unsafe { std_vector_int_size(self.v) as usize }
    }
}

impl Iterator for std_vector_int_iter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        unsafe {
            if self.i < std_vector_int_size(self.v) {
                let v = std_vector_int_at(self.v, self.i);
                self.i += 1;
                Some(v as i32)
            } else {
                None
            }
        }
    }
}

#[repr(C)]
pub struct std_string {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    pub fn std_string_string() -> *mut std_string;
    pub fn std_string_destructor(s: *mut std_string);
    pub fn std_string_c_str(s: *const std_string) -> *mut ffi::c_char;
}

#[repr(C)]
pub struct std_mt19937 {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    pub fn std_mt19937_mt19937() -> *mut std_mt19937;
    pub fn std_mt19937_destructor(gen: *mut std_mt19937);
    pub fn std_mt19937_seed(gen: *mut std_mt19937, seed: ffi::c_int);
    pub fn std_mt19937_random_seed(gen: *mut std_mt19937);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn create_and_delete_pool() {
        unsafe {
            let pool = hwy_ThreadPool_ThreadPool(1);
            hwy_ThreadPool_destructor(pool);
        }
    }
}