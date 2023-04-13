# Changelog

All notable changes to this project will be documented in this file.

<!-- next-header -->

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

- Added new `llm_chain::parsing::find_yaml` for finding YAML output in LLMs

### Changed

- Tools system revamped to give proper errors and use new YAML parsing.
- Templates changed to support `tera` templates, and static string non-template templates.
- Prompt templates and model names can now be &str

### Removed

- The `llm-chain-tools` has been moved into the main `llm-tools` crate.

<!-- next-url -->
