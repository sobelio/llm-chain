#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::{
    frame::Frame,
    serialization::StorableEntity,
    traits::{Executor, Step},
    Parameters,
};

// A sequential chain is a chain where each step is executed in order, with the output of the previous being available to the next.
pub struct Chain<S: Step> {
    steps: Vec<S>,
}

impl<S: Step> Chain<S> {
    pub fn new(steps: Vec<S>) -> Chain<S> {
        Chain { steps }
    }
    pub fn of_one(step: S) -> Chain<S> {
        Chain { steps: vec![step] }
    }

    pub async fn run<E: Executor<Step = S>>(
        &self,
        parameters: Parameters,
        executor: &E,
    ) -> Option<E::Output> {
        let mut current_params = parameters;
        let mut output: Option<E::Output> = None;
        for step in self.steps.iter() {
            let frame = Frame::new(executor, step);
            let res = frame.format_and_execute(&current_params).await;
            current_params = E::apply_output_to_parameters(current_params, &res);
            output = Some(res);
        }
        output
    }
}

#[cfg(feature = "serialization")]
impl<S: Step + Serialize> Serialize for Chain<S> {
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: serde::Serializer,
    {
        Serialize::serialize(&self.steps, serializer)
    }
}

#[cfg(feature = "serialization")]
impl<'de, S: Step + Deserialize<'de>> Deserialize<'de> for Chain<S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|steps| Chain { steps })
    }
}

impl<S: Step + StorableEntity> StorableEntity for Chain<S> {
    fn get_metadata() -> Vec<(String, String)> {
        let mut base = vec![(
            "chain-type".to_string(),
            "llm-chain::chains::sequential::Chain".to_string(),
        )];
        base.append(&mut S::get_metadata());
        base
    }
}
