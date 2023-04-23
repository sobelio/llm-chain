//! Vector stores store vectors for embeddings which are then used to find similar documents
//!
//! Vector stores are used to store vectors for embeddings. This allows us to find documents that are nearby in the vector space. This is useful for finding similar documents, or for finding documents that are related to a given document.
//!
//! This, in turn, is useful for giving the LLM memory of previous things it has worked with and to be able to retrive things the LLM is interested in.
#[cfg(feature = "qdrant")]
pub mod qdrant;
