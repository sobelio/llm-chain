use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestFunctionMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionResponseStream, CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    CreateChatCompletionResponse, Role,
};
use futures::StreamExt;
use llm_chain::prompt::{self, Prompt};
use llm_chain::{
    output::{Output, StreamSegment},
    prompt::{ChatMessage, ChatMessageCollection},
};

use super::error::OpenAIInnerError;

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
        Role::Tool => prompt::ChatRole::Other("Tool".to_string()),
        Role::Function => prompt::ChatRole::Other("Function".to_string()),
    }
}

fn format_chat_message(
    message: &prompt::ChatMessage<String>,
) -> Result<ChatCompletionRequestMessage, OpenAIInnerError> {
    let role = convert_role(message.role());
    let content = message.body().to_string();
    let msg = match role {
        Role::Assistant => ChatCompletionRequestMessage::Assistant(
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(content)
                .build()?,
        ),
        Role::System => ChatCompletionRequestMessage::System(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(content)
                .build()?,
        ),
        Role::User => ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessageArgs::default()
                .content(content)
                .build()?,
        ),
        Role::Tool => ChatCompletionRequestMessage::Tool(
            ChatCompletionRequestToolMessageArgs::default()
                .content(content)
                .build()?,
        ),
        Role::Function => ChatCompletionRequestMessage::Function(
            ChatCompletionRequestFunctionMessageArgs::default()
                .content(content)
                .build()?,
        ),
    };
    Ok(msg)
}

pub fn format_chat_messages(
    messages: prompt::ChatMessageCollection<String>,
) -> Result<Vec<async_openai::types::ChatCompletionRequestMessage>, OpenAIInnerError> {
    messages.iter().map(format_chat_message).collect()
}

pub fn create_chat_completion_request(
    model: String,
    prompt: &Prompt,
    is_streaming: bool,
) -> Result<CreateChatCompletionRequest, OpenAIInnerError> {
    let messages = format_chat_messages(prompt.to_chat())?;
    Ok(CreateChatCompletionRequestArgs::default()
        .model(model)
        .stream(is_streaming)
        .messages(messages)
        .build()?)
}

pub fn completion_to_output(resp: CreateChatCompletionResponse) -> Output {
    let msg = resp.choices.first().unwrap().message.clone();
    let mut col = ChatMessageCollection::new();
    col.add_message(ChatMessage::new(
        convert_openai_role(&msg.role),
        msg.content.unwrap_or_default(), // "" for missing
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
