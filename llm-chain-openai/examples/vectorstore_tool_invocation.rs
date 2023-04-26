use llm_chain::executor;
use llm_chain::output::Output;
use llm_chain::prompt::{ChatMessageCollection, StringTemplate};
use llm_chain::step::Step;
use std::sync::Arc;

use async_trait::async_trait;

use llm_chain::schema::{Document, EmptyMetadata};
use llm_chain::tools::tools::{
    BashTool, VectorStoreTool, VectorStoreToolError, VectorStoreToolInput, VectorStoreToolOutput,
};
use llm_chain::tools::tools::{BashToolError, BashToolInput, BashToolOutput};
use llm_chain::tools::{Tool, ToolCollection, ToolDescription, ToolError};
use llm_chain::traits::VectorStore;
use llm_chain::vectorstores::qdrant::{Qdrant, QdrantError};
use llm_chain::{multitool, parameters};

use llm_chain_openai::embeddings::{Embeddings, OpenAIEmbeddingsError};
use qdrant_client::prelude::{QdrantClient, QdrantClientConfig};
use qdrant_client::qdrant::{CreateCollection, Distance, VectorParams, VectorsConfig};
use serde::{Deserialize, Serialize};
use thiserror::Error;
// A simple example generating a prompt with some tools.

// `multitool!` macro cannot handle generic annotations as of now; for now you will need to pass concrete arguments and alias your types
type QdrantTool = VectorStoreTool<Embeddings, EmptyMetadata, Qdrant<Embeddings, EmptyMetadata>>;
type QdrantToolInput = VectorStoreToolInput;
type QdrantToolOutput = VectorStoreToolOutput;
type QdrantToolError =
    VectorStoreToolError<QdrantError<OpenAIEmbeddingsError>, OpenAIEmbeddingsError>;

multitool!(
    Multitool,
    MultitoolInput,
    MultitoolOutput,
    MultitoolError,
    QdrantTool,
    QdrantToolInput,
    QdrantToolOutput,
    QdrantToolError,
    BashTool,
    BashToolInput,
    BashToolOutput,
    BashToolError
);

async fn build_local_qdrant() -> Qdrant<Embeddings, EmptyMetadata> {
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

    let qdrant = Qdrant::<llm_chain_openai::embeddings::Embeddings, EmptyMetadata>::new(
        client,
        collection_name,
        embeddings,
        None,
        None,
    );

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

    let exec = executor!().unwrap();

    let mut tool_collection = ToolCollection::<Multitool>::new();

    // WHY IS THERE A BASH TOOL IN THIS EXAMPLE?
    //
    // Models usually understand what Bash can do pretty well and may try to `curl` data from websites instead of using your vector database;
    //
    // This is a situation that you may encounter with other tools from our library, as well as your own.
    //
    // If it so happens that the model defers to a more general tool, it most likely means that the tool's prompt
    // is not aligned with what you want the model to do.
    //
    // Note that it does not necessarily mean that you did not describe your tool well -
    // a person or a model familiar with your tool would most likely know when and how they can use your tool based on their prior knowledge and given your prompt.
    //
    // However, if the model has not seen enough examples of the tool in action (as is likely the case with vector databases),
    // it will most likely defer to a tool that it knows.
    //
    // Try to adjust the `topic` and `topic_context` arguments if that happens, so that they describe situations when the model MUST use the specific tool.
    //
    // Treat this as a playground for real world tool usage issues. You can always remove the BashTool from multitool! and ToolCollection.
    tool_collection.add_tool(BashTool::new().into());
    tool_collection.add_tool(
        QdrantTool::new(
            qdrant,
            "factual information and trivia",
            "facts, news, knowledge or trivia",
        )
        .into(),
    );

    let task = "Tell me something about dogs";

    let prompt = ChatMessageCollection::new()
        .with_system(StringTemplate::tera(
            "You are an automated agent for performing tasks. Your output must always be YAML.",
        ))
        .with_user(StringTemplate::combine(vec![
            tool_collection.to_prompt_template().unwrap(),
            StringTemplate::tera("Please perform the following task: {{task}}."),
        ]));

    let result = Step::for_prompt_template(prompt.into())
        .run(&parameters!("task" => task), &exec)
        .await
        .unwrap();

    println!("{}", result);
    match tool_collection
        .process_chat_input(&result.primary_textual_output().await.unwrap())
        .await
    {
        Ok(output) => println!("{}", output),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
