# Changelog

All notable changes to this project will be documented in this file.

<!-- next-header -->

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased

## [0.6.0] 2023-04-17

### Added

- You can now create templates using the `tera` template language
- New prompt system shipped, prompts can now be used by both Chat and completion style models
- Updated LLaMA.cpp to the latest version

### Changed

- Templates now return Results rather than panic on errors
- Executors now return results rather than panic on errors

## [0.5.0] 2023-04-14

### Added

- Added new `llm_chain::parsing::find_yaml` for finding YAML output in LLMs
- Add support for generating embeddings for text, for now only OpenAI supported, but extensive for other packages
- Add support for the quadrant vector store with the possibility to extend it for other vector stores.

### Changed

- Tools system revamped to give proper errors and use new YAML parsing.
- Templates changed to support `tera` templates, and static string non-template templates.
- Prompt templates and model names can now be &str

### Removed

- The `llm-chain-tools` has been moved into the main `llm-tools` crate.

<!-- next-url -->
