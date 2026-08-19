use chrono::{DateTime, Utc};
use derive_more::Display;
use derive_setters::Setters;
use omega_domain::{Conversation, ConversationId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::{EternalReceipt, KipReceipt};

/// Unique identifier for a pathway checkpoint.
#[derive(Debug, Display, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct CheckpointId(Uuid);

impl Copy for CheckpointId {}

impl CheckpointId {
    /// Generates a new random checkpoint id.
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    /// Returns the id as a hyphenated UUID string.
    pub fn into_string(&self) -> String {
        self.0.to_string()
    }
}

/// Kind of event that produced a pathway checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Default)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointKind {
    /// User message boundary.
    UserTurn,
    /// LLM assistant response received.
    #[default]
    AgentResponse,
    /// Tool call finished.
    ToolEnd,
    /// Full agent turn completed.
    TurnEnd,
    /// Explicit rollback marker.
    Rollback,
    /// Manual / external checkpoint.
    Manual,
}

/// Immutable checkpoint along a session pathway.
///
/// Checkpoints form a hash chain via [`parent_hash`](Self::parent_hash) and
/// [`content_hash`](Self::content_hash) so history can be audited and rolled back.
#[derive(Debug, Clone, Serialize, Deserialize, Setters, PartialEq)]
#[setters(into, strip_option)]
pub struct PathwayCheckpoint {
    pub checkpoint_id: CheckpointId,
    pub pathway_id: super::PathwayId,
    pub conversation_id: ConversationId,
    /// Monotonic sequence within the pathway (1-based).
    pub seq: u64,
    /// Hex SHA-256 of the previous checkpoint (`"genesis"` for the first).
    pub parent_hash: String,
    /// Hex SHA-256 of this checkpoint payload.
    pub content_hash: String,
    pub kind: CheckpointKind,
    pub agent_id: Option<String>,
    pub message_count: usize,
    /// Full conversation snapshot used for chat-only rollback.
    pub conversation_snapshot: Conversation,
    pub created_at: DateTime<Utc>,
    pub kip_receipt: Option<KipReceipt>,
    pub eternal_receipt: Option<EternalReceipt>,
}

impl PathwayCheckpoint {
    /// Genesis parent hash marker for the first checkpoint in a pathway.
    pub const GENESIS_PARENT: &'static str = "genesis";

    /// Builds a checkpoint from a conversation snapshot and computes the content hash.
    ///
    /// # Arguments
    /// * `pathway_id` - Owning pathway
    /// * `conversation` - Conversation state to snapshot
    /// * `seq` - Next sequence number
    /// * `parent_hash` - Previous content hash or [`GENESIS_PARENT`](Self::GENESIS_PARENT)
    /// * `kind` - Checkpoint kind
    /// * `agent_id` - Optional agent identifier
    pub fn from_conversation(
        pathway_id: super::PathwayId,
        conversation: Conversation,
        seq: u64,
        parent_hash: impl Into<String>,
        kind: CheckpointKind,
        agent_id: Option<String>,
    ) -> Self {
        let parent_hash = parent_hash.into();
        let message_count = conversation.len();
        let conversation_id = conversation.id;
        let created_at = Utc::now();
        let content_hash = Self::compute_content_hash(
            &parent_hash,
            seq,
            kind,
            &conversation,
            message_count,
        );

        Self {
            checkpoint_id: CheckpointId::generate(),
            pathway_id,
            conversation_id,
            seq,
            parent_hash,
            content_hash,
            kind,
            agent_id,
            message_count,
            conversation_snapshot: conversation,
            created_at,
            kip_receipt: None,
            eternal_receipt: None,
        }
    }

    /// Computes the canonical content hash for a checkpoint payload.
    pub fn compute_content_hash(
        parent_hash: &str,
        seq: u64,
        kind: CheckpointKind,
        conversation: &Conversation,
        message_count: usize,
    ) -> String {
        let snapshot = serde_json::to_vec(conversation).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(parent_hash.as_bytes());
        hasher.update(b"|");
        hasher.update(seq.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(kind.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(message_count.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(&snapshot);
        hex::encode(hasher.finalize())
    }

    /// Verifies this checkpoint's content hash matches its payload.
    pub fn verify_hash(&self) -> bool {
        let expected = Self::compute_content_hash(
            &self.parent_hash,
            self.seq,
            self.kind,
            &self.conversation_snapshot,
            self.message_count,
        );
        expected == self.content_hash
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::PathwayId;

    fn fixture_conversation() -> Conversation {
        Conversation::generate().title("pathway test")
    }

    #[test]
    fn test_checkpoint_hash_is_stable_for_same_payload() {
        let conversation = fixture_conversation();
        let pathway_id = PathwayId::generate();

        let actual = PathwayCheckpoint::compute_content_hash(
            PathwayCheckpoint::GENESIS_PARENT,
            1,
            CheckpointKind::AgentResponse,
            &conversation,
            conversation.len(),
        );
        let expected = PathwayCheckpoint::compute_content_hash(
            PathwayCheckpoint::GENESIS_PARENT,
            1,
            CheckpointKind::AgentResponse,
            &conversation,
            conversation.len(),
        );

        assert_eq!(actual, expected);
        assert_eq!(actual.len(), 64);
    }

    #[test]
    fn test_from_conversation_verifies() {
        let setup = fixture_conversation();
        let actual = PathwayCheckpoint::from_conversation(
            PathwayId::generate(),
            setup,
            1,
            PathwayCheckpoint::GENESIS_PARENT,
            CheckpointKind::TurnEnd,
            Some("omega".into()),
        );
        assert!(actual.verify_hash());
        assert_eq!(actual.seq, 1);
        assert_eq!(actual.parent_hash, PathwayCheckpoint::GENESIS_PARENT);
    }

    #[test]
    fn test_hash_changes_when_parent_changes() {
        let conversation = fixture_conversation();
        let a = PathwayCheckpoint::compute_content_hash(
            "aaa",
            2,
            CheckpointKind::AgentResponse,
            &conversation,
            0,
        );
        let b = PathwayCheckpoint::compute_content_hash(
            "bbb",
            2,
            CheckpointKind::AgentResponse,
            &conversation,
            0,
        );
        assert_ne!(a, b);
    }
}
