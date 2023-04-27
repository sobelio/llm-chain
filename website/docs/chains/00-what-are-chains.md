# What are LLM chains and why are they useful?

Chains are a concept in the world of language models designed to model common patterns for applying large language models (LLMs) to a sequence of tasks. Although the term "chain" might suggest that it strictly involves chaining together LLM steps, the name has stuck, and it is now used more broadly.

Chains provide a convenient abstraction for organizing and executing a series of LLM steps in various ways to achieve desired outcomes. In this document, we will explore three main types of chains: Sequential, MapReduce, and Conversation chains. Each chain has its unique characteristics and serves specific purposes in applying LLMs.

## Sequential Chains
Sequential chains are a simple yet powerful approach to applying LLMs. They connect multiple steps together in a sequence, where the output of the first step becomes the input of the second step, and so on. This method allows for straightforward processing of information, where each step builds upon the results of the previous one.

## MapReduce Chains
MapReduce chains are designed to work with one or more documents. They split the documents into chunks that fit the LLM's context window and then apply the Map prompt to each chunk. After processing the chunks, a Reduce prompt is used to combine the results into a final output.

This approach is particularly useful when working with large documents or multiple documents, as it enables parallel processing and efficient combination of results.

## Conversation Chains
Conversation chains are tailored for chat-style use cases, where maintaining a conversation history with the LLM is essential. This chain type keeps building up a history of chat messages, removing the ones that do not fit the context window, starting from the oldest to the newest. The conversation chain allows for more dynamic and interactive experiences when working with LLMs.

In summary, chains are a useful concept in applying LLMs, as they provide a structured way of organizing and executing LLM steps for various tasks. Each chain type has its unique characteristics and advantages, and choosing the right chain for your specific use case can significantly improve the effectiveness of your LLM application.
