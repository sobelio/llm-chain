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
