use async_trait::async_trait;
use erniebot_rs::chat::{ChatEndpoint, ChatOpt};
use llm_chain::options::{Opt, Options, OptionsCascade};
use llm_chain::output::{Output, StreamSegment};
use llm_chain::prompt::Prompt;
use llm_chain::tokens::{
    PromptTokensError, TokenCount, Tokenizer as TokenizerTrait, TokenizerError,
};
use llm_chain::traits::{Executor as ExecutorTrait, ExecutorCreationError, ExecutorError};
use tokio;
use tokio_stream::StreamExt;

use crate::model::Model;
use crate::prompt::create_message;

#[derive(Clone)]
pub struct Executor {
    options: Options,
}

impl Executor {
    fn get_model_from_invocation_options(&self, opts: &OptionsCascade) -> String {
        let Some(Opt::Model(model)) = opts.get(llm_chain::options::OptDiscriminants::Model) else {
            return Model::ErnieBotTurbo.to_string();
        };
        model.to_name()
    }

    fn cascade<'a>(&'a self, opts: Option<&'a Options>) -> OptionsCascade<'a> {
        let mut v: Vec<&'a Options> = vec![&self.options];
        if let Some(o) = opts {
            v.push(o);
        }
        OptionsCascade::from_vec(v)
    }
    /// transform the options into a vector of ChatOpts, to be used in the chat endpoint
    fn option_transform(&self, opts: &OptionsCascade) -> Vec<ChatOpt> {
        let mut chat_opts = Vec::new();
        // Below code is so weird. Is there a method to enumerate options?
        if let Some(Opt::Temperature(temp)) =
            opts.get(llm_chain::options::OptDiscriminants::Temperature)
        {
            chat_opts.push(ChatOpt::Temperature(*temp));
        }
        if let Some(Opt::TopK(top_k)) = opts.get(llm_chain::options::OptDiscriminants::TopK) {
            chat_opts.push(ChatOpt::TopK(*top_k as u32));
        }
        if let Some(Opt::TopP(top_p)) = opts.get(llm_chain::options::OptDiscriminants::TopP) {
            chat_opts.push(ChatOpt::TopP(*top_p));
        }
        if let Some(Opt::RepeatPenalty(repeat_penalty)) =
            opts.get(llm_chain::options::OptDiscriminants::RepeatPenalty)
        {
            chat_opts.push(ChatOpt::PenaltyScore(*repeat_penalty));
        }
        if let Some(Opt::StopSequence(stop_sequence)) =
            opts.get(llm_chain::options::OptDiscriminants::StopSequence)
        {
            chat_opts.push(ChatOpt::Stop(stop_sequence.clone()));
        }
        if let Some(Opt::MaxTokens(max_tokens)) =
            opts.get(llm_chain::options::OptDiscriminants::MaxTokens)
        {
            chat_opts.push(ChatOpt::MaxOutputTokens(*max_tokens as u32));
        }
        chat_opts
    }
}

#[async_trait]
impl ExecutorTrait for Executor {
    type StepTokenizer<'a> = ErnieTokenizer;
    fn new_with_options(options: Options) -> Result<Self, ExecutorCreationError> {
        Ok(Executor { options })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let opts = self.cascade(Some(options));
        let model = self.get_model_from_invocation_options(&opts);
        let chat_endpoint =
            if let Ok(chat_endpoint) = ChatEndpoint::new_with_custom_endpoint(&model.to_string()) {
                chat_endpoint
            } else {
                return Err(ExecutorError::InvalidOptions);
            };
        let chat_opts = self.option_transform(&opts);
        let messages = create_message(prompt);
        if opts.is_streaming() {
            let mut stream_response = chat_endpoint
                .astream(&messages, &chat_opts)
                .await
                .map_err(|e| ExecutorError::InnerError(Box::new(e)))?;
            let (sender, result_stream) = Output::new_stream();
            tokio::spawn(async move {
                while let Some(chunk) = stream_response.next().await {
                    let segment = match chunk.get_chat_result() {
                        Ok(result) => StreamSegment::Content(result),
                        Err(e) => StreamSegment::Err(ExecutorError::InnerError(Box::new(e))),
                    };
                    if sender.send(segment).is_err() {
                        break;
                    }
                }
            });
            Ok(result_stream)
        } else {
            let response = chat_endpoint
                .ainvoke(&messages, &chat_opts)
                .await
                .map_err(|e| ExecutorError::InnerError(Box::new(e)))?;
            let chat_result = response
                .get_chat_result()
                .map_err(|e| ExecutorError::InnerError(Box::new(e)))?;
            Ok(Output::new_immediate(Prompt::text(chat_result)))
        }
    }

    fn tokens_used(
        &self,
        _options: &Options,
        _prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        // Not all models expose this information.
        unimplemented!();
    }

    fn max_tokens_allowed(&self, _: &Options) -> i32 {
        // Not all models expose this information.
        unimplemented!();
    }

    fn answer_prefix(&self, _prompt: &Prompt) -> Option<String> {
        // Not all models expose this information.
        unimplemented!();
    }

    fn get_tokenizer(&self, _: &Options) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
        // Not all models expose this information.
        unimplemented!();
    }
}

pub struct ErnieTokenizer {}

impl ErnieTokenizer {
    pub fn new(_executor: &Executor) -> Self {
        ErnieTokenizer {}
    }
}

impl TokenizerTrait for ErnieTokenizer {
    fn tokenize_str(
        &self,
        _doc: &str,
    ) -> Result<llm_chain::tokens::TokenCollection, llm_chain::tokens::TokenizerError> {
        unimplemented!()
    }

    fn to_string(
        &self,
        _tokens: llm_chain::tokens::TokenCollection,
    ) -> Result<String, llm_chain::tokens::TokenizerError> {
        unimplemented!()
    }
}
