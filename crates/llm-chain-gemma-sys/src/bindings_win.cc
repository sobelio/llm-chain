#include <cstddef>

extern "C" {

void* hwy_ThreadPool_ThreadPool(size_t num_threads) {
    return nullptr;
}

void hwy_ThreadPool_destructor(void* pool) {
}

void* gcpp_Gemma_Gemma(
    const char* tokenizer_path, size_t tokenizer_path_len,
    const char* compressed_weights_path, size_t compressed_weights_path_len,
    const char* weights_path, size_t weights_path_len,
    int model_type, void* pool) {
    return nullptr;
}

void gcpp_Gemma_destructor(void* gemma) {
}

void gcpp_Gemma_SetModelTraining(void* gemma, int training) {
}

void* gcpp_CreateKVCache(int model_type) {
    return nullptr;
}

void gcpp_KVCache_destructor(void* kvcache) {
}

void* std_vector_int_vector() {
    return nullptr;
}

void std_vector_int_destructor(void* v) {
}

size_t std_vector_int_size(const void* v) {
    return 0;
}

int std_vector_int_at(void* v, size_t i) {
    return 0;
}

void* std_string_string() {
    return nullptr;
}

void std_string_destructor(void* s) {
}

const char* std_string_c_str(const void* s) {
    return nullptr;
}

bool gcpp_Gemma_Encode(void* gemma, const char* input, size_t len, void* out) {
    return false;
}

bool gcpp_Gemma_Decode(void* gemma, int token, void* out) {
    return false;
}

bool gcpp_Gemma_Decodes(void* gemma, const int* tokens, int num_tokens, void* out) {
    return false;
}

void* std_mt19937_mt19937() {
    return nullptr;
}

void std_mt19937_destructor(void* gen) {
}

void std_mt19937_seed(void* gen, int seed) {
}

void std_mt19937_random_seed(void* gen) {
}

typedef bool (*stream_callback)(void*, int, float);
typedef bool (*accept_callback)(void*, int);

void gcpp_GenerateGemma(
    void* gemma, const void* config,
    const void* prompt, size_t start_pos,
    void* kvcache, void* pool,
    void* stream_context, stream_callback stream_token,
    void* gen) {
}

}