use crate::{AndaResult, EternalReceipt, PathwayCheckpoint};

/// Exports pathway checkpoints to eternal (optionally on-chain) storage.
#[async_trait::async_trait]
pub trait EternalStore: Send + Sync {
    /// Exports a single checkpoint and returns a durability receipt.
    async fn export_checkpoint(
        &self,
        checkpoint: &PathwayCheckpoint,
        label: &str,
    ) -> AndaResult<EternalReceipt>;
}

/// No-op eternal store that records an in-memory success without persistence.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopEternalStore;

#[async_trait::async_trait]
impl EternalStore for NoopEternalStore {
    async fn export_checkpoint(
        &self,
        checkpoint: &PathwayCheckpoint,
        label: &str,
    ) -> AndaResult<EternalReceipt> {
        Ok(EternalReceipt::ok(
            crate::EternalMode::Local,
            label,
            checkpoint.content_hash.clone(),
            format!("noop://{}", checkpoint.checkpoint_id),
        ))
    }
}
