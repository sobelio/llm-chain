//! # Envelope Serialization
//!
//! This module contains the Envelope struct and its related functionality. It allows you to serialize and deserialize LLM chains and other data structures into different formats, such as YAML. The Envelope struct wraps your data and includes additional metadata, which can be useful for managing and organizing your serialized data.
//!
//! This module is mostly intended for internal use, but you can also use it to serialize your own data structures.
//!
//! ## Usage
//!
//! First, you need to implement the StorableEntity trait for your custom data type, which requires the get_metadata() method. Then, you can use the StorableEntityExt trait to easily read and write your data to and from files.
//!
//! ## Example
//!
//!```rust
//! use serde::{Deserialize, Serialize};
//! use llm_chain::serialization::{Envelope, StorableEntity};
//!
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! struct MyData {
//!    value: i32,
//! }
//!
//! impl StorableEntity for MyData {
//!    fn get_metadata() -> Vec<(String, String)> {
//!        vec![("author".to_string(), "John Doe".to_string())]
//!    }
//! }
//!
//!
//! let data = MyData { value: 42 };
//!
//! // Convert the data into an envelope
//! let envelope = data.clone().write_file_sync("mydata.yaml").unwrap();
//! // Serialize the envelope to a YAML file
//! let path = "mydata.yaml";
//! // Deserialize the envelope from a YAML file
//! let read_data = MyData::read_file_sync(path).unwrap();
//! assert_eq!(data.value, read_data.value);
//!
//! ```
//! ## Features
//!
//! This module provides synchronous and asynchronous methods for reading and writing envelopes to and from files. The asynchronous methods are available behind the async feature flag.
//!
//! ## Errors
//!
//! The module also provides the EnvelopeError enum, which represents errors that can occur during serialization, deserialization, and file I/O operations.
use downcast_rs::{Downcast};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
#[derive(Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub metadata: HashMap<String, String>,
    pub data: Box<dyn StorableEntity>,
}

impl Envelope {
    pub fn new<T: StorableEntity>(data: T) -> Self {
        Envelope {
            metadata: HashMap::new(),
            data: Box::new(data),
        }
    }
}

#[derive(Error, Debug)]
pub enum EnvelopeError {
    // YAML parsing
    #[error("YAML parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

impl Envelope {
    pub fn read_file_sync(path: &str) -> Result<Self, EnvelopeError> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let envelope = serde_json::from_reader(reader)?;
        Ok(envelope)
    }
    #[cfg(feature = "async")]
    pub async fn read_file_async(path: &str) -> Result<Self, EnvelopeError> {
        use tokio::io::AsyncReadExt;
        let mut file = tokio::fs::File::open(path).await?;
        let mut contents: Vec<u8> = vec![];
        file.read_to_end(&mut contents).await?;
        let envelope = serde_json::from_slice(&contents)?;
        Ok(envelope)
    }
    pub fn write_file_sync(&self, path: &str) -> Result<(), EnvelopeError> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }
    #[cfg(feature = "async")]
    pub async fn write_file_async(&self, path: &str) -> Result<(), EnvelopeError> {
        use tokio::io::AsyncWriteExt;
        let data = serde_json::to_string(&self)?;
        let mut file = tokio::fs::File::create(path).await?;
        file.write_all(data.as_bytes()).await?;
        Ok(())
    }
}

/// An entity that can be stored in an envelope.

#[typetag::serde]
pub trait StorableEntity: Downcast + StorableEntityClone {
    fn get_metadata() -> Vec<(String, String)>
    where
        Self: Sized;
    fn to_envelope(self) -> Envelope
    where
        Self: Sized,
    {
        let mut envelope = Envelope {
            metadata: HashMap::new(),
            data: Box::new(self),
        };
        for (key, value) in Self::get_metadata() {
            envelope.metadata.insert(key, value);
        }
        envelope
    }
}

#[doc(hidden)]
pub trait StorableEntityClone {
    fn clone_box(&self) -> Box<dyn StorableEntity>;
}

impl<T> StorableEntityClone for T
where
    T: 'static + StorableEntity + Clone,
{
    fn clone_box(&self) -> Box<dyn StorableEntity> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn StorableEntity> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
