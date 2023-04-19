use serde::de::{Deserializer, MapAccess};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};

use crate::{
    frame::Frame,
    serialization::StorableEntity,
    step::Step,
    traits::{Executor, ExecutorError},
    Parameters,
};

#[cfg(feature = "serialization")]
use crate::serialization::StorableEntity;

#[derive(thiserror::Error, Debug)]
pub enum SequentialChainError<Err: ExecutorError> {
    #[error("ExecutorError: {0}")]
    ExecutorError(#[from] Err),
    #[error("The vector of steps was empty")]
    NoSteps,
}
// A sequential chain is a chain where each step is executed in order, with the output of the previous being available to the next.
#[derive(Clone, Debug)]
pub struct Chain<E: Executor> {
    steps: Vec<Step<E>>,
}

impl<E: Executor> Chain<E> {
    pub fn new(steps: Vec<Step<E>>) -> Chain<E> {
        Chain { steps }
    }
    pub fn of_one(step: Step<E>) -> Chain<E> {
        Chain { steps: vec![step] }
    }

    pub async fn run(
        &self,
        parameters: Parameters,
        executor: &E,
    ) -> Result<E::Output, SequentialChainError<E::Error>> {
        if self.steps.is_empty() {
            return Err(SequentialChainError::NoSteps);
        }

        let mut current_params = parameters;
        let mut output: Option<E::Output> = None;
        for step in self.steps.iter() {
            let frame = Frame::new(executor, step);
            let res = frame.format_and_execute(&current_params).await?;

            current_params = current_params.with_text_from_output(&res).await;
            output = Some(res);
        }
        Ok(output.expect("No output from chain"))
    }
}

impl<E: Executor> StorableEntity for Chain<E> {
    fn get_metadata() -> Vec<(String, String)> {
        let base = vec![(
            "chain-type".to_string(),
            "llm-chain::chains::sequential::Chain".to_string(),
        )];
        base
    }
}

impl<E: Executor> Serialize for Chain<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("steps", &self.steps)?;
        map.end()
    }
}

struct ChainVisitor<E: Executor>(std::marker::PhantomData<E>);

impl<'de, E: Executor> serde::de::Visitor<'de> for ChainVisitor<E> {
    type Value = Chain<E>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map with a key named 'steps'")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut steps = None;
        while let Some(key) = map.next_key()? {
            match key {
                "steps" => {
                    if steps.is_some() {
                        return Err(serde::de::Error::duplicate_field("steps"));
                    }
                    steps = Some(map.next_value()?);
                }
                _ => return Err(serde::de::Error::unknown_field(key, &["steps"])),
            }
        }
        let steps = steps.ok_or_else(|| serde::de::Error::missing_field("steps"))?;
        Ok(Chain { steps })
    }
}

impl<'de, E: Executor> Deserialize<'de> for Chain<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ChainVisitor(std::marker::PhantomData))
    }
}
