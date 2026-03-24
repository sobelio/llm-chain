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

use super::error::MiniMaxInnerError;

fn convert_role(role: &prompt::ChatRole) -> Role {
    match role {
        prompt::ChatRole::User => Role::User,
        prompt::ChatRole::Assistant => Role::Assistant,
        prompt::ChatRole::System => Role::System,
        prompt::ChatRole::Other(_s) => Role::User,
    }
}

fn convert_minimax_role(role: &Role) -> prompt::ChatRole {
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
) -> Result<ChatCompletionRequestMessage, MiniMaxInnerError> {
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
) -> Result<Vec<ChatCompletionRequestMessage>, MiniMaxInnerError> {
    messages.iter().map(format_chat_message).collect()
}

pub fn create_chat_completion_request(
    model: String,
    prompt: &Prompt,
    is_streaming: bool,
) -> Result<CreateChatCompletionRequest, MiniMaxInnerError> {
    let messages = format_chat_messages(prompt.to_chat())?;
    Ok(CreateChatCompletionRequestArgs::default()
        .model(model)
        .stream(is_streaming)
        .messages(messages)
        .build()?)
}

pub fn completion_to_output(resp: CreateChatCompletionResponse) -> Output {
    let msg = resp.choices.first().unwrap().message.clone();
    let content = msg.content.unwrap_or_default();
    // Strip MiniMax thinking tags if present (M2.5+ models may include <think>...</think>)
    let content = strip_think_tags(&content);
    let mut col = ChatMessageCollection::new();
    col.add_message(ChatMessage::new(
        convert_minimax_role(&msg.role),
        content,
    ));
    Output::new_immediate(col.into())
}

pub fn stream_to_output(resp: ChatCompletionResponseStream) -> Output {
    let stream = resp.flat_map(|x| {
        let resp = x.unwrap();
        let delta = resp.choices.first().unwrap().delta.clone();
        let mut v = vec![];
        if let Some(role) = delta.role {
            v.push(StreamSegment::Role(convert_minimax_role(&role)));
        }
        if let Some(content) = delta.content {
            // Strip thinking tags from streamed content
            let content = strip_think_tags(&content);
            if !content.is_empty() {
                v.push(StreamSegment::Content(content));
            }
        }
        futures::stream::iter(v)
    });
    Output::from_stream(stream)
}

/// Strip `<think>...</think>` tags that MiniMax models may include in responses.
fn strip_think_tags(content: &str) -> String {
    if let Some(start) = content.find("<think>") {
        if let Some(end) = content.find("</think>") {
            let end = end + "</think>".len();
            let mut result = String::new();
            result.push_str(&content[..start]);
            result.push_str(&content[end..]);
            return result.trim().to_string();
        }
    }
    content.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_think_tags_no_tags() {
        assert_eq!(strip_think_tags("Hello world"), "Hello world");
    }

    #[test]
    fn test_strip_think_tags_with_tags() {
        assert_eq!(
            strip_think_tags("<think>reasoning here</think>The answer is 42"),
            "The answer is 42"
        );
    }

    #[test]
    fn test_strip_think_tags_only_tags() {
        assert_eq!(
            strip_think_tags("<think>internal reasoning</think>"),
            ""
        );
    }

    #[test]
    fn test_strip_think_tags_with_surrounding_content() {
        assert_eq!(
            strip_think_tags("Before <think>thinking</think> After"),
            "Before  After"
        );
    }
}
