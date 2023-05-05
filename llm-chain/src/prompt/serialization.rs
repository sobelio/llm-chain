//! Serialization of prompts and prompt template

use crate::serialization::StorableEntity;

use super::{Data, StringTemplate};

#[typetag::serde]
impl StorableEntity for Data<String> {
    fn get_metadata() -> Vec<(String, String)> {
        vec![]
    }
}

#[typetag::serde]
impl StorableEntity for Data<StringTemplate> {
    fn get_metadata() -> Vec<(String, String)> {
        vec![]
    }
}
