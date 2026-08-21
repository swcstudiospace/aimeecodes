use aimee_domain::ConversationId;
use thiserror::Error;

use super::{CheckpointId, PathwayId};

/// Domain errors for Anda pathway operations.
#[derive(Debug, Error)]
pub enum AndaError {
    /// Pathway was not found for the given conversation.
    #[error("pathway not found for conversation {0}")]
    PathwayNotFound(ConversationId),

    /// Checkpoint was not found.
    #[error("checkpoint {checkpoint_id} not found on pathway {pathway_id}")]
    CheckpointNotFound {
        pathway_id: PathwayId,
        checkpoint_id: CheckpointId,
    },

    /// Sequence number does not exist on the pathway.
    #[error("checkpoint seq {seq} not found for conversation {conversation_id}")]
    SeqNotFound {
        conversation_id: ConversationId,
        seq: u64,
    },

    /// Hash chain verification failed.
    #[error("pathway hash chain broken at seq {seq}: expected {expected}, got {actual}")]
    HashChainBroken {
        seq: u64,
        expected: String,
        actual: String,
    },

    /// Checkpoint content hash mismatch.
    #[error("checkpoint content hash mismatch at seq {seq}")]
    ContentHashMismatch { seq: u64 },

    /// KIP backend failure.
    #[error("kip backend error: {0}")]
    Kip(String),

    /// Eternal store failure.
    #[error("eternal store error: {0}")]
    Eternal(String),

    /// Underlying I/O or serialization failure.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Result alias for Anda domain operations.
pub type AndaResult<T> = std::result::Result<T, AndaError>;
