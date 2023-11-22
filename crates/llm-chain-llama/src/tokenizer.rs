use llm_chain::output::Output;
use llm_chain::prompt::Data;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use llm_chain_llama_sys::{llama_token, llama_token_get_text, llama_tokenize};

use crate::context::LLamaContext;

// Helper function to convert a Rust string to a C string.
fn to_cstring(s: &str) -> CString {
    CString::new(s).expect("CString::new failed")
}

/// Converts a llama_token to a Rust String.
///
/// # Arguments
///
/// * `ctx` - A pointer to the llama_context.
/// * `token` - The llama_token to convert to a string.
///
/// # Returns
///
/// A Rust String representation of the given llama_token.
fn to_output(context: &LLamaContext, token: i32) -> String {
    let c_ptr = unsafe { llama_token_get_text(context.model, token) };
    let native_string = unsafe { CStr::from_ptr(c_ptr) }
        .to_string_lossy()
        .into_owned();
    native_string
}

/// Helper function to tokenize text using the provided LLamaContext and add_bos option.
///
/// # Arguments
///
/// * `context` - A reference to the LLamaContext used for tokenization.
/// * `text` - The text to tokenize.
/// * `add_bos` - Whether to add the beginning-of-sentence token.
///
/// # Returns
///
/// A Vec of llama_tokens representing the tokenized input.
pub(crate) fn tokenize(
    context: &LLamaContext,
    text: &str,
    add_bos: bool,
    special: bool,
) -> Vec<llama_token> {
    let mut res = Vec::with_capacity(text.len() + add_bos as usize);
    let c_text = to_cstring(text);
    let n_tokens = unsafe {
        llama_tokenize(
            context.model,
            c_text.as_ptr() as *const c_char,
            c_text.to_bytes().len() as i32,
            res.as_mut_ptr(),
            res.capacity() as i32,
            add_bos,
            special,
        )
    };
    if n_tokens < 0 {
        res.resize(-n_tokens as usize, 0);
        let new_n_tokens = unsafe {
            llama_tokenize(
                context.model,
                c_text.as_ptr() as *const c_char,
                c_text.to_bytes().len() as i32,
                res.as_mut_ptr(),
                res.capacity() as i32,
                add_bos,
                special,
            )
        };
        assert!(new_n_tokens == -n_tokens);
    } else {
        unsafe { res.set_len(n_tokens as usize) };
    }
    res
}

pub(crate) fn tokens_to_string(context: &LLamaContext, embd: &[i32]) -> String {
    let bfr = String::with_capacity(embd.len() * 2);
    embd.iter()
        .map(|token| to_output(context, *token))
        .fold(bfr, |cur, nxt| cur + &nxt)
}

/// Converts an embedding represented as a slice into the Output string.
pub(crate) fn embedding_to_output(context: &LLamaContext, embd: &[i32]) -> Output {
    Output::new_immediate(Data::<String>::text(tokens_to_string(context, embd)))
}
