//! Embedding inference (FastEmbed) and vector similarity helpers for Agent Vault.

pub mod engine;
pub mod similarity;

pub use engine::{EmbedEngine, EmbedEngineHandle, EmbedError, DEFAULT_EMBEDDING_MODEL_ID};
pub use similarity::{cosine_similarity, dot_product, euclidean_distance, l2_norm, normalize};
