---
slug: introducing-llm-chain-v060
title: "Introducing LLM-chain v0.6.0: Powerful Templating and Improved Prompt System"
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

We are thrilled to announce the release of llm-chain v0.6.0, which introduces significant enhancements to our library. This update focuses on making the llm-chain more robust and versatile, allowing developers to build even more advanced applications with ease.

### Major updates

#### 1. The switch to the `tera` template language

One of the most significant changes in this release is the introduction of the `tera` template language. This powerful and flexible templating system enables developers to create dynamic and complex templates for their projects. The `tera` language allows for more advanced control structures and filters, making it a substantial upgrade from the previous templating system.

#### 2. Improved prompt system

Another notable update is the revamped prompt system. With llm-chain v0.6.0, the prompt system now supports both Chat and completion-style models. This improvement means developers no longer need to worry about whether they are using a completion or chat model when crafting prompts. This unified approach simplifies the development process and makes it easier to work with various types of language models.

#### 3. Updated LLaMA.cpp

The latest version of LLaMA.cpp has been integrated into this release, ensuring better performance and stability for your projects.

### Other improvements

#### 1. Safer error handling

In addition to the major updates, llm-chain v0.6.0 also brings improvements to error handling. Templates now return `Result` rather than panicking on errors, making it more convenient to handle any issues that may arise during development. Similarly, Executors also return `Result` instead of panicking on errors, providing a more consistent and safer API.

### Time to move on from the old templating system

With the introduction of the `tera` template language, we strongly recommend moving away from the old templating system. This update provides a solid foundation for building even more advanced applications using the llm-chain library.

We hope you're as excited about these enhancements as we are! As always, we appreciate your feedback and support. If you have any questions or need help, please don't hesitate to reach out on [Discord](https://discord.gg/kewN9Gtjt2) !

Happy coding! 🚀
