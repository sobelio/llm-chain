use async_openai::types::{ChatCompletionRequestMessage, Role};
use llm_chain::{prompt, Parameters, PromptTemplate, PromptTemplateError};
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
/// A message prompt template consists of a role and a content. The role is either `User`, `System`, `Assistant`, and the content is a prompt template.
#[derive(Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct MessagePromptTemplate {
    role: Role,
    content: PromptTemplate,
}

impl<T: Into<PromptTemplate>> From<(Role, T)> for MessagePromptTemplate {
    fn from((role, content): (Role, T)) -> Self {
        let content: PromptTemplate = content.into();
        Self { role, content }
    }
}

fn convert_role(role: prompt::ChatRole) -> Role {
    match role {
        prompt::ChatRole::User => Role::User,
        prompt::ChatRole::Assistant => Role::Assistant,
        prompt::ChatRole::System => Role::System,
        prompt::ChatRole::Other(_s) => Role::User, // other roles are not supported by OpenAI
    }
}

impl From<prompt::ChatMessage> for MessagePromptTemplate {
    fn from(message: prompt::ChatMessage) -> Self {
        Self {
            role: convert_role(message.role()),
            content: message.content(),
        }
    }
}

impl MessagePromptTemplate {
    pub fn new(role: async_openai::types::Role, content: PromptTemplate) -> MessagePromptTemplate {
        MessagePromptTemplate { role, content }
    }
    pub fn format(
        &self,
        parameters: &Parameters,
    ) -> Result<ChatCompletionRequestMessage, PromptTemplateError> {
        Ok(ChatCompletionRequestMessage {
            role: self.role.clone(),
            content: self.content.format(parameters)?,
            name: None,
        })
    }
}

// From any list of things that can become messages we can create prompt templates.
impl<T: Into<MessagePromptTemplate>, L: IntoIterator<Item = T>> From<L> for ChatPromptTemplate {
    fn from(messages: L) -> Self {
        Self::new(messages.into_iter().map(|message| message.into()).collect())
    }
}

/// The `ChatPromptTemplate` struct represents a conversational template for generating prompts with LLMs. It consists of a list of messages that form the structure of the conversation.
///
/// Typically, a `ChatPromptTemplate` starts with a system message to set the context, followed by user messages and potential assistant messages. This design makes it easy to create dynamic and engaging conversational prompts for LLMs like ChatGPT.
///
/// # Example
///
/// ```
/// use llm_chain_openai::chatgpt::{ChatPromptTemplate, MessagePromptTemplate};
/// use async_openai::types::Role;
///
/// let system_msg = MessagePromptTemplate::new(Role::System, "You are an assistant that speaks like Shakespeare.".into());
/// let user_msg = MessagePromptTemplate::new(Role::User, "tell me a joke".into());
///
/// let chat_template = ChatPromptTemplate::new(vec![system_msg, user_msg]);
/// ```
/// Or simply
/// ```
/// use llm_chain_openai::chatgpt::ChatPromptTemplate;
/// use async_openai::types::Role;
/// let chat_template: ChatPromptTemplate = vec![
///   (Role::System, "You are an assistant that speaks like Shakespeare."),
///   (Role::User, "tell me a joke"),   
/// ].into();
/// ```
/// And for the truly lazy
/// ```
/// use llm_chain_openai::chatgpt::{ChatPromptTemplate};
/// let chat_template = ChatPromptTemplate::system_and_user(
///   "You are an assistant that speaks like Shakespeare.",
///   "tell me a joke",
/// );
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct ChatPromptTemplate {
    messages: Vec<MessagePromptTemplate>,
}

impl ChatPromptTemplate {
    pub fn new(messages: Vec<MessagePromptTemplate>) -> ChatPromptTemplate {
        ChatPromptTemplate { messages }
    }
    pub fn for_prompt<P: prompt::Prompt>(prompt: &P) -> ChatPromptTemplate {
        ChatPromptTemplate::new(
            prompt
                .as_chat_prompt()
                .into_iter()
                .map(Into::into)
                .collect(),
        )
    }
    pub fn system_and_user<S: Into<PromptTemplate>, U: Into<PromptTemplate>>(
        system: S,
        user: U,
    ) -> ChatPromptTemplate {
        ChatPromptTemplate {
            messages: vec![
                MessagePromptTemplate::new(Role::System, system.into()),
                MessagePromptTemplate::new(Role::User, user.into()),
            ],
        }
    }
    pub fn format(
        &self,
        parameters: &Parameters,
    ) -> Result<Vec<ChatCompletionRequestMessage>, PromptTemplateError> {
        self.messages
            .iter()
            .map(|message| message.format(parameters))
            .collect()
    }

    pub fn add<T: Into<MessagePromptTemplate>>(&mut self, message: T) {
        self.messages.push(message.into());
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_chat_prompt_template() {
        use super::*;
        use async_openai::types::Role;
        let system_msg = MessagePromptTemplate::new(
            Role::System,
            "You are an assistant that speaks like Shakespeare.".into(),
        );
        let user_msg = MessagePromptTemplate::new(Role::User, "tell me a joke".into());

        let chat_template = ChatPromptTemplate::new(vec![system_msg, user_msg]);
        let messages = chat_template.format(&Parameters::new()).unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(
            messages[0].content,
            "You are an assistant that speaks like Shakespeare."
        );
        assert_eq!(messages[1].content, "tell me a joke");
    }

    #[test]
    fn test_chat_prompt_template_with_named_parameters() {
        use super::*;
        use async_openai::types::Role;
        let system_msg = MessagePromptTemplate::new(
            Role::System,
            "You are an assistant that speaks like Shakespeare.".into(),
        );
        let user_msg = MessagePromptTemplate::new(Role::User, "tell me a joke".into());
        let assistant_msg = MessagePromptTemplate::new(
            Role::User,
            PromptTemplate::static_string("here is one, I'm sure, will crack you {up}"),
        );

        let chat_template = ChatPromptTemplate::new(vec![system_msg, user_msg, assistant_msg]);
        let messages = chat_template.format(&Parameters::new()).unwrap();
        assert_eq!(messages.len(), 3);
        assert_eq!(
            messages[0].content,
            "You are an assistant that speaks like Shakespeare."
        );
        assert_eq!(messages[1].content, "tell me a joke");
        assert_eq!(
            messages[2].content,
            "here is one, I'm sure, will crack you {up}"
        );
    }
}
