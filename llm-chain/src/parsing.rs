//! Functions for parsing the output of LLMs, including YAML and Markdown.
//!
//! This module provides a set of functions that allow you to parse and extract useful information from the output of Large Language Models (LLMs) in YAML and Markdown formats. These functions can be used to transform the LLM output into a more structured and usable format, enabling seamless integration with your applications.
//!
//! Key features include:
//! - Parsing YAML and Markdown content produced by LLMs
//! - Handling common edge cases and being lenient with LLM outputs
//! - Extracting and deserializing YAML objects from text
//!
//! With these functions, you can easily work with the outputs of LLMs, simplifying the process of integrating LLMs into your applications and workflows.

use markdown::{
    mdast::{Code, Node, Text},
    to_mdast, ParseOptions,
};
use serde::de::DeserializeOwned;
use serde_yaml::Value;
use std::collections::VecDeque;
use thiserror::Error;

/// Errors occuring when parsing
#[derive(Error, Debug)]
#[error(transparent)]
pub struct ExtractionError(#[from] ExtractionErrorImpl);

/// Enum representing the different error cases that can occur during YAML extraction.
#[derive(Error, Debug)]
enum ExtractionErrorImpl {
    /// The YAML content was valid, but it did not match the expected format.
    #[error("The YAML was valid, but it didn't match the expected format: {0}")]
    YamlFoundButFormatWrong(serde_yaml::Error),

    /// An error occurred while parsing the YAML content.
    #[error("YAML parsing failed with: {0}")]
    ParseError(#[from] serde_yaml::Error),

    /// No YAML content was found to parse.
    #[error("The string to parse was empty")]
    NoneFound,
}

impl ExtractionErrorImpl {
    /// Determines the most representative error between two instances of `ExtractionErrorImpl`.
    ///
    /// The function prefers `YamlFoundButFormatWrong` errors over `ParseError` errors,
    /// and `ParseError` errors over `NoneFound` errors.
    fn most_representative(a: Self, b: Self) -> Self {
        match (&a, &b) {
            (Self::YamlFoundButFormatWrong(_), _) => a,
            (_, Self::YamlFoundButFormatWrong(_)) => b,
            (Self::ParseError(_), _) => a,
            (_, Self::ParseError(_)) => b,
            _ => a,
        }
    }
}

/// Attempts to extract YAML content from a given code block and deserialize it into the specified type.
///
/// # Arguments
///
/// * `code_block` - A string slice containing the YAML content to be extracted and deserialized.
///
/// # Returns
///
/// * `Ok(T)` - If the YAML content is successfully extracted and deserialized into the specified type.
/// * `Err(ExtractionErrorImpl)` - If an error occurs during extraction or deserialization.
///
/// # Type Parameters
///
/// * `T: DeserializeOwned` - The type into which the extracted YAML content should be deserialized.
fn extract_yaml<T: DeserializeOwned>(code_block: &str) -> Result<T, ExtractionErrorImpl> {
    // Ensure that the input code block is not empty.
    let code_block = Some(code_block)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ExtractionErrorImpl::NoneFound)?;

    // Parse the code block as YAML.
    let yaml: Value = serde_yaml::from_str(code_block)?;

    // Attempt to deserialize the YAML into the specified type, handling any format issues.
    serde_yaml::from_value(yaml).map_err(ExtractionErrorImpl::YamlFoundButFormatWrong)
}

/// Attempts to find a YAML object in a string and deserialize it into the specified type.
/// # Arguments
///
/// * `text` - A string slice containing the document which may contain YAML content.
///
/// # Returns
///
/// * `Ok(Vec<T>)` - If the YAML content is successfully extracted and deserialized into the specified type.
/// * `Err(ExtractionError)` - If an error occurs during extraction or deserialization.
///
/// # Type Parameters
///
/// * `T: DeserializeOwned` - The type into which the extracted YAML content should be deserialized.
///
/// # Examples
///
/// It handles the obvious case where it is just YAML.
///
/// ```
/// #[derive(serde::Deserialize)]
/// struct Dummy {
///    hello: String
/// }
/// use llm_chain::parsing::find_yaml;
/// let data = "
/// hello: world
/// ";
/// let data: Vec<Dummy> = find_yaml(data).unwrap();
/// assert_eq!(data[0].hello, "world");
/// ```
///
/// It handles the case where it is in a code block.
///
/// ```
/// use llm_chain::parsing::find_yaml;
/// // NOTE: we are escaping the backticks because this is a doc test.
/// let data = "
/// \u{60}``yaml
/// hello: world
/// \u{60}``
/// ";
/// find_yaml::<serde_yaml::Value>(data).unwrap();
/// ```
pub fn find_yaml<T: DeserializeOwned>(text: &str) -> Result<Vec<T>, ExtractionError> {
    let mut current_error = ExtractionErrorImpl::NoneFound;
    if text.is_empty() {
        return Err(current_error.into());
    }

    // Attempt YAML parsing if it doesn't look like markdown output.
    if !text.starts_with("```") {
        match extract_yaml(text) {
            Ok(o) => return Ok(vec![o]),
            Err(e) => current_error = ExtractionErrorImpl::most_representative(current_error, e),
        }
    }

    // Parse the input text as markdown.
    let options = ParseOptions::default();
    let ast = to_mdast(text, &options).expect("we're not using MDX, so this should never fail");

    // Nodes to visit.
    let mut nodes = vec![ast];

    let mut found: VecDeque<_> = VecDeque::new();
    while let Some(node) = nodes.pop() {
        if let Some(children) = node.children() {
            children.iter().for_each(|child| nodes.push(child.clone()));
        }

        // Check for code blocks containing YAML.
        if let Node::Code(Code { value, lang, .. }) = node {
            let lang = lang.unwrap_or_default();
            match lang.as_str() {
                "yaml" | "yml" | "json" | "" => {
                    let code_block = value.as_str();
                    match extract_yaml(code_block) {
                        Ok(o) => found.push_front(o),
                        Err(e) => {
                            current_error =
                                ExtractionErrorImpl::most_representative(current_error, e)
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if !found.is_empty() {
        Ok(found.into())
    } else {
        Err(current_error.into())
    }
}

/// Extracts labeled text from markdown
///
/// LLMs often generate text that looks something like this
/// ```markdown
/// - *foo*: bar
/// - hello: world
/// ```
/// Which we want to parse as key value pairs (foo, bar), (hello, world).
///
/// # Parameters
/// - `text` the text to parse
///
/// # Returns
/// Vec<(String, String)> A vector of key value pairs.
///
/// # Examples
///
/// ```
/// use llm_chain::parsing::extract_labeled_text;
/// let data = "
/// - alpha: beta
/// - *gamma*: delta
/// ";
/// let labs = extract_labeled_text(data);
/// println!("{:?}", labs);
/// assert_eq!(labs[0], ("alpha".to_string(), "beta".to_string()));
/// ```
pub fn extract_labeled_text(text: &str) -> Vec<(String, String)> {
    let options = ParseOptions::default();
    let ast = to_mdast(text, &options).expect("markdown parsing can't fail");
    let mut nodes = VecDeque::new();
    nodes.push_back(ast);
    let mut extracted_labels = Vec::new();

    while let Some(node) = nodes.pop_front() {
        let found = match &node {
            Node::Text(Text { value, .. }) => extract_label_and_text(value.to_owned())
                .map(|(label, text)| (label.to_owned(), text.to_owned())),
            Node::Paragraph(_) | Node::ListItem(_) => {
                find_labeled_text(&node).map(|(label, text)| (label.to_owned(), text.to_owned()))
            }
            _ => None,
        };
        if let Some(kv) = found {
            // If found push to found
            extracted_labels.push(kv)
        } else if let Some(children) = node.children() {
            // If not found recur into it.
            let mut index = 0;
            for child in children.iter().cloned() {
                nodes.insert(index, child);
                index += 1;
            }
        }
    }
    extracted_labels
}

/// Finds labeled text
///
/// This function looks for patterns such as `**label**: text.
///
/// Returns an option indicating whether a label was found, and if so, the label and text.
fn find_labeled_text(n: &Node) -> Option<(String, String)> {
    if let Node::Text(Text { value, .. }) = n {
        extract_label_and_text(value.to_owned())
    } else {
        let children = n.children()?;
        // There should be exactly two children...
        if children.len() == 2 {
            let key = children
                .get(0)
                .map(inner_text)
                .map(format_key)
                .filter(|k| !k.is_empty());
            let value = children.get(1).map(inner_text).map(format_value);
            key.and_then(|key| value.map(|value| (key, value)))
        } else {
            None
        }
    }
}

fn extract_label_and_text(text: String) -> Option<(String, String)> {
    let value_split: Vec<&str> = text.splitn(2, ':').collect();

    if value_split.len() == 2 {
        let label = value_split[0].trim().to_string();
        if label.is_empty() {
            return None;
        }
        let text = value_split[1].trim().to_string();
        Some((label, text))
    } else {
        None
    }
}

/// Returns the inner text
fn inner_text(n: &Node) -> String {
    if let Node::Text(Text { value, .. }) = n {
        return value.to_owned();
    }
    let mut deq = VecDeque::new();
    deq.push_back(n.clone());
    let mut text = String::new();
    while let Some(node) = deq.pop_front() {
        if let Some(children) = node.children() {
            deq.extend(children.iter().cloned());
        }
        if let Node::Text(Text { value, .. }) = node {
            text.push_str(value.as_str());
        }
    }
    text
}

// Formats the key trimming it and remvove a potential ":" suffix
fn format_key(s: String) -> String {
    let key = s.trim();
    key.strip_suffix(":").unwrap_or(key).to_owned()
}

// Formats the value trimming, stripping potential ":" and then retrimming the start
fn format_value(s: String) -> String {
    s.trim()
        .strip_prefix(":")
        .unwrap_or(&s)
        .trim_start()
        .to_owned()
}
