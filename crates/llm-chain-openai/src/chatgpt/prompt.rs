use async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequest, Role};
use llm_chain::{
    prompt::StringTemplateError,
    prompt::{self, Prompt},
};

use super::Model;

fn convert_role(role: &prompt::ChatRole) -> Role {
    match role {
        prompt::ChatRole::User => Role::User,
        prompt::ChatRole::Assistant => Role::Assistant,
        prompt::ChatRole::System => Role::System,
        prompt::ChatRole::Other(_s) => Role::User, // other roles are not supported by OpenAI
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
    model: &Model,
    prompt: &Prompt,
    is_streaming: Option<bool>,
) -> Result<CreateChatCompletionRequest, StringTemplateError> {
    let messages = format_chat_messages(prompt.to_chat())?;
    Ok(CreateChatCompletionRequest {
        model: model.to_string(),
        messages,
        temperature: None,
        top_p: None,
        n: Some(1),
        stream: is_streaming,
        stop: None,
        max_tokens: None, // We should consider something here
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    })
}
