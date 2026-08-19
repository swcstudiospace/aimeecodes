use std::sync::Arc;

use chrono::Utc;
use omega_domain::{Conversation, ConversationId};
use tracing::{info, warn};

use crate::{
    AndaError, AndaResult, CheckpointKind, EternalStore, KipBackend, PathwayCheckpoint,
    PathwayStore, SessionPathway, backends::pathway_event_upsert,
};

/// Configuration for pathway logging side effects.
#[derive(Debug, Clone)]
pub struct PathwayLogOptions {
    /// When true, execute a KIP UPSERT for each checkpoint.
    pub kip_enabled: bool,
    /// When true, export each checkpoint to the eternal store.
    pub eternal_enabled: bool,
    /// Label prefix for eternal receipts.
    pub eternal_label_prefix: String,
    /// When true, pathway log failures bubble up; otherwise they are logged and skipped.
    pub hard_fail: bool,
}

impl Default for PathwayLogOptions {
    fn default() -> Self {
        Self {
            kip_enabled: false,
            eternal_enabled: true,
            eternal_label_prefix: "omega".into(),
            hard_fail: false,
        }
    }
}

/// Logs agent conversation state onto an append-only session pathway.
///
/// # Type parameters
/// * `S` - Pathway store
/// * `K` - KIP backend
/// * `E` - Eternal durability store
pub struct SessionPathwayService<S, K, E> {
    store: Arc<S>,
    kip: Arc<K>,
    eternal: Arc<E>,
    options: PathwayLogOptions,
}

impl<S, K, E> SessionPathwayService<S, K, E> {
    /// Creates a service with the given infrastructure dependencies.
    pub fn new(store: Arc<S>, kip: Arc<K>, eternal: Arc<E>, options: PathwayLogOptions) -> Self {
        Self { store, kip, eternal, options }
    }

    /// Returns a reference to the pathway store.
    pub fn store(&self) -> &S {
        self.store.as_ref()
    }
}

impl<S, K, E> SessionPathwayService<S, K, E>
where
    S: PathwayStore,
    K: KipBackend,
    E: EternalStore,
{
    /// Ensures a pathway exists for the conversation and returns it.
    pub async fn ensure_pathway(
        &self,
        conversation_id: ConversationId,
        agent_id: Option<String>,
    ) -> AndaResult<SessionPathway> {
        if let Some(existing) = self.store.get_pathway(&conversation_id).await? {
            return Ok(existing);
        }
        let mut pathway = SessionPathway::new(conversation_id);
        if let Some(agent_id) = agent_id {
            pathway.agent_id = Some(agent_id);
        }
        self.store.upsert_pathway(pathway.clone()).await?;
        Ok(pathway)
    }

    /// Appends a checkpoint for the current conversation snapshot.
    ///
    /// # Arguments
    /// * `conversation` - Full conversation state to snapshot
    /// * `kind` - Checkpoint kind
    /// * `agent_id` - Optional agent id
    pub async fn log_checkpoint(
        &self,
        conversation: Conversation,
        kind: CheckpointKind,
        agent_id: Option<String>,
    ) -> AndaResult<PathwayCheckpoint> {
        let conversation_id = conversation.id;
        let mut pathway = self
            .ensure_pathway(conversation_id, agent_id.clone())
            .await?;

        let seq = pathway.head_seq + 1;
        let mut checkpoint = PathwayCheckpoint::from_conversation(
            pathway.pathway_id,
            conversation,
            seq,
            pathway.head_hash.clone(),
            kind,
            agent_id.or(pathway.agent_id.clone()),
        );

        if !checkpoint.verify_hash() {
            return Err(AndaError::ContentHashMismatch { seq });
        }

        if self.options.kip_enabled {
            let command = pathway_event_upsert(
                &conversation_id.into_string(),
                seq,
                &checkpoint.content_hash,
                &kind.to_string(),
            );
            match self.kip.execute_kip(&command).await {
                Ok(receipt) => checkpoint.kip_receipt = Some(receipt),
                Err(err) if self.options.hard_fail => return Err(err),
                Err(err) => warn!(error = %err, "kip pathway upsert failed"),
            }
        }

        if self.options.eternal_enabled {
            let label = format!(
                "{}-{}-{}",
                self.options.eternal_label_prefix,
                conversation_id.into_string(),
                seq
            );
            match self.eternal.export_checkpoint(&checkpoint, &label).await {
                Ok(receipt) => checkpoint.eternal_receipt = Some(receipt),
                Err(err) if self.options.hard_fail => return Err(err),
                Err(err) => warn!(error = %err, "eternal pathway export failed"),
            }
        }

        self.store.append_checkpoint(checkpoint.clone()).await?;

        pathway.head_seq = seq;
        pathway.head_hash = checkpoint.content_hash.clone();
        pathway.updated_at = Utc::now();
        if pathway.agent_id.is_none() {
            pathway.agent_id = checkpoint.agent_id.clone();
        }
        self.store.upsert_pathway(pathway).await?;

        info!(
            conversation_id = %conversation_id,
            seq,
            kind = %kind,
            hash = %checkpoint.content_hash,
            "session pathway checkpoint logged"
        );

        Ok(checkpoint)
    }

    /// Rolls the conversation back to checkpoint `seq` (chat state only).
    ///
    /// Truncates later checkpoints and appends a [`CheckpointKind::Rollback`]
    /// marker pointing at the restored snapshot.
    pub async fn rollback_to(
        &self,
        conversation_id: ConversationId,
        seq: u64,
    ) -> AndaResult<Conversation> {
        let pathway = self
            .store
            .get_pathway(&conversation_id)
            .await?
            .ok_or(AndaError::PathwayNotFound(conversation_id))?;

        let target = self
            .store
            .get_checkpoint(&conversation_id, seq)
            .await?
            .ok_or(AndaError::SeqNotFound { conversation_id, seq })?;

        if !target.verify_hash() {
            return Err(AndaError::ContentHashMismatch { seq });
        }

        // Verify chain from genesis through target.
        let all = self.store.list_checkpoints(&conversation_id).await?;
        let mut expected_parent = PathwayCheckpoint::GENESIS_PARENT.to_string();
        for checkpoint in all.iter().filter(|c| c.seq <= seq) {
            if checkpoint.parent_hash != expected_parent {
                return Err(AndaError::HashChainBroken {
                    seq: checkpoint.seq,
                    expected: expected_parent,
                    actual: checkpoint.parent_hash.clone(),
                });
            }
            if !checkpoint.verify_hash() {
                return Err(AndaError::ContentHashMismatch {
                    seq: checkpoint.seq,
                });
            }
            expected_parent = checkpoint.content_hash.clone();
        }

        self.store.truncate_after(&conversation_id, seq).await?;

        let restored = target.conversation_snapshot.clone();
        let mut head = pathway;
        head.head_seq = seq;
        head.head_hash = target.content_hash.clone();
        head.updated_at = Utc::now();
        self.store.upsert_pathway(head).await?;

        // Record rollback marker as the next checkpoint so history remains auditable.
        let _marker = self
            .log_checkpoint(
                restored.clone(),
                CheckpointKind::Rollback,
                target.agent_id.clone(),
            )
            .await?;

        Ok(restored)
    }

    /// Lists checkpoints for a conversation in ascending order.
    pub async fn list_checkpoints(
        &self,
        conversation_id: &ConversationId,
    ) -> AndaResult<Vec<PathwayCheckpoint>> {
        self.store.list_checkpoints(conversation_id).await
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{MemoryPathwayStore, NoopEternalStore, NoopKipBackend};
    use omega_domain::Context;

    fn fixture_service() -> SessionPathwayService<MemoryPathwayStore, NoopKipBackend, NoopEternalStore>
    {
        SessionPathwayService::new(
            Arc::new(MemoryPathwayStore::new()),
            Arc::new(NoopKipBackend),
            Arc::new(NoopEternalStore),
            PathwayLogOptions {
                kip_enabled: true,
                eternal_enabled: true,
                eternal_label_prefix: "test".into(),
                hard_fail: true,
            },
        )
    }

    fn conversation_with_messages(n: usize) -> Conversation {
        let mut conversation = Conversation::generate().title("rollback demo");
        let mut ctx = Context::default();
        for i in 0..n {
            ctx = ctx.add_message(omega_domain::ContextMessage::user(
                format!("msg-{i}"),
                None,
            ));
        }
        conversation.context = Some(ctx);
        conversation
    }

    #[tokio::test]
    async fn test_log_checkpoint_advances_seq_and_hash_chain() {
        let service = fixture_service();
        let c1 = conversation_with_messages(1);
        let id = c1.id;

        let a = service
            .log_checkpoint(c1.clone(), CheckpointKind::AgentResponse, Some("a".into()))
            .await
            .unwrap();
        let c2 = {
            let mut c = c1;
            c.context = Some(
                c.context
                    .unwrap_or_default()
                    .add_message(omega_domain::ContextMessage::user("msg-1b", None)),
            );
            c
        };
        let b = service
            .log_checkpoint(c2, CheckpointKind::TurnEnd, Some("a".into()))
            .await
            .unwrap();

        assert_eq!(a.seq, 1);
        assert_eq!(b.seq, 2);
        assert_eq!(b.parent_hash, a.content_hash);
        assert!(a.kip_receipt.unwrap().ok);
        assert!(a.eternal_receipt.unwrap().ok);

        let list = service.list_checkpoints(&id).await.unwrap();
        assert_eq!(list.len(), 2);
    }

    #[tokio::test]
    async fn test_rollback_restores_snapshot() {
        let service = fixture_service();
        let c1 = conversation_with_messages(1);
        let id = c1.id;
        let first = service
            .log_checkpoint(c1.clone(), CheckpointKind::AgentResponse, None)
            .await
            .unwrap();

        let mut c2 = c1.clone();
        c2.context = Some(
            c2.context
                .unwrap_or_default()
                .add_message(omega_domain::ContextMessage::user("later", None)),
        );
        service
            .log_checkpoint(c2, CheckpointKind::AgentResponse, None)
            .await
            .unwrap();

        let restored = service.rollback_to(id, first.seq).await.unwrap();
        assert_eq!(restored.len(), first.conversation_snapshot.len());
        assert_eq!(restored.id, id);

        let list = service.list_checkpoints(&id).await.unwrap();
        // seq1 kept, seq2 truncated, rollback marker appended as seq2
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].seq, 1);
        assert_eq!(list[1].kind, CheckpointKind::Rollback);
    }
}
