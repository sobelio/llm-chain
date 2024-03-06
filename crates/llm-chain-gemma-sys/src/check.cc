#include <gemma.h>
#include <util/app.h>
#include <iostream>

int main(int argc, char* argv[]) {
    gcpp::LoaderArgs largs(argc, argv);
    gcpp::InferenceArgs iargs(argc, argv);
    gcpp::AppArgs aargs(argc, argv);

    largs.Validate();

    hwy::ThreadPool pool(1);
    hwy::ThreadPool inner_pool(0);

    gcpp::Gemma gemma(largs, pool);

    std::mt19937 gen;
    gen.seed(42);

    std::vector<int> tokens;
    gemma.Tokenizer().Encode(
        "<start_of_turn>user\nWhat is a gemma?<end_of_turn>\n<start_of_turn>model\n", &tokens);
    for (auto token : tokens) {
        std::cout << "token: " << token << std::endl;
    }

    gcpp::GenerateGemma(
        gemma, iargs, tokens, 0,
        pool, inner_pool,
        [&gemma](int token, float value) {
            std::string decoded;
            gemma.Tokenizer().Decode(std::vector<int>{token}, &decoded);
            std::cout << decoded;
            return true;
        },
        [](int token) { return true; },
        gen,
        10
    );
    std::cout << std::endl;
    return 0;
}
