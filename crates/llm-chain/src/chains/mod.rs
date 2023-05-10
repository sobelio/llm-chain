//! Chains are multi-step modes of execution for LLMs such as Sequential or MapReduce.
//!
//! This module contains submodules for various types of chains. Chains are powerful tools that allow you to connect multiple steps together in a sequence. They take a set of parameters and an executor, perform the steps, and return the result.
//!
//! Currently, we support two types of chains that cater to different use cases. But worry not! We will be adding more in the future
//!
//! Here are the supported chain types:
//! 1. **Sequential**: This chain type executes the steps one after another in a linear sequence. It's perfect for tasks that need a clear and simple order of execution.
//! 2. **MapReduce**: This chain type follows the MapReduce paradigm, where the steps are divided into mapping and reducing phases. It's great for tasks that require parallel processing and data aggregation.
//! 3. **Converstation**: This chain type models a conversation between the LLM and some other entity. It's great for tasks that require a back-and-forth between the LLM and the user.
//! Stay tuned for more chain types, and feel free to contribute your own! 🎉

pub mod conversation;
pub mod map_reduce;
pub mod sequential;
