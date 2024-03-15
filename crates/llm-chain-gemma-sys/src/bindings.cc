#include <gemma.h>

extern "C" {

hwy::ThreadPool* hwy_ThreadPool_ThreadPool(size_t num_threads) {
    return new hwy::ThreadPool(num_threads);
}

void hwy_ThreadPool_destructor(hwy::ThreadPool* pool) {
    delete pool;
}

gcpp::Gemma* gcpp_Gemma_Gemma(
    const char* tokenizer_path, size_t tokenizer_path_len,
    const char* compressed_weights_path, size_t compressed_weights_path_len,
    const char* weights_path, size_t weights_path_len,
    gcpp::Model model_type, hwy::ThreadPool* pool) {
    gcpp::Path tpath;
    tpath.path = std::string(tokenizer_path, tokenizer_path_len);
    gcpp::Path cwpath;
    cwpath.path = std::string(compressed_weights_path, compressed_weights_path_len);
    gcpp::Path wpath;
    wpath.path = std::string(weights_path, weights_path_len);
    return new gcpp::Gemma(tpath, cwpath, wpath, model_type, *pool);
}

void gcpp_Gemma_destructor(gcpp::Gemma* gemma) {
    delete gemma;
}

void gcpp_Gemma_SetModelTraining(gcpp::Gemma* gemma, gcpp::ModelTraining training) {
    gemma->model_training = training;
}

gcpp::KVCache* gcpp_CreateKVCache(gcpp::Model model_type) {
    gcpp::KVCache* cache = new gcpp::KVCache{};
    *cache = gcpp::CreateKVCache(model_type);
    return cache;
}

void gcpp_KVCache_destructor(gcpp::KVCache* kvcache) {
    delete kvcache;
}

std::vector<int>* std_vector_int_vector() {
    return new std::vector<int>();
}

void std_vector_int_destructor(std::vector<int>* v) {
    delete v;
}

size_t std_vector_int_size(const std::vector<int>* v) {
    return v->size();
}

int std_vector_int_at(const std::vector<int>* v, size_t i) {
    return v->at(i);
}

std::string* std_string_string() {
    return new std::string();
}

void std_string_destructor(std::string* s) {
    delete s;
}

const char* std_string_c_str(const std::string* s) {
    return s->c_str();
}

bool gcpp_Gemma_Encode(gcpp::Gemma* gemma, const char* input, size_t len, std::vector<int>* out) {
    return gemma->Tokenizer()->Encode(std::string(input, len), out).ok();
}

bool gcpp_Gemma_Decode(gcpp::Gemma* gemma, int token, std::string* out) {
    return gemma->Tokenizer()->Decode(std::vector<int>{token}, out).ok();
}

bool gcpp_Gemma_Decodes(gcpp::Gemma* gemma, const int* tokens, int num_tokens, std::string* out) {
    std::vector<int> v;
    v.reserve(num_tokens);
    for (int i = 0; i < num_tokens; i++) {
        v.push_back(tokens[i]);
    }
    return gemma->Tokenizer()->Decode(v, out).ok();
}

std::mt19937* std_mt19937_mt19937() {
    return new std::mt19937();
}

void std_mt19937_destructor(std::mt19937* gen) {
    delete gen;
}

void std_mt19937_seed(std::mt19937* gen, int seed) {
    gen->seed(seed);
}

void std_mt19937_random_seed(std::mt19937* gen) {
    std::random_device rd;
    gen->seed(rd());
}

typedef bool (*stream_callback)(void*, int, float);
typedef bool (*accept_callback)(void*, int);

void gcpp_GenerateGemma(
    gcpp::Gemma* gemma, const gcpp::RuntimeConfig* config,
    const std::vector<int>* prompt, size_t start_pos,
    gcpp::KVCache* kvcache, hwy::ThreadPool* pool,
    void* stream_context, stream_callback stream_token,
    std::mt19937* gen) {
    gcpp::GenerateGemma(
        *gemma, *config, *prompt, start_pos, *kvcache, *pool,
        [&stream_context, &stream_token](int token, float value) {
            return stream_token(stream_context, token, value);
        },
        *gen);
}

}