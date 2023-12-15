use llm_chain_llama_sys::{llama_batch, llama_batch_free, llama_seq_id};
use std::ptr::null_mut;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LlamaBatch {
    n_tokens: i32,
    token: Vec<i32>,
    embd: Vec<f32>,
    pos: Vec<i32>,
    n_seq_id: Vec<i32>,
    seq_id: Vec<Vec<i32>>,
    logits: Vec<bool>,
    all_pos_0: i32,
    all_pos_1: i32,
    all_seq_id: i32,
}

impl LlamaBatch {
    pub fn new_with_tokens(tokens: Vec<i32>, max_seq: i32) -> Self {
        let pos = (0..tokens.len()).map(|p| p as i32).collect::<Vec<_>>();
        let embd = vec![];
        let n_seq_id = vec![max_seq; tokens.len()];
        let seq_id = vec![vec![0; tokens.len()]; tokens.len()];
        let logits = vec![false; tokens.len()];
        let all_pos_0 = 0;
        let all_pos_1 = 0;
        let all_seq_id = 0;

        Self {
            n_tokens: tokens.len() as i32,
            token: tokens,
            embd,
            pos,
            n_seq_id,
            seq_id,
            logits,
            all_pos_0,
            all_pos_1,
            all_seq_id,
        }
    }

    pub fn new_with_token(token: i32, pos: i32) -> Self {
        Self {
            n_tokens: 1,
            token: vec![token],
            embd: vec![],
            pos: vec![pos],
            n_seq_id: vec![1],
            seq_id: vec![vec![0]],
            logits: vec![true],
            all_pos_0: 0,
            all_pos_1: 0,
            all_seq_id: 0,
        }
    }

    pub fn token_count(&self) -> usize {
        self.n_tokens as usize
    }

    pub fn enable_logits(&mut self, pos: usize) {
        self.logits[pos] = true;
    }
}

impl Drop for LlamaBatch {
    fn drop(&mut self) {
        unsafe {
            llama_batch_free(self.into());
        }
    }
}

fn convert_llama_batch(batch: &LlamaBatch) -> llama_batch {
    let n_tokens = batch.n_tokens;
    let token_ptr = Box::leak(batch.token.clone().into_boxed_slice()).as_mut_ptr();
    let embd_ptr = if batch.embd.is_empty() {
        null_mut()
    } else {
        Box::leak(batch.embd.clone().into_boxed_slice()).as_mut_ptr()
    };
    let pos_ptr = Box::leak(batch.pos.clone().into_boxed_slice()).as_mut_ptr();
    let n_seq_id_ptr = Box::leak(batch.n_seq_id.clone().into_boxed_slice()).as_mut_ptr();
    let raw_pointers = batch
        .seq_id
        .clone()
        .into_iter()
        .map(|inner_vec| Box::leak(inner_vec.into_boxed_slice()).as_mut_ptr())
        .collect::<Vec<*mut llama_seq_id>>();
    let seq_id_ptr = Box::leak(raw_pointers.into_boxed_slice()).as_mut_ptr();
    let logits_ptr = Box::leak(batch.logits.clone().into_boxed_slice()).as_mut_ptr();
    llama_batch {
        n_tokens,
        token: token_ptr,
        embd: embd_ptr,
        pos: pos_ptr,
        n_seq_id: n_seq_id_ptr,
        seq_id: seq_id_ptr,
        logits: logits_ptr as *mut i8,
        all_pos_0: batch.all_pos_0,
        all_pos_1: batch.all_pos_1,
        all_seq_id: batch.all_seq_id,
    }
}

impl From<&LlamaBatch> for llama_batch {
    fn from(batch: &LlamaBatch) -> Self {
        convert_llama_batch(batch)
    }
}

impl From<&mut LlamaBatch> for llama_batch {
    fn from(batch: &mut LlamaBatch) -> Self {
        convert_llama_batch(batch)
    }
}
