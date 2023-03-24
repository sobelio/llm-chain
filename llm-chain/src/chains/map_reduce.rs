use crate::{
    traits::{Executor, Step},
    Parameters,
};
use futures::future::join_all;

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
            .fold(None, |a, b| a.map(|a| (L::combine_outputs(&a, &b))))?;

        // TODO: We need to do this recursively for really big documents

        let combined_parameters = L::apply_output_to_parameters(base_parameters, &combined_output);

        let formatted = self.reduce.format(&combined_parameters);
        let output = executor.execute(formatted).await;
        Some(output)
    }
}
