use anyhow::Ok;
use llm_chain::{
    schema::{Document, EmptyMetadata},
    traits::VectorStore,
};
use llm_chain_milvus::Milvus;
use milvus::client::Client as MilvusClient;
use milvus::schema::CollectionSchemaBuilder;
use milvus::schema::FieldSchema;
use std::sync::Arc;
use std::vec;

use async_trait::async_trait;
use llm_chain::traits;
use rand::prelude::*;
use thiserror::Error;

struct RandomEmbedder {
    dim: usize,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RandomEmbedderError {
    #[error("empty error")]
    Empty,
}

impl traits::EmbeddingsError for RandomEmbedderError {}

#[async_trait]
impl traits::Embeddings for RandomEmbedder {
    type Error = RandomEmbedderError;

    async fn embed_texts(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, Self::Error> {
        let mut rng = rand::thread_rng();
        let mut vecs = Vec::new();
        for _ in 0..texts.len() {
            let mut data: Vec<f32> = Vec::new();

            for _ in 1..=(self.dim) {
                let val = rng.gen();
                data.push(val);
            }
            vecs.push(data);
        }

        Ok(vecs).map_err(|_| RandomEmbedderError::Empty)
    }

    async fn embed_query(&self, _query: String) -> Result<Vec<f32>, Self::Error> {
        let mut rng = rand::thread_rng();
        let mut query_vec: Vec<f32> = Vec::new();
        for _ in 1..=(self.dim) {
            let val = rng.gen();
            query_vec.push(val);
        }
        Ok(query_vec).map_err(|_| RandomEmbedderError::Empty)
    }
}

#[tokio::main]
async fn main() {
    const URL: &str = "http://localhost:19530";

    let collection_name = "test_collection".to_string();
    let client = Arc::new(MilvusClient::new(URL).await.unwrap());
    let embedding_dim: i64 = 256;
    let default_vec_field: &str = "embedding";
    let default_payload_field: &str = "payload";

    if client.has_collection(&collection_name).await.unwrap() {
        let collection = client.get_collection(&collection_name).await.unwrap();
        collection.drop().await.unwrap();
    }
    let schema = CollectionSchemaBuilder::new(&collection_name, "a test collection ")
        .add_field(FieldSchema::new_primary_int64(
            "id",
            "primary key field",
            true,
        ))
        .add_field(FieldSchema::new_float_vector(
            default_vec_field,
            "vector embedding field",
            embedding_dim,
        ))
        .add_field(FieldSchema::new_varchar(
            default_payload_field,
            "vector embedding field",
            200,
        ))
        .build()
        .unwrap();

    let _ = client
        .create_collection(schema.clone(), None)
        .await
        .unwrap();

    // let embeddings = llm_chain_openai::embeddings::Embeddings::default();
    let embedder = RandomEmbedder {
        dim: embedding_dim as usize,
    };

    let milvus: Milvus<_, EmptyMetadata> = Milvus::new(
        client,
        collection_name.clone(),
        default_vec_field.to_string(),
        Some(default_payload_field.to_string()),
        None,
        None,
        embedder,
    );
    let doc_dog_definition = r#"The dog (Canis familiaris[4][5] or Canis lupus familiaris[5]) is a domesticated descendant of the wolf. Also called the domestic dog, it is derived from the extinct Pleistocene wolf,[6][7] and the modern wolf is the dog's nearest living relative.[8] Dogs were the first species to be domesticated[9][8] by hunter-gatherers over 15,000 years ago[7] before the development of agriculture.[1] Due to their long association with humans, dogs have expanded to a large number of domestic individuals[10] and gained the ability to thrive on a starch-rich diet that would be inadequate for other canids.[11]

    The dog has been selectively bred over millennia for various behaviors, sensory capabilities, and physical attributes.[12] Dog breeds vary widely in shape, size, and color. They perform many roles for humans, such as hunting, herding, pulling loads, protection, assisting police and the military, companionship, therapy, and aiding disabled people. Over the millennia, dogs became uniquely adapted to human behavior, and the human–canine bond has been a topic of frequent study.[13] This influence on human society has given them the sobriquet of "man's best friend"."#.to_string();

    let doc_woodstock_sound = r#"Sound for the concert was engineered by sound engineer Bill Hanley. "It worked very well", he says of the event. "I built special speaker columns on the hills and had 16 loudspeaker arrays in a square platform going up to the hill on 70-foot [21 m] towers. We set it up for 150,000 to 200,000 people. Of course, 500,000 showed up."[48] ALTEC designed marine plywood cabinets that weighed half a ton apiece and stood 6 feet (1.8 m) tall, almost 4 feet (1.2 m) deep, and 3 feet (0.91 m) wide. Each of these enclosures carried four 15-inch (380 mm) JBL D140 loudspeakers. The tweeters consisted of 4×2-Cell & 2×10-Cell Altec Horns. Behind the stage were three transformers providing 2,000 amperes of current to power the amplification setup.[49][page needed] For many years this system was collectively referred to as the Woodstock Bins.[50] The live performances were captured on two 8-track Scully recorders in a tractor trailer back stage by Edwin Kramer and Lee Osbourne on 1-inch Scotch recording tape at 15 ips, then mixed at the Record Plant studio in New York.[51]"#.to_string();

    let doc_reddit_creep_shots = r#"A year after the closure of r/jailbait, another subreddit called r/CreepShots drew controversy in the press for hosting sexualized images of women without their knowledge.[34] In the wake of this media attention, u/violentacrez was added to r/CreepShots as a moderator;[35] reports emerged that Gawker reporter Adrian Chen was planning an exposé that would reveal the real-life identity of this user, who moderated dozens of controversial subreddits, as well as a few hundred general-interest communities. Several major subreddits banned links to Gawker in response to the impending exposé, and the account u/violentacrez was deleted.[36][37][38] Moderators defended their decisions to block the site from these sections of Reddit on the basis that the impending report was "doxing" (a term for exposing the identity of a pseudonymous person), and that such exposure threatened the site's structural integrity.[38]"#.to_string();

    let doc_ids = milvus
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

    println!("{:?} vectors stored in milvus", doc_ids.len());
    // collection.drop().await.unwrap();
}
