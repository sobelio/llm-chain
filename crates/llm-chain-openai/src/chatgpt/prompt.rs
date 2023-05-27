use async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequest, Role};
use futures::StreamExt;
use llm_chain::{
    output::{Output, StreamSegment},
    prompt::{ChatMessage, ChatMessageCollection},
};
use llm_chain::{
    prompt::StringTemplateError,
    prompt::{self, Prompt},
};

use async_openai::types::{ChatCompletionResponseStream, CreateChatCompletionResponse};

fn convert_role(role: &prompt::ChatRole) -> Role {
    match role {
        prompt::ChatRole::User => Role::User,
        prompt::ChatRole::Assistant => Role::Assistant,
        prompt::ChatRole::System => Role::System,
        prompt::ChatRole::Other(_s) => Role::User, // other roles are not supported by OpenAI
    }
}

fn convert_openai_role(role: &Role) -> prompt::ChatRole {
    match role {
        Role::User => prompt::ChatRole::User,
        Role::Assistant => prompt::ChatRole::Assistant,
        Role::System => prompt::ChatRole::System,
    }
}

fn format_chat_message(
    message: &prompt::ChatMessage<String>,
) -> Result<ChatCompletionRequestMessage, StringTemplateError> {
    let role = convert_role(message.role());
    let content = message.body().to_string();
    Ok(ChatCompletionRequestMessage {
        role,
        content,
        name: None,
    })
}

pub fn format_chat_messages(
    messages: prompt::ChatMessageCollection<String>,
) -> Result<Vec<ChatCompletionRequestMessage>, StringTemplateError> {
    messages.iter().map(format_chat_message).collect()
}

pub fn create_chat_completion_request(
    model: String,
    prompt: &Prompt,
    is_streaming: bool,
) -> Result<CreateChatCompletionRequest, StringTemplateError> {
    let messages = format_chat_messages(prompt.to_chat())?;
    Ok(CreateChatCompletionRequest {
        model,
        messages,
        temperature: None,
        top_p: None,
        n: Some(1),
        stream: Some(is_streaming),
        stop: None,
        max_tokens: None, // We should consider something here
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    })
}

pub fn completion_to_output(resp: CreateChatCompletionResponse) -> Output {
    let msg = resp.choices.first().unwrap().message.clone();
    let mut col = ChatMessageCollection::new();
    col.add_message(ChatMessage::new(
        convert_openai_role(&msg.role),
        msg.content,
    ));
    Output::new_immediate(col.into())
}

pub fn stream_to_output(resp: ChatCompletionResponseStream) -> Output {
    let stream = resp.flat_map(|x| {
        // Can't unwrap here!!
        let resp = x.unwrap();

        let delta = resp.choices.first().unwrap().delta.clone();

        let mut v = vec![];

        if let Some(role) = delta.role {
            v.push(StreamSegment::Role(convert_openai_role(&role)));
        }
        if let Some(content) = delta.content {
            v.push(StreamSegment::Content(content))
        }
        futures::stream::iter(v)
    });
    Output::from_stream(stream)
}
