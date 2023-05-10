//! Utilities for working with executors
//!
/// A macro that creates a new executor for a specified model.
///
/// This macro makes it easy to create a new executor for a specific model without having to
/// directly call the constructor functions of the respective executor structs. The macro
/// supports creating executors for ChatGPT and LLaMA models.
///
/// # Usage
///
/// ```ignore
/// # use llm_chain::executor;
/// executor!(); // Creates a ChatGPT executor with default options.
/// ```
///
/// # Examples
///
/// ```ignore
/// // Create a ChatGPT executor with default options.
/// let chatgpt_executor = executor!();
///
/// // Create a ChatGPT executor with custom per-executor options.
/// let chatgpt_executor_with_options = executor!(chatgpt, per_executor_options);
///
/// // Create a ChatGPT executor with custom per-executor and per-invocation options.
/// let chatgpt_executor_with_both_options = executor!(chatgpt, per_executor_options, per_invocation_options);
///
/// // Create a LLaMA executor with default options.
/// let llama_executor = executor!(llama);
///
/// // Create a LLaMA executor with custom per-executor options.
/// let llama_executor_with_options = executor!(llama, per_executor_options);
///
/// // Create a LLaMA executor with custom per-executor and per-invocation options.
/// let llama_executor_with_both_options = executor!(llama, per_executor_options, per_invocation_options);
/// ```
///
/// # Parameters
///
/// - `()` or `chatgpt`: Creates a ChatGPT executor with default options.
/// - `chatgpt, per_executor_options`: Creates a ChatGPT executor with custom per-executor options.
/// - `chatgpt, per_executor_options, per_invocation_options`: Creates a ChatGPT executor with custom per-executor and per-invocation options.
/// - `llama`: Creates a LLaMA executor with default options.
/// - `llama, per_executor_options`: Creates a LLaMA executor with custom per-executor options.
/// - `llama, per_executor_options, per_invocation_options`: Creates a LLaMA executor with custom per-executor and per-invocation options.s
#[macro_export]
macro_rules! executor {
    () => {
        executor!(chatgpt)
    };
    (chatgpt) => {{
        use llm_chain::traits::Executor;
        llm_chain_openai::chatgpt::Executor::new()
    }};
    (chatgpt, $options:expr) => {{
        use llm_chain::traits::Executor;
        llm_chain_openai::chatgpt::Executor::new_with_options(Some($options), None)
    }};
    (chatgpt, $per_executor_options:expr, $per_invocation_options:expr) => {{
        use $crate::traits::Executor;
        llm_chain_openai::chatgpt::Executor::new_with_options(
            Some($per_executor_options),
            Some($per_invocation_options),
        )
    }};
    (llama) => {{
        use llm_chain::traits::Executor;
        llm_chain_llama::Executor::new()
    }};
    (llama, $options:expr) => {{
        use llm_chain::traits::Executor;
        llm_chain_llama::Executor::new_with_options(Some($options), None)
    }};
    (llama, $per_executor_options:expr, $per_invocation_options:expr) => {{
        use llm_chain::traits::Executor;
        llm_chain_llama::Executor::new_with_options(
            Some($per_executor_options),
            Some($per_invocation_options),
        )
    }};
}
