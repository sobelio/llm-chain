use async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequest, Role};
use llm_chain::{
    prompt::chat::{self, ChatRole},
    prompt::Prompt,
    Parameters, PromptTemplateError,
};

use super::Model;

fn convert_role(role: chat::ChatRole) -> Role {
    match role {
        ChatRole::User => Role::User,
        ChatRole::Assistant => Role::Assistant,
        ChatRole::System => Role::System,
        ChatRole::Other(_s) => Role::User, // other roles are not supported by OpenAI
    }
}

fn format_chat_message(
    message: chat::ChatMessage,
) -> Result<ChatCompletionRequestMessage, PromptTemplateError> {
    let role = convert_role(message.role());
    let content = message.content().format(&Parameters::new())?;
    Ok(ChatCompletionRequestMessage {
        role,
        content,
        name: None,
    })
}

fn format_chat_messages(
    messages: Vec<chat::ChatMessage>,
) -> Result<Vec<ChatCompletionRequestMessage>, PromptTemplateError> {
    messages.into_iter().map(format_chat_message).collect()
}

pub fn create_chat_completion_request(
    model: &Model,
    prompt: &Prompt,
    _parameters: &Parameters,
) -> Result<CreateChatCompletionRequest, PromptTemplateError> {
    let messages = format_chat_messages(prompt.as_chat_prompt())?;
    Ok(CreateChatCompletionRequest {
        model: model.to_string(),
        messages,
        temperature: None,
        top_p: None,
        n: Some(1),
        stream: None,
        stop: None,
        max_tokens: None, // We should consider something here
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    })
}
