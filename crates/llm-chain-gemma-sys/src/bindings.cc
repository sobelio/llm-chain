#include <gemma.h>

extern "C" {

gcpp::LoaderArgs* gcpp_LoaderArgs_LoaderArgs(int argc, char* argv[]) {
    return new gcpp::LoaderArgs(argc, argv);
}

void gcpp_LoaderArgs_destructor(gcpp::LoaderArgs* args) {
    delete args;
}

const char* gcpp_LoaderArgs_Validate(gcpp::LoaderArgs* args) {
    return args->Validate();
}

gcpp::Model gcpp_LoaderArgs_ModelType(const gcpp::LoaderArgs* args) {
    return args->ModelType();
}

gcpp::ModelTraining gcpp_LoaderArgs_ModelTraining(const gcpp::LoaderArgs* args) {
    return args->ModelTraining();
}

void gcpp_LoaderArgs_SetTokenizer(gcpp::LoaderArgs* args, char* path) {
    args->tokenizer.path = std::string(path);
}

const char* gcpp_LoaderArgs_Tokenizer(gcpp::LoaderArgs* args) {
    return args->tokenizer.path.c_str();
}

void gcpp_LoaderArgs_SetModel(gcpp::LoaderArgs* args, char* path) {
    args->model.path = std::string(path);
}

const char* gcpp_LoaderArgs_Model(gcpp::LoaderArgs* args) {
    return args->model.path.c_str();
}

void gcpp_LoaderArgs_SetCache(gcpp::LoaderArgs* args, char* path) {
    args->cache.path = std::string(path);
}

const char* gcpp_LoaderArgs_Cache(gcpp::LoaderArgs* args) {
    return args->cache.path.c_str();
}

void gcpp_LoaderArgs_SetModelTypeValue(gcpp::LoaderArgs* args, char* v) {
    args->model_type = std::string(v);
}

const char* gcpp_LoaderArgs_ModelTypeValue(gcpp::LoaderArgs* args) {
    return args->model_type.c_str();
}

hwy::ThreadPool* hwy_ThreadPool_ThreadPool(size_t num_threads) {
    return new hwy::ThreadPool(num_threads);
}

void hwy_ThreadPool_destructor(hwy::ThreadPool* pool) {
    delete pool;
}

gcpp::Gemma* gcpp_Gemma_Gemma(const gcpp::LoaderArgs* args, hwy::ThreadPool* pool) {
    return new gcpp::Gemma(*args, *pool);
}

void gcpp_Gemma_destructor(gcpp::Gemma* gemma) {
    delete gemma;
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
    return gemma->Tokenizer().Encode(std::string(input, len), out).ok();
}

bool gcpp_Gemma_Decode(gcpp::Gemma* gemma, int token, std::string* out) {
    return gemma->Tokenizer().Decode(std::vector<int>{token}, out).ok();
}

bool gcpp_Gemma_Decodes(gcpp::Gemma* gemma, const int* tokens, int num_tokens, std::string* out) {
    std::vector<int> v;
    v.reserve(num_tokens);
    for (int i = 0; i < num_tokens; i++) {
        v.push_back(tokens[i]);
    }
    return gemma->Tokenizer().Decode(v, out).ok();
}

gcpp::InferenceArgs* gcpp_InferenceArgs_InferenceArgs(int argc, char* argv[]) {
    return new gcpp::InferenceArgs(argc, argv);
}

void gcpp_InferenceArgs_destructor(gcpp::InferenceArgs* args) {
    delete args;
}

const char* gcpp_InferenceArgs_Validate(gcpp::InferenceArgs* args) {
    return args->Validate();
}

size_t gcpp_InferenceArgs_MaxTokens(gcpp::InferenceArgs* args) {
    return args->max_tokens;
}

void gcpp_InferenceArgs_SetMaxTokens(gcpp::InferenceArgs* args, size_t mt) {
    args->max_tokens = mt;
}

size_t gcpp_InferenceArgs_MaxGeneratedTokens(gcpp::InferenceArgs* args) {
    return args->max_generated_tokens;
}

void gcpp_InferenceArgs_SetMaxGeneratedTokens(gcpp::InferenceArgs* args, size_t mgt) {
    args->max_generated_tokens = mgt;
}

float gcpp_InferenceArgs_Temperature(gcpp::InferenceArgs* args) {
    return args->temperature;
}

void gcpp_InferenceArgs_SetTemperature(gcpp::InferenceArgs* args, float t) {
    args->temperature = t;
}

bool gcpp_InferenceArgs_Deterministic(gcpp::InferenceArgs* args) {
    return args->deterministic;
}

void gcpp_InferenceArgs_SetDeterministic(gcpp::InferenceArgs* args, bool d) {
    args->deterministic = d;
}

bool gcpp_InferenceArgs_Multiturn(gcpp::InferenceArgs* args) {
    return args->multiturn;
}

void gcpp_InferenceArgs_SetMultiturn(gcpp::InferenceArgs* args, bool mt) {
    args->multiturn = mt;
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
    gcpp::Gemma* gemma, const gcpp::InferenceArgs* args,
    const std::vector<int>* prompt, size_t start_pos,
    hwy::ThreadPool* pool, hwy::ThreadPool* inner_pool,
    void* stream_context,
    stream_callback stream_token,
    void* accept_context,
    accept_callback accept_token,
    std::mt19937* gen, int verbosity) {
    gcpp::GenerateGemma(
        *gemma, *args, *prompt, start_pos,
        *pool, *inner_pool,
        [&stream_context, &stream_token](int token, float value) {
            return stream_token(stream_context, token, value);
        },
        [&accept_context, &accept_token](int token) {
            return accept_token(accept_context, token);
        },
        *gen, verbosity);
}

}