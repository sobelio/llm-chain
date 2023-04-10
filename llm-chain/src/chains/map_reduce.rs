//! The `map_reduce` module contains the `Chain` struct, which represents a map-reduce chain.
//!
//! A map-reduce chain is a combination of two steps - a `map` step and a `reduce` step.
//! The `map` step processes each input document and the `reduce` step combines the results
//! of the `map` step into a single output.
//!
//! The `Chain` struct is generic over the type of the `Step` and provides a convenient way
//! to execute map-reduce operations using a provided `Executor`.

use crate::{
    frame::Frame,
    serialization::StorableEntity,
    traits::{Executor, Step},
    Parameters,
};
use futures::future::join_all;
#[cfg(feature = "serialization")]
use serde::{
    de::{MapAccess, Visitor},
    Deserialize,
};

/// The `Chain` struct represents a map-reduce chain, consisting of a `map` step and a `reduce` step.
///
/// The struct is generic over the type of the `Step` and provides methods for constructing and
/// executing the chain using a given `Executor`.
pub struct Chain<S: Step> {
    map: S,
    reduce: S,
}

impl<S: Step> Chain<S> {
    /// Constructs a new `Chain` with the given `map` and `reduce` steps.
    ///
    /// The `new` function takes two instances of `Step` and returns a new `Chain` instance.
    pub fn new(map: S, reduce: S) -> Chain<S> {
        Chain { map, reduce }
    }

    /// Executes the map-reduce chain using the provided `Executor`.
    ///
    /// The `run` function takes a vector of input documents, a base set of parameters, and a reference
    /// to an `Executor`. It processes the input documents using the `map` step and the `reduce` step,
    /// and returns the result as an `Option<E::Output>`.
    ///
    /// The function is asynchronous and must be awaited.
    pub async fn run<E: Executor<Step = S>>(
        &self,
        documents: Vec<Parameters>,
        base_parameters: Parameters,
        executor: &E,
    ) -> Option<E::Output> {
        let map_frame = Frame::new(executor, &self.map);
        let reduce_frame = Frame::new(executor, &self.reduce);

        // Execute the `map` step for each document, combining the base parameters with each document's parameters.
        let combined_docs: Vec<_> = documents
            .iter()
            .map(|doc| base_parameters.combine(doc))
            .collect();
        let futures: Vec<_> = combined_docs
            .iter()
            .map(|doc| map_frame.format_and_execute(doc))
            .collect();
        let mapped_documents = join_all(futures).await;

        // Combine the results of the `map` step using the `reduce` step.
        let combined_output = mapped_documents
            .iter()
            .fold(None, |a, b| a.map(|a| (E::combine_outputs(&a, b))))?;

        // TODO: We need to do this recursively for really big documents

        // Apply the combined output to the base parameters.
        let combined_parameters = E::apply_output_to_parameters(base_parameters, &combined_output);

        // Execute the `reduce` step using the combined parameters.
        let output = reduce_frame.format_and_execute(&combined_parameters).await;
        Some(output)
    }
}

#[cfg(feature = "serialization")]
impl<'de, S: Step + Deserialize<'de>> Deserialize<'de> for Chain<S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ChainVisitor<S>(std::marker::PhantomData<S>);

        impl<'de, S: Step + Deserialize<'de>> Visitor<'de> for ChainVisitor<S> {
            type Value = Chain<S>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an object with fields `map` and `reduce`")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut map_value: Option<S> = None;
                let mut reduce_value: Option<S> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "map" => {
                            if map_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("map"));
                            }
                            map_value = Some(map.next_value()?);
                        }
                        "reduce" => {
                            if reduce_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("reduce"));
                            }
                            reduce_value = Some(map.next_value()?);
                        }
                        _ => (),
                    }
                }

                let map = map_value.ok_or_else(|| serde::de::Error::missing_field("map"))?;
                let reduce =
                    reduce_value.ok_or_else(|| serde::de::Error::missing_field("reduce"))?;
                Ok(Chain { map, reduce })
            }
        }

        deserializer.deserialize_struct(
            "Chain",
            &["map", "reduce"],
            ChainVisitor(std::marker::PhantomData),
        )
    }
}

/// Implements the `StorableEntity` trait for the `Chain` struct.
///
/// This implementation provides a method for extracting metadata from a `Chain` instance, in order to identify it
impl<S> StorableEntity for Chain<S>
where
    S: Step + StorableEntity,
{
    /// Returns metadata about the Chain instance.
    ///
    /// The metadata is returned as a vector of tuples, where each tuple contains a key-value pair
    /// representing a metadata field and its value.
    ///
    /// This method also extracts metadata from the Step instances associated with the Chain.
    fn get_metadata() -> Vec<(String, String)> {
        let mut base = vec![(
            "chain-type".to_string(),
            "llm-chain::chains::map_reduce::Chain".to_string(),
        )];
        base.append(&mut S::get_metadata());
        base
    }
}
