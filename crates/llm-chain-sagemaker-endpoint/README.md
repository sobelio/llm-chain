# llm-chain-mock

Amazon SageMaker Endppoint driver. Allows you to invoke a model hosted on Amazon SageMaker Endpoint, this includes Amazon SageMaker Jumpstart models.

# Getting Started

1. Follow the SageMaker JumpStart documentation to [find an LLM](https://docs.aws.amazon.com/sagemaker/latest/dg/jumpstart-foundation-models-use.html), then [deploy it](https://docs.aws.amazon.com/sagemaker/latest/dg/jumpstart-deploy.html).
2. Note down the SageMaker Endpoint name created by SageMaker JumpStart.
3. Some models is included in this crate, see `model::Model::<model_name>`. Select one in your executor options's `Model` field. See `examples/simple.rs` for example.
3. For custom models or models not included in `model::Model`, use `model::Model::Other(<model_name>)`, where `model_name` is the SageMaker endpoint name.