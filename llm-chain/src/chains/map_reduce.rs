use crate::{
    serialization::StorableEntity,
    traits::{Executor, Step},
    Parameters,
};
use futures::future::join_all;
#[cfg(feature = "serialization")]
use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};

pub struct Chain<S: Step> {
    map: S,
    reduce: S,
}

impl<S: Step> Chain<S> {
    pub fn new(map: S, reduce: S) -> Chain<S> {
        Chain { map, reduce }
    }
    pub async fn run<L: Executor<Step = S>>(
        &self,
        documents: Vec<Parameters>,
        base_parameters: Parameters,
        executor: L,
    ) -> Option<L::Output> {
        let mapped_documents = documents
            .iter()
            .map(|doc| base_parameters.combine(doc))
            .map(|doc| self.map.format(&doc))
            .map(|formatted| executor.execute(formatted));
        let mapped_documents = join_all(mapped_documents).await;

        let combined_output = mapped_documents
            .iter()
            .fold(None, |a, b| a.map(|a| (L::combine_outputs(&a, b))))?;

        // TODO: We need to do this recursively for really big documents

        let combined_parameters = L::apply_output_to_parameters(base_parameters, &combined_output);

        let formatted = self.reduce.format(&combined_parameters);
        let output = executor.execute(formatted).await;
        Some(output)
    }
}

#[cfg(feature = "serialization")]
impl<S: Step + Serialize> Serialize for Chain<S> {
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: serde::Serializer,
    {
        let mut strct = serializer.serialize_struct("Chain", 2)?;
        strct.serialize_field("map", &self.map)?;
        strct.serialize_field("reduce", &self.reduce)?;
        strct.end()
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

impl<S> StorableEntity for Chain<S>
where
    S: Step + StorableEntity,
{
    fn get_metadata() -> Vec<(String, String)> {
        let mut base = vec![(
            "chain-type".to_string(),
            "llm-chain::chains::map_reduce::Chain".to_string(),
        )];
        base.append(&mut S::get_metadata());
        base
    }
}
