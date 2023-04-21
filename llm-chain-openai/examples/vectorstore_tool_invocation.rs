use std::sync::Arc;

use async_trait::async_trait;
use llm_chain::output::Output;
use llm_chain::prompt::chat::{ChatMessage, ChatPrompt, ChatRole};
use llm_chain::schema::Document;
use llm_chain::tools::tools::{BashTool, VectorStoreTool};
use llm_chain::tools::{Format, Tool, ToolCollection, ToolDescription, ToolError};
use llm_chain::vectorstores::qdrant::Qdrant;
use llm_chain::{multitool, PromptTemplate};
use llm_chain::{traits::StepExt, Parameters};
use llm_chain_openai::chatgpt::{Executor, Step};
use llm_chain_openai::embeddings::Embeddings;
use qdrant_client::prelude::{QdrantClient, QdrantClientConfig};
use qdrant_client::qdrant::{CreateCollection, Distance, VectorParams, VectorsConfig};
use serde::{Deserialize, Serialize};
use thiserror::Error;
// A simple example generating a prompt with some tools.

/// Your custom tool's implementation:
#[derive(Debug, Error)]
#[error("MyTool custom error")]
struct MyToolError(#[from] serde_yaml::Error);

impl ToolError for MyToolError {}

#[derive(Serialize, Deserialize)]
struct MyToolInput;
#[derive(Serialize, Deserialize)]
struct MyToolOutput;

struct MyTool {}

#[async_trait]
impl Tool for MyTool {
    type Input = MyToolInput;
    type Output = MyToolOutput;
    type Error = MyToolError;

    fn description(&self) -> ToolDescription {
        ToolDescription {
            name: "MyTool".into(),
            description: "My custom implementation of a tool".into(),
            description_context: "You are able to use my tool".into(),
            input_format: Format::new(vec![]),
            output_format: Format::new(vec![]),
        }
    }

    async fn invoke(&self, _: serde_yaml::Value) -> Result<serde_yaml::Value, Self::Error> {
        Ok(serde_yaml::Value::Null)
    }

    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
struct MyMetadata;

// `multitool!` macro cannot handle generic annotations as of now; for now you will need to pass concrete arguments and alias your types
type QdrantTool = VectorStoreTool<Embeddings, Qdrant<Embeddings, MyMetadata>>;
type QdrantToolInput = <QdrantTool as Tool>::Input;
type QdrantToolOutput = <QdrantTool as Tool>::Output;
type QdrantToolError = <QdrantTool as Tool>::Error;

#[derive(Serialize, Deserialize)]
enum MultiToolInput {
    QdrantToolInput(QdrantToolInput),
    MyToolInput(MyToolInput),
}
impl TryInto<QdrantToolInput> for MultiToolInput {
    type Error = MultitoolError;
    fn try_into(self) -> Result<QdrantToolInput, Self::Error> {
        if let MultiToolInput::QdrantToolInput(t) = self {
            Ok(t)
        } else {
            Err(MultitoolError::BadVariant)
        }
    }
}
impl TryInto<MyToolInput> for MultiToolInput {
    type Error = MultitoolError;
    fn try_into(self) -> Result<MyToolInput, Self::Error> {
        if let MultiToolInput::MyToolInput(t) = self {
            Ok(t)
        } else {
            Err(MultitoolError::BadVariant)
        }
    }
}
#[derive(Serialize, Deserialize)]
enum MultiToolOutput {
    QdrantToolOutput(QdrantToolOutput),
    MyToolOutput(MyToolOutput),
}
impl From<QdrantToolOutput> for MultiToolOutput {
    fn from(tool: QdrantToolOutput) -> Self {
        MultiToolOutput::QdrantToolOutput(tool)
    }
}
impl From<MyToolOutput> for MultiToolOutput {
    fn from(tool: MyToolOutput) -> Self {
        MultiToolOutput::MyToolOutput(tool)
    }
}
impl TryInto<QdrantToolOutput> for MultiToolOutput {
    type Error = MultitoolError;
    fn try_into(self) -> Result<QdrantToolOutput, Self::Error> {
        if let MultiToolOutput::QdrantToolOutput(t) = self {
            Ok(t)
        } else {
            Err(MultitoolError::BadVariant)
        }
    }
}
impl TryInto<MyToolOutput> for MultiToolOutput {
    type Error = MultitoolError;
    fn try_into(self) -> Result<MyToolOutput, Self::Error> {
        if let MultiToolOutput::MyToolOutput(t) = self {
            Ok(t)
        } else {
            Err(MultitoolError::BadVariant)
        }
    }
}
#[derive(Debug, Error)]
enum MultitoolError {
    #[error("Could not convert")]
    BadVariant,
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    QdrantToolError(#[from] QdrantToolError),
    #[error(transparent)]
    MyToolError(#[from] MyToolError),
}
impl ToolError for MultitoolError {}

enum Multitool {
    QdrantTool(QdrantTool),
    MyTool(MyTool),
}
impl From<QdrantTool> for Multitool {
    fn from(tool: QdrantTool) -> Self {
        Multitool::QdrantTool(tool)
    }
}
impl From<MyTool> for Multitool {
    fn from(tool: MyTool) -> Self {
        Multitool::MyTool(tool)
    }
}
#[async_trait]
impl Tool for Multitool {
    type Input = MultiToolInput;
    type Output = MultiToolOutput;
    type Error = MultitoolError;
    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        match (self, input) {
            (Multitool::QdrantTool(t), MultiToolInput::QdrantToolInput(i)) => t
                .invoke_typed(i)
                .await
                .map(|o| <QdrantToolOutput as Into<Self::Output>>::into(o))
                .map_err(|e| e.into()),
            (Multitool::MyTool(t), MultiToolInput::MyToolInput(i)) => t
                .invoke_typed(i)
                .await
                .map(|o| <MyToolOutput as Into<Self::Output>>::into(o))
                .map_err(|e| e.into()),
            _ => Err(MultitoolError::BadVariant),
        }
    }
    #[doc = " Returns the `ToolDescription` containing metadata about the tool."]
    fn description(&self) -> ToolDescription {
        match self {
            Multitool::QdrantTool(t) => t.description(),
            Multitool::MyTool(t) => t.description(),
        }
    }
    #[doc = " Invokes the tool with the given YAML-formatted input."]
    #[doc = ""]
    #[doc = " # Errors"]
    #[doc = ""]
    #[doc = " Returns an `ToolUseError` if the input is not in the expected format or if the tool"]
    #[doc = " fails to produce a valid output."]
    async fn invoke(&self, input: serde_yaml::Value) -> Result<serde_yaml::Value, Self::Error> {
        match self {
            Multitool::QdrantTool(t) => t.invoke(input).await.map_err(|e| e.into()),
            Multitool::MyTool(t) => t.invoke(input).await.map_err(|e| e.into()),
        }
    }
    #[doc = " Checks whether the tool matches the given name."]
    #[doc = ""]
    #[doc = " This function is used to find the appropriate tool in a `ToolCollection` based on its name."]
    fn matches(&self, name: &str) -> bool {
        match self {
            Multitool::QdrantTool(t) => t.description().name == name,
            Multitool::MyTool(t) => t.description().name == name,
        }
    }
}

async fn build_local_qdrant() -> Qdrant<Embeddings> {
    // Qdrant prep
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = Arc::new(QdrantClient::new(Some(config)).await.unwrap());
    let collection_name = "my-collection".to_string();
    let embedding_size = 1536;

    if !client
        .has_collection(collection_name.clone())
        .await
        .unwrap()
    {
        client
            .create_collection(&CreateCollection {
                collection_name: collection_name.clone(),
                vectors_config: Some(VectorsConfig {
                    config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                        VectorParams {
                            size: embedding_size,
                            distance: Distance::Cosine.into(),
                            hnsw_config: None,
                            quantization_config: None,
                        },
                    )),
                }),
                ..Default::default()
            })
            .await
            .unwrap();
    }

    let embeddings = llm_chain_openai::embeddings::Embeddings::default();

    let qdrant = Qdrant::new(client, collection_name, embeddings, None, None);

    let doc_dog_definition = r#"The dog (Canis familiaris[4][5] or Canis lupus familiaris[5]) is a domesticated descendant of the wolf. Also called the domestic dog, it is derived from the extinct Pleistocene wolf,[6][7] and the modern wolf is the dog's nearest living relative.[8] Dogs were the first species to be domesticated[9][8] by hunter-gatherers over 15,000 years ago[7] before the development of agriculture.[1] Due to their long association with humans, dogs have expanded to a large number of domestic individuals[10] and gained the ability to thrive on a starch-rich diet that would be inadequate for other canids.[11]
    
        The dog has been selectively bred over millennia for various behaviors, sensory capabilities, and physical attributes.[12] Dog breeds vary widely in shape, size, and color. They perform many roles for humans, such as hunting, herding, pulling loads, protection, assisting police and the military, companionship, therapy, and aiding disabled people. Over the millennia, dogs became uniquely adapted to human behavior, and the human–canine bond has been a topic of frequent study.[13] This influence on human society has given them the sobriquet of "man's best friend"."#.to_string();

    let doc_woodstock_sound = r#"Sound for the concert was engineered by sound engineer Bill Hanley. "It worked very well", he says of the event. "I built special speaker columns on the hills and had 16 loudspeaker arrays in a square platform going up to the hill on 70-foot [21 m] towers. We set it up for 150,000 to 200,000 people. Of course, 500,000 showed up."[48] ALTEC designed marine plywood cabinets that weighed half a ton apiece and stood 6 feet (1.8 m) tall, almost 4 feet (1.2 m) deep, and 3 feet (0.91 m) wide. Each of these enclosures carried four 15-inch (380 mm) JBL D140 loudspeakers. The tweeters consisted of 4×2-Cell & 2×10-Cell Altec Horns. Behind the stage were three transformers providing 2,000 amperes of current to power the amplification setup.[49][page needed] For many years this system was collectively referred to as the Woodstock Bins.[50] The live performances were captured on two 8-track Scully recorders in a tractor trailer back stage by Edwin Kramer and Lee Osbourne on 1-inch Scotch recording tape at 15 ips, then mixed at the Record Plant studio in New York.[51]"#.to_string();

    let doc_reddit_creep_shots = r#"A year after the closure of r/jailbait, another subreddit called r/CreepShots drew controversy in the press for hosting sexualized images of women without their knowledge.[34] In the wake of this media attention, u/violentacrez was added to r/CreepShots as a moderator;[35] reports emerged that Gawker reporter Adrian Chen was planning an exposé that would reveal the real-life identity of this user, who moderated dozens of controversial subreddits, as well as a few hundred general-interest communities. Several major subreddits banned links to Gawker in response to the impending exposé, and the account u/violentacrez was deleted.[36][37][38] Moderators defended their decisions to block the site from these sections of Reddit on the basis that the impending report was "doxing" (a term for exposing the identity of a pseudonymous person), and that such exposure threatened the site's structural integrity.[38]"#.to_string();

    let doc_ids = qdrant
        .add_documents(
            vec![
                doc_dog_definition,
                doc_woodstock_sound,
                doc_reddit_creep_shots,
            ]
            .into_iter()
            .map(Document::new)
            .collect(),
        )
        .await
        .unwrap();

    println!("Documents stored under IDs: {:?}", doc_ids);
    qdrant
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::var("OPENAI_API_KEY").expect(
        "You need an OPENAI_API_KEY env var with a valid OpenAI API key to run this example",
    );
    let qdrant = build_local_qdrant().await;

    let exec = Executor::new_default();

    let mut tool_collection = ToolCollection::<Multitool>::new();
    tool_collection.add_tool(BashTool::new().into());
    tool_collection.add_tool(QdrantTool::new(qdrant, "random facts", "all sorts of facts").into());

    let template = PromptTemplate::combine(vec![
        tool_collection.to_prompt_template().unwrap(),
        PromptTemplate::tera("Please perform the following task: {{task}}."),
    ]);

    let task = "Tell me something about dogs";

    let prompt = ChatPrompt::builder()
        .system("You are an automated agent for performing tasks. Your output must always be YAML.")
        .add_message(ChatMessage::from_template(ChatRole::User, template))
        .build()
        .unwrap();

    let result = Step::for_prompt(prompt)
        .run(&Parameters::new().with("task", task), &exec)
        .await?;

    println!("{}", result);
    match tool_collection.process_chat_input(&result.primary_textual_output().await.unwrap()) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
