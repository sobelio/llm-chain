# llm-chain-llama 🦙

Welcome to LLM-Chain-LLaMa, a powerful and versatile driver for LLaMa-style models! This crate leverages the amazing [llama.cpp](https://github.com/ggerganov/llama.cpp) library, making it simple and efficient to run LLaMa, Alpaca, and similar models in a Rust environment.

## Getting Started 🏁

To begin, you'll need to acquire a LLaMa model and adapt it for `llama.cpp`. Don't worry; we've got your back! Just follow the instructions from [llama.cpp](https://github.com/ggerganov/llama.cpp#usage) and you'll be up and running in no time. 🦾

## Features 🌟

LLM-Chain-LLaMa is packed with all the features you need to harness the full potential of LLaMa, Alpaca, and similar models. Here's a glimpse of what's inside:

- Running chained LLaMa-style models in a Rust environment, taking your applications to new heights 🌄
- Prompts for working with `instruct` models, empowering you to easily build virtual assistants amazing applications 🧙‍♂️

So gear up and dive into the fantastic world of LLM-Chain-LLaMa! Let the power of LLaMa-style models propel your projects to the next level. Happy coding, and enjoy the ride! 🎉🥳


## CUDA Support
This requires the [CUDA toolkit] to be installed on the system. CUDA support can
then be enabled by setting the following environment variables:
* LLM_CHAIN_CUDA  
This should be set to `true` to enable CUDA support.

* LLM_CHAIN_CUDA_LIB_PATH  
This should be set to the path of the CUDA library directory. For example, on
Fedora, this could be `/usr/local/cuda-12.2/lib64`.


Example of building with CUDA support:
```console
$ env LLM_CHAIN_CUDA_LIB_PATH=/usr/local/cuda-12.2/lib64 LLM_CHAIN_CUDA=true cargo b -vv
```
Using `-vv` will enable the output from llama.cpp build process to be displayed
which can be useful for debugging build issues.

[CUDA toolkit]: https://developer.nvidia.com/cuda-downloads
```
