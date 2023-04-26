---
slug: introducing-llm-chain-v080
title: "Introducing LLM-chain v0.8.0 - Expanding the prompt system"
authors: [whn]
tags:
  [
    llm-chain,
    update,
    large language models,
    rust,
    tera,
    templating,
    prompt system,
  ]
---

We're excited to announce the release of llm-chain v0.8.0, a significant update to our LLM library. This release introduces a host of improvements and new features, including a completely revamped Prompt system and more streamlined handling of Parameters. Let's dive into the details!

## Revamped Prompt System

Our new Prompt system has been redesigned from the ground up to provide greater flexibility and efficiency in working with language models. In llm-chain v0.8.0, we've introduced new structs and enums to better represent chat messages and their roles, such as ChatMessage, ChatMessageCollection, and ChatRole. The Data enum has also been introduced to represent either a collection of chat messages or a single text, making it easier to work with different types of data.

Furthermore, we've created a more powerful PromptTemplate system that allows you to format prompts with a set of parameters. This enables you to dynamically generate prompts for your language models without the need for cumbersome string manipulation.

## Executors No Longer Handle Parameters

With the release of llm-chain v0.8.0, we've shifted the responsibility of handling Parameters from the executors to the main llm-chain crate. This change simplifies the process of working with executors, allowing developers to focus more on the core functionality of their language models.

## What's Next?

This release marks a significant step forward in the evolution. However, we're not stopping here! We'll continue to refine and expand the capabilities of llm-chain, making it even more powerful and user-friendly.

We encourage you to check out llm-chain v0.8.0 and experience the benefits of the improved Prompt system and streamlined handling of Parameters. As always, we appreciate your feedback and contributions to help make llm-chain the best language model library out there.

Upgrade to llm-chain v0.8.0 today and take your language models to the next level!
