# llm-chain-sagemaker-endpoint

Amazon SageMaker Endppoint driver. Allows you to invoke a model hosted on Amazon SageMaker Endpoint, this includes Amazon SageMaker Jumpstart models.

# Getting Started
1. This crate uses the AWS SDK for Rust to communicate with Amazon SageMaker. You need to set up the credentials and a region following [this guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html)
1. Follow the SageMaker JumpStart documentation to [find an LLM](https://docs.aws.amazon.com/sagemaker/latest/dg/jumpstart-foundation-models-use.html), then [deploy it](https://docs.aws.amazon.com/sagemaker/latest/dg/jumpstart-deploy.html).
1. Note down the SageMaker Endpoint name created by SageMaker JumpStart.
1. Some models is included in this crate, see `model::Model::<model_name>`. Select one in your executor options's `Model` field. See `examples/simple.rs` for example.
1. For custom models or models not included in `model::Model`, use `model::Model::Other(<model_name>)`, where `model_name` is the SageMaker endpoint name.
