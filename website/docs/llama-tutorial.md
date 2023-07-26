# Tutorial: Getting Started using the LLAMA driver

In this tutorial, you will learn how to set up an llm-project using the LLAMA drive. If you wish to use the other drivers you can skip this part of the tutorial.

## Prerequisites

To follow this tutorial, you will need:

- Ubuntu Linux 18.04 or higher
- Rust 1.71.0 or higher
- Cargo, the Rust package manager
- GCC/G++ 8 or higher
- A Hugging Face account1
- Git and Git LFS
- Pyenv, a Python version manager
- Python 3.11.3 or higher
- cmake, libclang-dev

We tested using these exact software versions, but you should be able to get similar results using the latest versions. Also, there are many alternative ways to install and use these products. For example install Python via your Linux distribution's package manager. You can adapt this tutorial to your environment.

## Step 1: Create a new Rust project

First, you will create a new Rust project using Cargo. To create a new project, open a terminal and run the following command:

```
cargo new --bin llm-chain-demo
cd llm-chain-demo
```

This will create a new directory called `llm-chain-demo` with the following structure:

```
llm-chain-demo
├── Cargo.toml
└── src
    └── main.rs
```

The `Cargo.toml` file contains the metadata and dependencies of your project. The `src/main.rs` file contains the main source code of your project.

## Step 2: Add dependencies

To add these dependencies, run the following commands in your terminal:

```
cargo add llm-chain
cargo add tokio --features all
```
## Step 3: Update Rust

The minimum version required to run this tutorial is Rust 1.65.0. At the time of this writing, the latest stable version is 1.71.0, so we'll use that. 

To switch to Rust 1.71.0, you need to use `rustup`, which is a tool that helps you manage multiple versions of Rust on your system.

To install `rustup`, follow the instruction on [https://rustup.rs/](https://rustup.rs/)

To install Rust 1.71.0 using `rustup`, run the following command in your terminal:

```
rustup install 1.71.0
```

To switch to Rust 1.71.0 as the default version for your project, run the following command in your terminal:

```
rustup default 1.71.0
```

You can verify that you are using Rust 1.71.0 by running the following command in your terminal:

```
rustc --version
```

This should output something like this:

```
rustc 1.71.0 (8ede3aae2 2023-07-12)
```

## Step 4: Install and run llama.cpp

Now that you have set up your Rust project and switched to the correct version of Rust, you need to install and run llama.cpp, which is a C++ implementation of LLaMa models for inference. 

llama.cpp requires GCC and G++ 8 or newer, if your distribution use GCC/G++ 7 by default, use these commands to update:

```
sudo apt install gcc-8 g++-8
sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-8 10
sudo update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-8 10
sudo update-alternatives --install /usr/bin/cc cc /usr/bin/gcc 30
sudo update-alternatives --set cc /usr/bin/gcc

sudo update-alternatives --install /usr/bin/c++ c++ /usr/bin/g++ 30
sudo update-alternatives --set c++ /usr/bin/g++
```

To install llama.cpp, you need to clone its repository from GitHub and build it from source. To do that, run the following commands in your terminal:

```
git clone https://github.com/ggerganov/llama.cpp
cd llama.cpp
make
```

Notice that we clone the `llama.cpp` folder inside the `llm-chain-demo` folder. 

To run llama.cpp, you need to download a LLaMa model and convert it to a binary format that llama.cpp can read. In this tutorial, you will use the Alpaca model.

To download the Alpaca model, you need to have a Hugging Face account and install Git LFS. Hugging Face is a platform that hosts and distributes various natural language processing models, including LLaMa models. Git LFS is an extension for Git that allows you to store large files on GitHub. Because these LLMs are usually quite big, Hugging Face use Git LFS to allow you to download them using git.

To create a Hugging Face account, go to [Hugging Face](https://huggingface.co/) and sign up.

To install Git LFS, run the following commands in your terminal:

```
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.deb.sh | sudo bash
sudo apt install git-lfs
git lfs install
```

This will download and install Git LFS on your system.

To download the Alpaca model, run the following commands in your terminal:

```
cd ./models
git clone https://huggingface.co/chavinlo/alpaca-native
```

This will clone the Alpaca model repository to your `models` directory.

To convert the Alpaca model to the format llama.cpp accepts, you need to install Python and run a conversion script. In this tutorial, you will use Python 3.11.3, which is the latest stable version of Python at the time of writing this tutorial.

To install Python 3.11.3, you need to use `pyenv`, which is a tool that helps you manage multiple versions of Python on your system.

To install `pyenv`, run the following command in your terminal:

```
curl https://pyenv.run | bash
```

Then, you need to install the `pyenv-virtualenv` plugin to let `pyenv` manage virtualenv for you. Run this command to install `pyenv-virtualenv`:

```
git clone https://github.com/pyenv/pyenv-virtualenv.git $(pyenv root)/plugins/pyenv-virtualenv
```

If you use zsh, add the following lines to your `~/.zshrc` file, run the following command in your terminal:

```
echo 'eval "$(pyenv init -)"' >> ~/.zshrc
echo 'eval "$(pyenv virtualenv-init -)"' >> ~/.zshrc
source ~/.zshrc
```
Or replace it with `~/.bashrc` if you prefer bash.


This will enable `pyenv` to manage your Python versions and virtual environments.

To install Python 3.11.3 using `pyenv`, run the following command in your terminal:

```
pyenv install 3.11.3
```

To create a virtual environment for Python 3.11.3 using `pyenv`, run the following command in your terminal:

```
pyenv virtualenv 3.11.3 llama
```

To activate the virtual environment, run the following command in your terminal:

```
pyenv activate llama
```

This will activate the virtual environment and change your prompt to indicate that you are using it.

To install the required Python packages for the conversion script, run the following command in your terminal:

```
# in the llama.cpp root directory
pip install -r requirements.txt
```

With the Python dependencies installed, you need to run the conversion script that will convert the Alpaca model to a binary format that llama.cpp can read. To do that, run the following command in your terminal:

```
python convert.py /models/alpaca-native
```

This will run the `convert.py` script that is located in the `llama.cpp` directory. The script will take the Alpaca model directory as an argument and output a binary file called `ggml-model-f32.bin` in the same directory. 

To test llama.cpp with the Alpaca model, run the following command in your terminal:

```
./main -m models/alpaca-native/ggml-model-f32.bin -n 128 -p "I love Rust because"
```

This will run the `main` executable that is located in the `llama.cpp` directory. The executable will take two arguments: `-m`, `-n` and `-p` The `-m` argument specifies the path to the model binary file. The `-n` argument specifies the number of tokens to generate for each input. The `-p` argument specifies the prompt you send to the LLM.

The model may take some time to load. You should see something like this in your terminal:

```
main: build = 849 (d01bccd)
main: seed  = 1690205678
llama.cpp: loading model from models/alpaca-native/ggml-model-f32.bin
llama_model_load_internal: format     = ggjt v1 (pre #1405)
...
llama_new_context_with_model: kv self size  =  256.00 MB

system_info: n_threads = 4 / 8 | AVX = 1 | AVX2 = 1 | AVX512 = 1 | AVX512_VBMI = 0 | AVX512_VNNI = 0 | FMA = 1 | NEON = 0 | ARM_FMA = 0 | F16C = 1 | FP16_VA = 0 | WASM_SIMD = 0 | BLAS = 0 | SSE3 = 1 | VSX = 0 | 
sampling: repeat_last_n = 64, repeat_penalty = 1.100000, presence_penalty = 0.000000, frequency_penalty = 0.000000, top_k = 40, tfs_z = 1.000000, top_p = 0.950000, typical_p = 1.000000, temp = 0.800000, mirostat = 0, mirostat_lr = 0.100000, mirostat_ent = 5.000000
generate: n_ctx = 512, n_batch = 512, n_predict = 128, n_keep = 0

 I love Rust because it allows me to create things that are both practical and beautiful. I can design objects that are functional, reliable, and secure - all while still looking great. It’s also a fun language to work with, as it encourages creativity through its focus on code spelunking and efficient algorithmic thinking. [end of text]

llama_print_timings:        load time = 113290.18 ms
...
llama_print_timings:       total time = 44284.74 ms
```

This means that llama.cpp has successfully loaded the Alpaca model and generated text based on your prompt.


## Step 5: Add the llama.cpp driver

Now we can get back to the `llm-chain-demo` Rust project. To use the LLAMA driver, you need to add it as a dependency to your Rust project. You can run the following command in the terminal:

```
cargo add llm-chain-llama
```

## Step 6: Run the example code

To run the example code, you need to copy and paste it into your `src/main.rs` file. The example code creates a LLAMA executor with the Alpaca model and generates text for a given prompt.

```rust
use llm_chain::executor;
use llm_chain::{parameters, prompt};
use llm_chain::options::*;
use llm_chain::options;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let opts = options!(
		Model: ModelRef::from_path("./llama.cpp/models/alpaca-native/ggml-model-f32.bin"), // Notice that we reference the model binary path
		ModelType: "llama",
		MaxContextSize: 512_usize,
		NThreads: 4_usize,
		MaxTokens: 0_usize,
		TopK: 40_i32,
		TopP: 0.95,
		TfsZ: 1.0,
		TypicalP: 1.0,
		Temperature: 0.8,
		RepeatPenalty: 1.1,
		RepeatPenaltyLastN: 64_usize,
		FrequencyPenalty: 0.0,
		PresencePenalty: 0.0,
		Mirostat: 0_i32,
		MirostatTau: 5.0,
		MirostatEta: 0.1,
		PenalizeNl: true,
		StopSequence: vec!["\n".to_string()]
	);
	let exec = executor!(llama, opts)?;
	let res = prompt!("I love Rust because")
		.run(
			&parameters!(),
			&exec,
		)
		.await?;
	println!("{}", res.to_immediate().await?);
	Ok(())
}
```

When we set up llama.cpp, we use `make` to compile it. But llama.cpp can also be compiled with `cmake`. the `llm-chain-llama` crate uses `llm-chain-llama-sys` internally, and `llm-chain-llama.sys` uses `cmake` to compile the bindings for llama.cpp. Before you run the example code, you may need to install some additional packages for the compilation, such as `libclang-dev` and `cmake`. To install `libclang-dev`, run the following command in your terminal:

```
sudo apt install libclang-dev
```

This will install the Clang library development files on your system.

To install `cmake`, you can to use a PPA from Kitware, which provides the latest version of `cmake`. You can also compile from source if you have concerns using 3rd-party PPA. To do that, run the following commands in your terminal:

```
wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc 2>/dev/null | gpg --dearmor - | sudo tee /etc/apt/trusted.gpg.d/kitware.gpg >/dev/null
sudo apt-add-repository "deb https://apt.kitware.com/ubuntu/ $(lsb_release -cs) main"
sudo apt update
sudo apt install kitware-archive-keyring
sudo apt update
sudo apt install cmake
```

To run your program, run the following command in your terminal:

```
cargo run
```

You should see something like this in your terminal:

```
   Compiling llm-chain-demo v0.1.0 (/home/ubuntu/environment/llm-chain-demo)
    Finished dev [unoptimized + debuginfo] target(s) in 9.05s
     Running `target/debug/llm-chain-demo`
...
I love Rust because it allows me to create things that are both practical and beautiful. I can design objects that are functional, reliable, and secure - all while still looking great. It’s also a fun language to work with, as it encourages creativity through its focus on code spelunking and efficient algorithmic thinking. [end of text]
...
```

This is the text generated through the llm-chain and the LLAMA driver based on your prompt.

Congratulations! You have successfully run the example code using the llama.cpp driver. You can experiment with different models, model parameters and prompts.