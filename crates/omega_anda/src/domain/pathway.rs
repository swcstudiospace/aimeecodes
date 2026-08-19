use chrono::{DateTime, Utc};
use derive_more::Display;
use derive_setters::Setters;
use omega_domain::ConversationId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a session pathway.
#[derive(Debug, Display, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct PathwayId(Uuid);

impl Copy for PathwayId {}

impl PathwayId {
    /// Generates a new random pathway id.
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    /// Returns the id as a hyphenated UUID string.
    pub fn into_string(&self) -> String {
        self.0.to_string()
    }
}

/// Append-only session pathway metadata for a conversation.
///
/// The checkpoint bodies live in the [`PathwayStore`](crate::PathwayStore);
/// this type tracks the head of the hash chain.
#[derive(Debug, Clone, Serialize, Deserialize, Setters, PartialEq)]
#[setters(into, strip_option)]
pub struct SessionPathway {
    pub pathway_id: PathwayId,
    pub conversation_id: ConversationId,
    pub agent_id: Option<String>,
    /// Latest checkpoint sequence (0 if empty).
    pub head_seq: u64,
    /// Latest checkpoint content hash (`genesis` if empty).
    pub head_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SessionPathway {
    /// Creates a new empty pathway for a conversation.
    pub fn new(conversation_id: ConversationId) -> Self {
        let now = Utc::now();
        Self {
            pathway_id: PathwayId::generate(),
            conversation_id,
            agent_id: None,
            head_seq: 0,
            head_hash: super::PathwayCheckpoint::GENESIS_PARENT.to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Convenience constructor that also sets the agent id.
    pub fn with_agent(conversation_id: ConversationId, agent_id: impl Into<String>) -> Self {
        Self::new(conversation_id).agent_id(agent_id.into())
    }

    /// Returns true when no checkpoints have been appended.
    pub fn is_empty(&self) -> bool {
        self.head_seq == 0
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::PathwayCheckpoint;

    #[test]
    fn test_new_pathway_is_empty_at_genesis() {
        let actual = SessionPathway::new(ConversationId::generate());
        let expected_hash = PathwayCheckpoint::GENESIS_PARENT;

        assert!(actual.is_empty());
        assert_eq!(actual.head_seq, 0);
        assert_eq!(actual.head_hash, expected_hash);
    }
}
