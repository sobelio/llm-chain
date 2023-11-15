use lazy_static::lazy_static;
use paste::paste;
use std::{collections::HashMap, env::VarError, ffi::OsStr};

use serde::{Deserialize, Serialize};
use strum_macros::EnumDiscriminants;

use crate::tokens::Token;

/// A collection of options that can be used to configure a model.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
/// `Options` is the struct that represents a set of options for a large language model.
/// It provides methods for creating, adding, and retrieving options.
///
/// The 'Options' struct is mainly created using the `OptionsBuilder` to allow for
/// flexibility in setting options.
pub struct Options {
    /// The actual options, stored as a vector.
    opts: Vec<Opt>,
}

#[derive(thiserror::Error, Debug)]
/// An error indicating that a required option is not set.
#[error("Option not set")]
struct OptionNotSetError;

lazy_static! {
    /// An empty set of options, useful as a default.
    static ref EMPTY_OPTIONS: Options = Options::builder().build();
}

impl Options {
    /// Constructs a new `OptionsBuilder` for creating an `Options` instance.
    ///
    /// This function serves as an entry point for using the builder pattern to create `Options`.
    ///
    /// # Returns
    ///
    /// An `OptionsBuilder` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use llm_chain::options::*;
    /// let builder = Options::builder();
    /// ```
    pub fn builder() -> OptionsBuilder {
        OptionsBuilder::new()
    }

    /// Returns a reference to an empty set of options.
    ///
    /// This function provides a static reference to an empty `Options` instance,
    /// which can be useful as a default value.
    ///
    /// # Returns
    ///
    /// A reference to an empty `Options`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use llm_chain::options::*;
    /// let empty_options = Options::empty();
    /// ```
    pub fn empty() -> &'static Self {
        &EMPTY_OPTIONS
    }
    /// Gets the value of an option from this set of options.
    ///
    /// This function finds the first option in `opts` that matches the provided `OptDiscriminants`.
    ///
    /// # Arguments
    ///
    /// * `opt_discriminant` - An `OptDiscriminants` value representing the discriminant of the desired `Opt`.
    ///
    /// # Returns
    ///
    /// An `Option` that contains a reference to the `Opt` if found, or `None` if not found.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use llm_chain::options::*;
    /// let mut builder = Options::builder();
    /// builder.add_option(Opt::Model(ModelRef::from_path("path_to_model")));
    /// let options = builder.build();
    /// let model_option = options.get(OptDiscriminants::Model);
    /// ```
    pub fn get(&self, opt_discriminant: OptDiscriminants) -> Option<&Opt> {
        self.opts
            .iter()
            .find(|opt| OptDiscriminants::from(*opt) == opt_discriminant)
    }
}

/// `options!` is a declarative macro that facilitates the creation of an `Options` instance.
///
/// # Usage
///
/// This macro can be used to construct an instance of `Options` using a more readable and
/// ergonomic syntax. The syntax of the macro is:
///
/// ```ignore
/// options!{
///     OptionName1: value1,
///     OptionName2: value2,
///     ...
/// }
/// ```
///
/// Here, `OptionNameN` is the identifier of the option you want to set, and `valueN` is the value
/// you want to assign to that option.
///
/// # Example
///
/// ```ignore
/// let options = options!{
///     FooBar: "lol",
///     SomeReadyMadeOption: "another_value"
/// };
/// ```
///
/// In this example, an instance of `Options` is being created with two options: `FooBar` and
/// `SomeReadyMadeOption`, which are set to `"lol"` and `"another_value"`, respectively.
///
/// # Notes
///
/// - The option identifier (`OptionNameN`) must match an enum variant in `Opt`. If the identifier
///   does not match any of the `Opt` variants, a compilation error will occur.
///
/// - The value (`valueN`) should be of a type that is acceptable for the corresponding option.
///   If the value type does not match the expected type for the option, a compilation error will occur.
///
#[macro_export]
macro_rules! options {
    ( $( $opt_name:ident : $opt_value:expr ),* ) => {
        {
            let mut _opts = $crate::options::Options::builder();
            $(
                _opts.add_option($crate::options::Opt::$opt_name($opt_value.into()));
            )*
            _opts.build()
        }
    };
}

/// `OptionsBuilder` is a helper structure used to construct `Options` in a flexible way.
///
/// `OptionsBuilder` follows the builder pattern, providing a fluent interface to add options
/// and finally, build an `Options` instance. This pattern is used to handle cases where the `Options`
/// instance may require complex configuration or optional fields.
///
///
/// # Example
///
/// ```rust
/// # use llm_chain::options::*;
/// let mut builder = OptionsBuilder::new();
/// builder.add_option(Opt::Model(ModelRef::from_path("path_to_model")));
/// let options = builder.build();
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OptionsBuilder {
    /// A Vec<Opt> field that holds the options to be added to the `Options` instance.
    opts: Vec<Opt>,
}

impl OptionsBuilder {
    /// Constructs a new, empty `OptionsBuilder`.
    ///
    /// Returns an `OptionsBuilder` instance with an empty `opts` field.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use llm_chain::options::*;
    /// let builder = OptionsBuilder::new();
    /// ```
    pub fn new() -> Self {
        OptionsBuilder { opts: Vec::new() }
    }

    /// Adds an option to the `OptionsBuilder`.
    ///
    /// This function takes an `Opt` instance and pushes it to the `opts` field.
    ///
    /// # Arguments
    ///
    /// * `opt` - An `Opt` instance to be added to the `OptionsBuilder`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use llm_chain::options::*;
    /// let mut builder = OptionsBuilder::new();
    /// builder.add_option(Opt::Model(ModelRef::from_path("path_to_model")));
    /// ```
    pub fn add_option(&mut self, opt: Opt) {
        self.opts.push(opt);
    }

    /// Consumes the `OptionsBuilder`, returning an `Options` instance.
    ///
    /// This function consumes the `OptionsBuilder`, moving its `opts` field to a new `Options` instance.
    ///
    /// # Returns
    ///
    /// An `Options` instance with the options added through the builder.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use llm_chain::options::*;
    /// let mut builder = OptionsBuilder::new();
    /// builder.add_option(Opt::Model(ModelRef::from_path("path_to_model")));
    /// let options = builder.build();
    /// ```
    pub fn build(self) -> Options {
        Options { opts: self.opts }
    }
}

/// A cascade of option sets.
///
/// Options added later in the cascade override earlier options.
pub struct OptionsCascade<'a> {
    /// The sets of options, in the order they were added.
    cascades: Vec<&'a Options>,
}

impl<'a> OptionsCascade<'a> {
    /// Creates a new, empty cascade of options.
    pub fn new() -> Self {
        OptionsCascade::from_vec(Vec::new())
    }

    /// Setups a typical options cascade, with model_defaults, environment defaults a model config and possibly a specific config.
    pub fn new_typical(
        model_default: &'a Options,
        env_defaults: &'a Options,
        model_config: &'a Options,
        specific_config: Option<&'a Options>,
    ) -> Self {
        let mut v = vec![model_default, env_defaults, model_config];
        if let Some(specific_config) = specific_config {
            v.push(specific_config);
        }
        Self::from_vec(v)
    }

    /// Creates a new cascade of options from a vector of option sets.
    pub fn from_vec(cascades: Vec<&'a Options>) -> Self {
        OptionsCascade { cascades }
    }

    /// Returns a new cascade of options with the given set of options added.
    pub fn with_options(mut self, options: &'a Options) -> Self {
        self.cascades.push(options);
        self
    }

    /// Gets the value of an option from this cascade.
    ///
    /// Returns `None` if the option is not present in any set in this cascade.
    /// If the option is present in multiple sets, the value from the most
    /// recently added set is returned.
    pub fn get(&self, opt_discriminant: OptDiscriminants) -> Option<&Opt> {
        for options in self.cascades.iter().rev() {
            if let Some(opt) = options.get(opt_discriminant) {
                return Some(opt);
            }
        }
        None
    }

    /// Returns a boolean indicating if options indicate that requests should be streamed or not.
    pub fn is_streaming(&self) -> bool {
        let Some(Opt::Stream(val)) = self.get(OptDiscriminants::Stream) else {
            return false;
        };
        *val
    }
}

impl<'a> Default for OptionsCascade<'a> {
    /// Returns a new, empty cascade of options.
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// A reference to a model name or path
/// Useful for
pub struct ModelRef(String);

impl ModelRef {
    pub fn from_path<S: Into<String>>(p: S) -> Self {
        Self(p.into())
    }
    pub fn from_model_name<S: Into<String>>(model_name: S) -> Self {
        Self(model_name.into())
    }
    /// Returns the path for this model reference
    pub fn to_path(&self) -> String {
        self.0.clone()
    }
    /// Returns the name of the model
    pub fn to_name(&self) -> String {
        self.0.clone()
    }
}

/// A list of tokens to bias during the process of inferencing.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenBias(Vec<(Token, f32)>); // TODO: Serialize to a JSON object of str(F32) =>

impl TokenBias {
    /// Returns the token bias as a hashmap where the keys are i32 and the value f32. If the type doesn't match returns None
    pub fn as_i32_f32_hashmap(&self) -> Option<HashMap<i32, f32>> {
        let mut map = HashMap::new();
        for (token, value) in &self.0 {
            map.insert(token.to_i32()?, *value);
        }
        Some(map)
    }
}

#[derive(EnumDiscriminants, Clone, Debug, Serialize, Deserialize)]
pub enum Opt {
    /// The name or path of the model used.
    Model(ModelRef),
    /// The API key for the model service.
    ApiKey(String),
    /// The number of threads to use for parallel processing.
    /// This is common to all models.
    NThreads(usize),
    /// The maximum number of tokens that the model will generate.
    /// This is common to all models.
    MaxTokens(usize),
    /// The maximum context size of the model.
    MaxContextSize(usize),
    /// The sequences that, when encountered, will cause the model to stop generating further tokens.
    /// OpenAI models allow up to four stop sequences.
    StopSequence(Vec<String>),
    /// Whether or not to use streaming mode.
    /// This is common to all models.
    Stream(bool),

    /// The penalty to apply for using frequent tokens.
    /// This is used by OpenAI and llama models.
    FrequencyPenalty(f32),
    /// The penalty to apply for using novel tokens.
    /// This is used by OpenAI and llama models.
    PresencePenalty(f32),

    /// A bias to apply to certain tokens during the inference process.
    /// This is known as logit bias in OpenAI and is also used in llm-chain-local.
    TokenBias(TokenBias),

    /// The maximum number of tokens to consider for each step of generation.
    /// This is common to all models, but is not used by OpenAI.
    TopK(i32),
    /// The cumulative probability threshold for token selection.
    /// This is common to all models.
    TopP(f32),
    /// The temperature to use for token selection. Higher values result in more random output.
    /// This is common to all models.
    Temperature(f32),
    /// The penalty to apply for repeated tokens.
    /// This is common to all models.
    RepeatPenalty(f32),
    /// The number of most recent tokens to consider when applying the repeat penalty.
    /// This is common to all models.
    RepeatPenaltyLastN(usize),

    /// The TfsZ parameter for llm-chain-llama.
    TfsZ(f32),
    /// The TypicalP parameter for llm-chain-llama.
    TypicalP(f32),
    /// The Mirostat parameter for llm-chain-llama.
    Mirostat(i32),
    /// The MirostatTau parameter for llm-chain-llama.
    MirostatTau(f32),
    /// The MirostatEta parameter for llm-chain-llama.
    MirostatEta(f32),
    /// Whether or not to penalize newline characters for llm-chain-llama.
    PenalizeNl(bool),

    /// The batch size for llm-chain-local.
    NBatch(usize),
    /// The username for llm-chain-openai.
    User(String),
    /// The type of the model.
    ModelType(String),
}

// Helper function to extract environment variables
fn option_from_env<K, F>(opts: &mut OptionsBuilder, key: K, f: F) -> Result<(), VarError>
where
    K: AsRef<OsStr>,
    F: FnOnce(String) -> Option<Opt>,
{
    match std::env::var(key) {
        Ok(v) => {
            if let Some(x) = f(v) {
                opts.add_option(x);
            }
            Ok(())
        }
        Err(VarError::NotPresent) => Ok(()),
        Err(e) => Err(e),
    }
}

// Conversion functions for each Opt variant
fn model_from_string(s: String) -> Option<Opt> {
    Some(Opt::Model(ModelRef::from_path(s)))
}

fn api_key_from_string(s: String) -> Option<Opt> {
    Some(Opt::ApiKey(s))
}

macro_rules! opt_parse_str {
    ($v:ident) => {
        paste! {
            fn [< $v:snake:lower _from_string >] (s: String) -> Option<Opt> {
                        Some(Opt::$v(s.parse().ok()?))
            }
        }
    };
}

opt_parse_str!(NThreads);
opt_parse_str!(MaxTokens);
opt_parse_str!(MaxContextSize);
// Skip stop sequence?
// Skip stream?

opt_parse_str!(FrequencyPenalty);
opt_parse_str!(PresencePenalty);
// Skip TokenBias for now
opt_parse_str!(TopK);
opt_parse_str!(TopP);
opt_parse_str!(Temperature);
opt_parse_str!(RepeatPenalty);
opt_parse_str!(RepeatPenaltyLastN);
opt_parse_str!(TfsZ);
opt_parse_str!(PenalizeNl);
opt_parse_str!(NBatch);

macro_rules! opt_from_env {
    ($opt:ident, $v:ident) => {
        paste! {
            option_from_env(&mut $opt, stringify!([<
                LLM_CHAIN_ $v:snake:upper
                >]), [< $v:snake:lower _from_string >])?;
        }
    };
}

macro_rules! opts_from_env {
    ($opt:ident, $($v:ident),*) => {
        $(
            opt_from_env!($opt, $v);
        )*
    };
}

/// Loads options from environment variables.
/// Every option that can be easily understood from a string is avaliable the name
/// of the option will be in upper snake case, that means that the option `Opt::ApiKey` has the environment variable
/// `LLM_CHAIN_API_KEY`
pub fn options_from_env() -> Result<Options, VarError> {
    let mut opts = OptionsBuilder::new();
    opts_from_env!(
        opts,
        Model,
        ApiKey,
        NThreads,
        MaxTokens,
        MaxContextSize,
        FrequencyPenalty,
        PresencePenalty,
        TopK,
        TopP,
        Temperature,
        RepeatPenalty,
        RepeatPenaltyLastN,
        TfsZ,
        PenalizeNl,
        NBatch
    );
    Ok(opts.build())
}

#[cfg(test)]
mod tests {
    use super::*;
    // Tests for FromStr
    #[test]
    fn test_options_from_env() {
        use std::env;
        let orig_model = "/123/123.bin";
        let orig_nbatch = 1_usize;
        let orig_api_key = "!asd";
        env::set_var("LLM_CHAIN_MODEL", orig_model);
        env::set_var("LLM_CHAIN_N_BATCH", orig_nbatch.to_string());
        env::set_var("LLM_CHAIN_API_KEY", orig_api_key);
        let opts = options_from_env().unwrap();
        let model_path = opts
            .get(OptDiscriminants::Model)
            .and_then(|x| match x {
                Opt::Model(m) => Some(m),
                _ => None,
            })
            .unwrap();
        let nbatch = opts
            .get(OptDiscriminants::NBatch)
            .and_then(|x| match x {
                Opt::NBatch(m) => Some(m),
                _ => None,
            })
            .unwrap();
        let api_key = opts
            .get(OptDiscriminants::ApiKey)
            .and_then(|x| match x {
                Opt::ApiKey(m) => Some(m),
                _ => None,
            })
            .unwrap();
        assert_eq!(model_path.to_path(), orig_model);
        assert_eq!(nbatch.clone(), orig_nbatch);
        assert_eq!(api_key, orig_api_key);
    }
}
