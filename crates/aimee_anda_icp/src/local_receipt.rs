use std::path::{Path, PathBuf};

use aimee_anda::{
    AndaError, AndaResult, EternalMode, EternalReceipt, EternalStore, PathwayCheckpoint,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use tokio::fs;

use crate::IcpResult;

/// Portable capsule payload written alongside a local eternal receipt.
#[derive(Debug, Clone, Serialize)]
pub struct PathwayCapsule {
    pub schema: &'static str,
    pub conversation_id: String,
    pub pathway_id: String,
    pub seq: u64,
    pub kind: String,
    pub content_hash: String,
    pub parent_hash: String,
    pub message_count: usize,
    pub checkpoint: PathwayCheckpoint,
}

impl PathwayCapsule {
    const SCHEMA: &'static str = "aimee.anda.pathway_capsule.v1";

    /// Builds a capsule from a checkpoint.
    pub fn from_checkpoint(checkpoint: &PathwayCheckpoint) -> Self {
        Self {
            schema: Self::SCHEMA,
            conversation_id: checkpoint.conversation_id.into_string(),
            pathway_id: checkpoint.pathway_id.into_string(),
            seq: checkpoint.seq,
            kind: checkpoint.kind.to_string(),
            content_hash: checkpoint.content_hash.clone(),
            parent_hash: checkpoint.parent_hash.clone(),
            message_count: checkpoint.message_count,
            checkpoint: checkpoint.clone(),
        }
    }

    /// SHA-256 of the canonical JSON capsule body (excluding nested receipt fields noise is ok).
    pub fn digest(&self) -> IcpResult<String> {
        let bytes = serde_json::to_vec(self)?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        Ok(hex::encode(hasher.finalize()))
    }
}

/// Local filesystem eternal store.
///
/// Writes:
/// ```text
/// {root}/{conversation_id}/{seq:020}-{content_hash_prefix}.capsule.json
/// {root}/{conversation_id}/{seq:020}-{content_hash_prefix}.receipt.json
/// ```
#[derive(Debug, Clone)]
pub struct LocalReceiptEternalStore {
    root: PathBuf,
}

impl LocalReceiptEternalStore {
    /// Creates a store under `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn paths(&self, checkpoint: &PathwayCheckpoint) -> (PathBuf, PathBuf) {
        let dir = self.root.join(checkpoint.conversation_id.into_string());
        let prefix = format!(
            "{:020}-{}",
            checkpoint.seq,
            checkpoint
                .content_hash
                .get(..12)
                .unwrap_or(&checkpoint.content_hash)
        );
        (
            dir.join(format!("{prefix}.capsule.json")),
            dir.join(format!("{prefix}.receipt.json")),
        )
    }

    async fn write_json(path: &Path, value: &impl Serialize) -> IcpResult<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let bytes = serde_json::to_vec_pretty(value)?;
        fs::write(path, bytes).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl EternalStore for LocalReceiptEternalStore {
    async fn export_checkpoint(
        &self,
        checkpoint: &PathwayCheckpoint,
        label: &str,
    ) -> AndaResult<EternalReceipt> {
        let capsule = PathwayCapsule::from_checkpoint(checkpoint);
        let digest = capsule
            .digest()
            .map_err(|e| AndaError::Eternal(e.to_string()))?;
        let (capsule_path, receipt_path) = self.paths(checkpoint);

        Self::write_json(&capsule_path, &capsule)
            .await
            .map_err(|e| AndaError::Eternal(e.to_string()))?;

        let receipt = EternalReceipt::success(
            EternalMode::Local,
            label,
            checkpoint.content_hash.clone(),
            capsule_path.display().to_string(),
        )
        .detail(format!(
            "capsule_digest={digest}; receipt={}",
            receipt_path.display()
        ));

        Self::write_json(&receipt_path, &receipt)
            .await
            .map_err(|e| AndaError::Eternal(e.to_string()))?;

        Ok(receipt)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    use super::*;
    use aimee_anda::{CheckpointKind, PathwayCheckpoint, PathwayId};
    use aimee_domain::Conversation;

    #[tokio::test]
    async fn test_local_receipt_writes_capsule_and_receipt() {
        let tmp = TempDir::new().unwrap();
        let store = LocalReceiptEternalStore::new(tmp.path());
        let conversation = Conversation::generate();
        let checkpoint = PathwayCheckpoint::from_conversation(
            PathwayId::generate(),
            conversation,
            1,
            PathwayCheckpoint::GENESIS_PARENT,
            CheckpointKind::TurnEnd,
            None,
        );

        let actual = store
            .export_checkpoint(&checkpoint, "aimee-test-1")
            .await
            .unwrap();

        assert!(actual.ok);
        assert_eq!(actual.mode, EternalMode::Local);
        assert_eq!(actual.content_hash, checkpoint.content_hash);
        assert!(PathBuf::from(&actual.location).exists());
    }
}
