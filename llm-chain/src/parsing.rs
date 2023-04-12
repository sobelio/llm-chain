use markdown::{
    mdast::{Code, Node},
    to_mdast, ParseOptions,
};
use serde::de::DeserializeOwned;
use serde_yaml::Value;
use thiserror::Error;

// The main error type for extraction.
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

    let mut found: Vec<T> = Vec::new();
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
                        Ok(o) => found.push(o),
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
        Ok(found)
    } else {
        Err(current_error.into())
    }
}
