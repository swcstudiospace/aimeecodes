use std::collections::HashMap;
use std::sync::Arc;

use omega_domain::ConversationId;
use tokio::sync::RwLock;

use crate::{
    AndaError, AndaResult, PathwayCheckpoint, PathwayStore, SessionPathway,
};

/// In-memory pathway store for tests and ephemeral sessions.
#[derive(Debug, Default, Clone)]
pub struct MemoryPathwayStore {
    inner: Arc<RwLock<MemoryState>>,
}

#[derive(Debug, Default)]
struct MemoryState {
    pathways: HashMap<ConversationId, SessionPathway>,
    checkpoints: HashMap<ConversationId, Vec<PathwayCheckpoint>>,
}

impl MemoryPathwayStore {
    /// Creates an empty store.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl PathwayStore for MemoryPathwayStore {
    async fn get_pathway(
        &self,
        conversation_id: &ConversationId,
    ) -> AndaResult<Option<SessionPathway>> {
        Ok(self.inner.read().await.pathways.get(conversation_id).cloned())
    }

    async fn upsert_pathway(&self, pathway: SessionPathway) -> AndaResult<()> {
        self.inner
            .write()
            .await
            .pathways
            .insert(pathway.conversation_id, pathway);
        Ok(())
    }

    async fn append_checkpoint(&self, checkpoint: PathwayCheckpoint) -> AndaResult<()> {
        let mut guard = self.inner.write().await;
        let list = guard
            .checkpoints
            .entry(checkpoint.conversation_id)
            .or_default();
        if let Some(last) = list.last() {
            if checkpoint.seq != last.seq + 1 {
                return Err(AndaError::Other(anyhow::anyhow!(
                    "non-monotonic seq: got {}, expected {}",
                    checkpoint.seq,
                    last.seq + 1
                )));
            }
            if checkpoint.parent_hash != last.content_hash {
                return Err(AndaError::HashChainBroken {
                    seq: checkpoint.seq,
                    expected: last.content_hash.clone(),
                    actual: checkpoint.parent_hash.clone(),
                });
            }
        } else if checkpoint.seq != 1 {
            return Err(AndaError::Other(anyhow::anyhow!(
                "first checkpoint must be seq 1, got {}",
                checkpoint.seq
            )));
        }
        list.push(checkpoint);
        Ok(())
    }

    async fn get_checkpoint(
        &self,
        conversation_id: &ConversationId,
        seq: u64,
    ) -> AndaResult<Option<PathwayCheckpoint>> {
        Ok(self
            .inner
            .read()
            .await
            .checkpoints
            .get(conversation_id)
            .and_then(|list| list.iter().find(|c| c.seq == seq).cloned()))
    }

    async fn list_checkpoints(
        &self,
        conversation_id: &ConversationId,
    ) -> AndaResult<Vec<PathwayCheckpoint>> {
        Ok(self
            .inner
            .read()
            .await
            .checkpoints
            .get(conversation_id)
            .cloned()
            .unwrap_or_default())
    }

    async fn truncate_after(
        &self,
        conversation_id: &ConversationId,
        after_seq: u64,
    ) -> AndaResult<usize> {
        let mut guard = self.inner.write().await;
        let Some(list) = guard.checkpoints.get_mut(conversation_id) else {
            return Ok(0);
        };
        let before = list.len();
        list.retain(|c| c.seq <= after_seq);
        Ok(before.saturating_sub(list.len()))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{CheckpointKind, PathwayCheckpoint, PathwayId};

    #[tokio::test]
    async fn test_append_and_list() {
        let store = MemoryPathwayStore::new();
        let conversation = omega_domain::Conversation::generate();
        let conversation_id = conversation.id;
        let checkpoint = PathwayCheckpoint::from_conversation(
            PathwayId::generate(),
            conversation,
            1,
            PathwayCheckpoint::GENESIS_PARENT,
            CheckpointKind::AgentResponse,
            None,
        );

        store.append_checkpoint(checkpoint.clone()).await.unwrap();
        let actual = store.list_checkpoints(&conversation_id).await.unwrap();
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0].content_hash, checkpoint.content_hash);
    }
}
