//! Schema for Documents that can be stored in vector stores.
//!
//! This schema is used to store documents in vector stores. It is used to store the document's content and metadata.

#[derive(Debug)]
pub struct Document<M = EmptyMetadata>
where
    M: serde::Serialize + serde::de::DeserializeOwned,
{
    pub page_content: String,
    pub metadata: Option<M>,
}

impl<M> Document<M>
where
    M: serde::Serialize + serde::de::DeserializeOwned,
{
    pub fn new(page_content: String) -> Self {
        Document {
            page_content,
            metadata: None,
        }
    }
}

#[derive(Debug)]
pub struct EmptyMetadata;

impl From<()> for EmptyMetadata {
    fn from(_: ()) -> Self {
        EmptyMetadata
    }
}

// impl a serializer that turns it into a null
impl serde::Serialize for EmptyMetadata {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }
}

// impl a deserializer that turns it into a null
use serde::de::{self, Deserialize, Deserializer};

impl<'de> Deserialize<'de> for EmptyMetadata {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct EmptyMetadataVisitor;

        impl<'de> de::Visitor<'de> for EmptyMetadataVisitor {
            type Value = EmptyMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a null value")
            }

            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(EmptyMetadata)
            }
        }

        deserializer.deserialize_unit(EmptyMetadataVisitor)
    }
}
