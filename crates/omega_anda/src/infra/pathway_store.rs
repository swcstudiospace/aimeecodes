use omega_domain::ConversationId;

use crate::{AndaResult, PathwayCheckpoint, PathwayId, SessionPathway};

/// Persistence for session pathways and checkpoints.
///
/// Implementations must treat checkpoints as append-only except for explicit
/// truncate-after used during rollback.
#[async_trait::async_trait]
pub trait PathwayStore: Send + Sync {
    /// Loads pathway metadata for a conversation, if present.
    async fn get_pathway(
        &self,
        conversation_id: &ConversationId,
    ) -> AndaResult<Option<SessionPathway>>;

    /// Creates or updates pathway metadata.
    async fn upsert_pathway(&self, pathway: SessionPathway) -> AndaResult<()>;

    /// Appends a checkpoint. Callers must ensure `seq == head_seq + 1`.
    async fn append_checkpoint(&self, checkpoint: PathwayCheckpoint) -> AndaResult<()>;

    /// Returns a checkpoint by sequence number.
    async fn get_checkpoint(
        &self,
        conversation_id: &ConversationId,
        seq: u64,
    ) -> AndaResult<Option<PathwayCheckpoint>>;

    /// Lists checkpoints in ascending sequence order.
    async fn list_checkpoints(
        &self,
        conversation_id: &ConversationId,
    ) -> AndaResult<Vec<PathwayCheckpoint>>;

    /// Removes checkpoints with `seq > after_seq` and returns how many were removed.
    async fn truncate_after(
        &self,
        conversation_id: &ConversationId,
        after_seq: u64,
    ) -> AndaResult<usize>;

    /// Returns pathway id if known.
    async fn pathway_id(
        &self,
        conversation_id: &ConversationId,
    ) -> AndaResult<Option<PathwayId>> {
        Ok(self.get_pathway(conversation_id).await?.map(|p| p.pathway_id))
    }
}
