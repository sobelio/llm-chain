use crate::{
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

    pub async fn run<L: Executor<Step = S>>(
        &self,
        parameters: Parameters,
        executor: L,
    ) -> Option<<L as Executor>::Output> {
        let mut current_params = parameters;
        let mut output: Option<L::Output> = None;
        for step in self.steps.iter() {
            let formatted = step.format(&current_params);
            let res = executor.execute(formatted).await;
            current_params = L::apply_output_to_parameters(current_params, &res);
            output = Some(res);
        }
        output
    }
}
