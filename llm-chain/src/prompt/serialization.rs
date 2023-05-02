//! Serialization of prompts and prompt template
use serde::{de::DeserializeOwned, Serialize};

use crate::serialization::StorableEntity;

use super::Data;

impl<T> StorableEntity for Data<T>
where
    T: Serialize + DeserializeOwned,
{
    fn get_metadata() -> Vec<(String, String)> {
        vec![]
    }
}
