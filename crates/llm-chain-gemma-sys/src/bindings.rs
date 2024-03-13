use std::ffi;

pub type gcpp_Model = ffi::c_int;
pub const gcpp_Model_GEMMA_2B: gcpp_Model = 0;
pub const gcpp_Model_GEMMA_7B: gcpp_Model = 1;

pub type gcpp_ModelTraining = ffi::c_int;
pub const gcpp_ModelTraining_GEMMA_IT: gcpp_ModelTraining = 0;
pub const gcpp_ModelTraining_GEMMA_PT: gcpp_ModelTraining = 1;

pub const EOS_ID: i32 = 1;

#[repr(C)]
pub struct gcpp_LoaderArgs {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    pub fn gcpp_LoaderArgs_LoaderArgs(argc: ffi::c_int, argv: *mut *mut ffi::c_char) -> *mut gcpp_LoaderArgs;
    pub fn gcpp_LoaderArgs_destructor(largs: *mut gcpp_LoaderArgs);
    pub fn gcpp_LoaderArgs_Validate(largs: *mut gcpp_LoaderArgs) -> *const ffi::c_char;
    pub fn gcpp_LoaderArgs_ModelType(largs: *const gcpp_LoaderArgs) -> gcpp_Model;
    pub fn gcpp_LoaderArgs_ModelTraining(largs: *const gcpp_LoaderArgs) -> gcpp_ModelTraining;
    pub fn gcpp_LoaderArgs_SetTokenizer(largs: *mut gcpp_LoaderArgs, path: *const ffi::c_char, n: ffi::c_uint);
    pub fn gcpp_LoaderArgs_Tokenizer(largs: *const gcpp_LoaderArgs) -> *mut ffi::c_char;
    pub fn gcpp_LoaderArgs_SetModel(largs: *mut gcpp_LoaderArgs, path: *const ffi::c_char, n: ffi::c_uint);
    pub fn gcpp_LoaderArgs_Model(largs: *const gcpp_LoaderArgs) -> *mut ffi::c_char;
    pub fn gcpp_LoaderArgs_SetCache(largs: *mut gcpp_LoaderArgs, path: *const ffi::c_char, n: ffi::c_uint);
    pub fn gcpp_LoaderArgs_Cache(largs: *const gcpp_LoaderArgs) -> *mut ffi::c_char;
    pub fn gcpp_LoaderArgs_SetModelTypeValue(largs: *mut gcpp_LoaderArgs, s: *const ffi::c_char, n: ffi::c_uint);
    pub fn gcpp_LoaderArgs_ModelTypeValue(largs: *const gcpp_LoaderArgs) -> *mut ffi::c_char;
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
    pub fn gcpp_Gemma_Gemma(args: *mut gcpp_LoaderArgs, pool: *mut hwy_ThreadPool) -> *mut gcpp_Gemma;
    pub fn gcpp_Gemma_destructor(gemma: *mut gcpp_Gemma);
    pub fn gcpp_Gemma_Encode(gemma: *mut gcpp_Gemma, input: *mut ffi::c_char, len: ffi::c_uint, out: *mut std_vector_int) -> ffi::c_char;
    pub fn gcpp_Gemma_Decode(gemma: *mut gcpp_Gemma, token: ffi::c_int, out: *mut std_string) -> ffi::c_char;
    pub fn gcpp_Gemma_Decodes(gemma: *mut gcpp_Gemma, tokens: *const ffi::c_int, num_tokens: ffi::c_int, out: *mut std_string) -> ffi::c_char;

    pub fn gcpp_GenerateGemma(
        gemma: *mut gcpp_Gemma, args: *mut gcpp_InferenceArgs,
        prompt: *const std_vector_int, start_pos: ffi::c_uint,
        pool: *mut hwy_ThreadPool, inner_pool: *mut hwy_ThreadPool,
        stream_context: *mut ffi::c_void,
        stream_token: extern fn(*mut ffi::c_void, ffi::c_int, ffi::c_float) -> ffi::c_char,
        accept_context: *mut ffi::c_void,
        accept_token: extern fn(*mut ffi::c_void, ffi::c_int) -> ffi::c_char,
        gen: *mut std_mt19937, verbosity: ffi::c_int,
    );
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
pub struct gcpp_InferenceArgs {
    _data: [u8; 0],
    _marker:
        core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    pub fn gcpp_InferenceArgs_InferenceArgs(argc: ffi::c_int, argv: *mut *mut ffi::c_char) -> *mut gcpp_InferenceArgs;
    pub fn gcpp_InferenceArgs_destructor(args: *mut gcpp_InferenceArgs);
    pub fn gcpp_InferenceArgs_Validate(args: *mut gcpp_InferenceArgs) -> *const ffi::c_char;

    pub fn gcpp_InferenceArgs_MaxTokens(args: *const gcpp_InferenceArgs) -> ffi::c_uint;
    pub fn gcpp_InferenceArgs_SetMaxTokens(args: *mut gcpp_InferenceArgs, mt: ffi::c_uint);
    pub fn gcpp_InferenceArgs_MaxGeneratedTokens(args: *const gcpp_InferenceArgs) -> ffi::c_uint;
    pub fn gcpp_InferenceArgs_SetMaxGeneratedTokens(args: *mut gcpp_InferenceArgs, mgt: ffi::c_uint);
    pub fn gcpp_InferenceArgs_Temperature(args: *const gcpp_InferenceArgs) -> ffi::c_float;
    pub fn gcpp_InferenceArgs_SetTemperature(args: *mut gcpp_InferenceArgs, t: ffi::c_float);
    pub fn gcpp_InferenceArgs_Deterministic(args: *const gcpp_InferenceArgs) -> ffi::c_char;
    pub fn gcpp_InferenceArgs_SetDeterministic(args: *mut gcpp_InferenceArgs, d: ffi::c_char);
    pub fn gcpp_InferenceArgs_Multiturn(args: *const gcpp_InferenceArgs) -> ffi::c_char;
    pub fn gcpp_InferenceArgs_SetMultiturn(args: *mut gcpp_InferenceArgs, mt: ffi::c_char);
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
    fn create_and_delete_largs() {
        let args = vec![
            "prog",
            "--tokenizer", "tokenizer.spm",
            "--model", "2b-pt",
            "--compressed_weights", "2b-pt.sbs",
        ];
        unsafe {
            let largs = gcpp_LoaderArgs_LoaderArgs(
                args.len() as ffi::c_int,
                Vec::from_iter(args.into_iter().map(|arg|
                    ffi::CString::new(arg).unwrap().into_raw()
                ).into_iter()).as_mut_ptr(),
            );
            assert_eq!(gcpp_Model_GEMMA_2B, gcpp_LoaderArgs_ModelType(largs));
            assert_eq!(gcpp_ModelTraining_GEMMA_PT, gcpp_LoaderArgs_ModelTraining(largs));
            let tp = gcpp_LoaderArgs_Tokenizer(largs);
            let s = ffi::CStr::from_ptr(tp).to_str().unwrap();
            assert_eq!(s, "tokenizer.spm");
            gcpp_LoaderArgs_destructor(largs);
        }
    }

    #[test]
    fn create_and_delete_largs_direct() {
        let tokenizer_path = "tokenizer.spm";
        let compressed_weights = "2b-pt.sbs";
        let model = "2b-pt";
        unsafe {
            let largs = gcpp_LoaderArgs_LoaderArgs(0, std::ptr::null_mut());
            gcpp_LoaderArgs_SetTokenizer(largs, tokenizer_path.as_ptr() as *const i8, tokenizer_path.len() as ffi::c_uint);
            gcpp_LoaderArgs_SetCache(largs, compressed_weights.as_ptr() as *const i8, compressed_weights.len() as ffi::c_uint);
            gcpp_LoaderArgs_SetModelTypeValue(largs, model.as_ptr() as *const i8, model.len() as ffi::c_uint);
            let err = gcpp_LoaderArgs_Validate(largs);
            if err != std::ptr::null_mut() {
                println!("{}", ffi::CStr::from_ptr(err).to_str().unwrap());
            }
            assert_eq!(std::ptr::null(), err);
        }
    }

    #[test]
    fn create_and_delete_iargs_direct() {
        unsafe {
            let iargs = gcpp_InferenceArgs_InferenceArgs(0, std::ptr::null_mut());
            assert_eq!(gcpp_InferenceArgs_Validate(iargs), std::ptr::null());

            assert_eq!(gcpp_InferenceArgs_MaxGeneratedTokens(iargs), 2048);
            assert_eq!(gcpp_InferenceArgs_MaxTokens(iargs), 3072);

            gcpp_InferenceArgs_SetMaxGeneratedTokens(iargs, 4096);

            assert_ne!(gcpp_InferenceArgs_Validate(iargs), std::ptr::null());

            gcpp_InferenceArgs_destructor(iargs);
        }
    }

    #[test]
    fn create_and_delete_pool() {
        unsafe {
            let pool = hwy_ThreadPool_ThreadPool(1);
            hwy_ThreadPool_destructor(pool);
        }
    }
}