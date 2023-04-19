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
use serde::de::{DeserializeOwned, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
#[derive(Debug, Clone)]
pub struct Envelope<T> {
    pub metadata: HashMap<String, String>,
    pub data: T,
}

impl<T: Serialize> Serialize for Envelope<T> {
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: Serializer,
    {
        let mut envelope = serializer.serialize_struct("Envelope", 2)?;
        envelope.serialize_field("metadata", &self.metadata)?;
        envelope.serialize_field("data", &self.data)?;
        envelope.end()
    }
}

impl<'de, T: Serialize + Deserialize<'de>> Deserialize<'de> for Envelope<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EnvelopeVisitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> Visitor<'de> for EnvelopeVisitor<T>
        where
            T: Serialize + Deserialize<'de>,
        {
            type Value = Envelope<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Envelope")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut metadata = None;
                let mut data: Option<T> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "metadata" => {
                            let hm = map.next_value()?;
                            metadata = Some(hm);
                        }
                        "data" => {
                            data = Some(map.next_value()?);
                        }
                        _ => (),
                    }
                }

                let metadata = metadata.unwrap_or_default();
                let data = data.ok_or_else(|| serde::de::Error::missing_field("data"))?;

                Ok(Envelope { metadata, data })
            }
        }
        deserializer.deserialize_map(EnvelopeVisitor(std::marker::PhantomData))
    }
}

impl<T> Envelope<T> {
    pub fn new(data: T) -> Self {
        Envelope {
            metadata: HashMap::new(),
            data,
        }
    }
}

#[derive(Error, Debug)]
pub enum EnvelopeError {
    // YAML parsing
    #[error("YAML parsing error: {0}")]
    YamlParsingError(#[from] serde_yaml::Error),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

impl<T> Envelope<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn read_file_sync(path: &str) -> Result<Self, EnvelopeError> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let envelope = serde_yaml::from_reader(reader)?;
        Ok(envelope)
    }
    #[cfg(feature = "async")]
    pub async fn read_file_async(path: &str) -> Result<Self, EnvelopeError> {
        use tokio::io::AsyncReadExt;
        let mut file = tokio::fs::File::open(path).await?;
        let mut contents: Vec<u8> = vec![];
        file.read_to_end(&mut contents).await?;
        let envelope = serde_yaml::from_slice(&contents)?;
        Ok(envelope)
    }
    pub fn write_file_sync(&self, path: &str) -> Result<(), EnvelopeError> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_yaml::to_writer(writer, &self)?;
        Ok(())
    }
    #[cfg(feature = "async")]
    pub async fn write_file_async(&self, path: &str) -> Result<(), EnvelopeError> {
        use tokio::io::AsyncWriteExt;
        let data = serde_yaml::to_string(&self)?;
        let mut file = tokio::fs::File::create(path).await?;
        file.write_all(data.as_bytes()).await?;
        Ok(())
    }
}

/// An entity that can be stored in an envelope.
pub trait StorableEntity: Serialize + DeserializeOwned {
    fn get_metadata() -> Vec<(String, String)>;
    fn to_envelope(self) -> Envelope<Self>
    where
        Self: Sized,
    {
        let mut envelope = Envelope {
            metadata: HashMap::new(),
            data: self,
        };
        for (key, value) in Self::get_metadata() {
            envelope.metadata.insert(key, value);
        }
        envelope
    }
    fn from_envelope(envelope: Envelope<Self>) -> Self {
        envelope.data
    }
    fn read_file_sync(path: &str) -> Result<Self, EnvelopeError> {
        Envelope::<Self>::read_file_sync(path).map(|envelope| Self::from_envelope(envelope))
    }
    fn write_file_sync(self, path: &str) -> Result<(), EnvelopeError> {
        Envelope::new(self).write_file_sync(path)
    }
}
