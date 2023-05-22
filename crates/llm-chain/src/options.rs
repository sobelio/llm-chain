use lazy_static::lazy_static;
use std::{env::VarError, ffi::OsStr};

use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumIter, EnumString};

use crate::tokens::Token;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    opts: Vec<Opt>,
}

#[derive(thiserror::Error, Debug)]
/// Errror indicating that an option is not set.
#[error("Options not set")]
struct OptionNotSetError;

lazy_static! {
    pub static ref EMPTY_OPTIONS: Options = Options::new();
}

impl Options {
    pub fn new() -> Self {
        Options { opts: Vec::new() }
    }

    pub fn with_option(mut self, opt: Opt) -> Self {
        self.add(opt);
        self
    }

    pub fn add(&mut self, opt: Opt) {
        self.opts.push(opt)
    }

    pub fn get(&self, opt_discriminant: OptDiscriminants) -> Option<&Opt> {
        self.opts
            .iter()
            .find(|opt| OptDiscriminants::from(*opt) == opt_discriminant)
    }
}

pub struct OptionsCascade<'a> {
    cascades: Vec<&'a Options>,
}

impl<'a> OptionsCascade<'a> {
    pub fn new() -> Self {
        OptionsCascade {
            cascades: Vec::new(),
        }
    }

    pub fn with_options(mut self, options: &'a Options) -> Self {
        self.cascades.push(options);
        self
    }

    pub fn cascades(&self) -> &[&'a Options] {
        &self.cascades
    }
    pub fn get(&self, opt_discriminant: OptDiscriminants) -> Option<&Opt> {
        for options in self.cascades.iter().rev() {
            if let Some(opt) = options.get(opt_discriminant) {
                return Some(opt);
            }
        }
        None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelRef(String);

impl ModelRef {
    fn from_path<S: Into<String>>(p: S) -> Self {
        Self(p.into())
    }
    /// Returns the path for this model reference
    pub fn to_path(&self) -> String {
        self.0.clone()
    }
}

/// A list of tokens to bias during the process of inferencing.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenBias(Vec<(Token, f32)>); // TODO: Serialize to a JSON object of str(F32) =>

#[derive(EnumDiscriminants, Clone, Debug, Serialize, Deserialize)]
pub enum Opt {
    Model(ModelRef),
    ApiKey(String),
    /// Common for all models
    NThreads(usize),
    /// Common for all models
    MaxTokens(usize),
    /// COmmon for all models openai allows up to four stopseqs.
    StopSequence(Vec<String>),
    /// Common for all
    Stream(bool),

    /// For OpenAI and llama
    FrequencyPenalty(f32),
    /// For OpenAI and llama
    PresencePenalty(f32),

    /// For OpenAI and llm-chain-local, Logit-bias in openai
    TokenBias(TokenBias),

    /// Common for all models, not in OpenAI
    TopK(i32),
    /// Common for all models
    TopP(f32),
    /// Common for all models
    Temperature(f32),
    /// Common for all models
    RepeatPenalty(f32),
    /// Common for all models
    RepeatPenaltyLastN(usize),

    /// For llm-chain-llama
    TfsZ(f32),
    /// For llm-chain-llama
    TypicalP(f32),
    /// For llm-chain-llama
    Mirostat(i32),
    /// For llm-chain-llama
    MirostatTau(f32),
    /// For llm-chain-llama
    MirostatEta(f32),
    /// For llm-chain-llama
    PenalizeNl(bool),

    /// For llm-chain-local
    NBatch(usize),
    /// For llm-chain-local
    BiasTokens(String),
    /// For llm-chain-local

    /// For llm-chain-openai
    N(u8),
    /// For llm-chain-openai
    User(String),
    ModelType(String),
}

fn option_from_env<K, F>(opts: &mut Options, key: K, f: F) -> Result<(), VarError>
where
    K: AsRef<OsStr>,
    F: FnOnce(String) -> Option<Opt>,
{
    match std::env::var(key) {
        Ok(v) => {
            if let Some(x) = f(v) {
                opts.add(x)
            }
            Ok(())
        }
        Err(VarError::NotPresent) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn options_from_env() -> Result<Options, VarError> {
    let mut opts = Options::new();

    if let Ok(x) = std::env::var("LLM_CHAIN_MODEL_PATH") {
        opts.add(Opt::Model(ModelRef::from_path(x)));
    }
    option_from_env(&mut opts, "LLM_CHAIN_MODEL_PATH", |s| {
        Some(Opt::Model(ModelRef::from_path(s)))
    })?;
    option_from_env(&mut opts, "LLM_CHAIN_MODEL_TYPE", |s| {
        Some(Opt::ModelType(s))
    })?;

    Ok(opts)
}
