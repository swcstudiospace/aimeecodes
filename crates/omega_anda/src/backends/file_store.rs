use std::path::{Path, PathBuf};

use omega_domain::ConversationId;
use tokio::fs;

use crate::{
    AndaError, AndaResult, PathwayCheckpoint, PathwayStore, SessionPathway,
};

/// File-backed pathway store.
///
/// Layout:
/// ```text
/// {root}/{conversation_id}/pathway.json
/// {root}/{conversation_id}/checkpoints/{seq:020}.json
/// ```
#[derive(Debug, Clone)]
pub struct FilePathwayStore {
    root: PathBuf,
}

impl FilePathwayStore {
    /// Creates a store rooted at `root`, creating the directory if needed.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn conversation_dir(&self, conversation_id: &ConversationId) -> PathBuf {
        self.root.join(conversation_id.into_string())
    }

    fn pathway_path(&self, conversation_id: &ConversationId) -> PathBuf {
        self.conversation_dir(conversation_id).join("pathway.json")
    }

    fn checkpoints_dir(&self, conversation_id: &ConversationId) -> PathBuf {
        self.conversation_dir(conversation_id).join("checkpoints")
    }

    fn checkpoint_path(&self, conversation_id: &ConversationId, seq: u64) -> PathBuf {
        self.checkpoints_dir(conversation_id)
            .join(format!("{seq:020}.json"))
    }

    async fn ensure_dirs(&self, conversation_id: &ConversationId) -> AndaResult<()> {
        fs::create_dir_all(self.checkpoints_dir(conversation_id))
            .await
            .map_err(|e| AndaError::Other(e.into()))?;
        Ok(())
    }

    async fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> AndaResult<Option<T>> {
        match fs::read(path).await {
            Ok(bytes) => {
                let value = serde_json::from_slice(&bytes)
                    .map_err(|e| AndaError::Other(e.into()))?;
                Ok(Some(value))
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(AndaError::Other(err.into())),
        }
    }

    async fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> AndaResult<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| AndaError::Other(e.into()))?;
        }
        let bytes = serde_json::to_vec_pretty(value).map_err(|e| AndaError::Other(e.into()))?;
        fs::write(path, bytes)
            .await
            .map_err(|e| AndaError::Other(e.into()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl PathwayStore for FilePathwayStore {
    async fn get_pathway(
        &self,
        conversation_id: &ConversationId,
    ) -> AndaResult<Option<SessionPathway>> {
        Self::read_json(&self.pathway_path(conversation_id)).await
    }

    async fn upsert_pathway(&self, pathway: SessionPathway) -> AndaResult<()> {
        self.ensure_dirs(&pathway.conversation_id).await?;
        Self::write_json(&self.pathway_path(&pathway.conversation_id), &pathway).await
    }

    async fn append_checkpoint(&self, checkpoint: PathwayCheckpoint) -> AndaResult<()> {
        self.ensure_dirs(&checkpoint.conversation_id).await?;
        let path = self.checkpoint_path(&checkpoint.conversation_id, checkpoint.seq);
        if fs::try_exists(&path).await.unwrap_or(false) {
            return Err(AndaError::Other(anyhow::anyhow!(
                "checkpoint seq {} already exists",
                checkpoint.seq
            )));
        }
        if let Some(prev) = self
            .get_checkpoint(&checkpoint.conversation_id, checkpoint.seq.saturating_sub(1))
            .await?
        {
            if checkpoint.parent_hash != prev.content_hash {
                return Err(AndaError::HashChainBroken {
                    seq: checkpoint.seq,
                    expected: prev.content_hash,
                    actual: checkpoint.parent_hash.clone(),
                });
            }
        }
        Self::write_json(&path, &checkpoint).await
    }

    async fn get_checkpoint(
        &self,
        conversation_id: &ConversationId,
        seq: u64,
    ) -> AndaResult<Option<PathwayCheckpoint>> {
        Self::read_json(&self.checkpoint_path(conversation_id, seq)).await
    }

    async fn list_checkpoints(
        &self,
        conversation_id: &ConversationId,
    ) -> AndaResult<Vec<PathwayCheckpoint>> {
        let dir = self.checkpoints_dir(conversation_id);
        let mut entries = match fs::read_dir(&dir).await {
            Ok(entries) => entries,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
            Err(err) => return Err(AndaError::Other(err.into())),
        };

        let mut paths = Vec::new();
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| AndaError::Other(e.into()))?
        {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                paths.push(path);
            }
        }
        paths.sort();

        let mut out = Vec::with_capacity(paths.len());
        for path in paths {
            if let Some(checkpoint) = Self::read_json::<PathwayCheckpoint>(&path).await? {
                out.push(checkpoint);
            }
        }
        Ok(out)
    }

    async fn truncate_after(
        &self,
        conversation_id: &ConversationId,
        after_seq: u64,
    ) -> AndaResult<usize> {
        let checkpoints = self.list_checkpoints(conversation_id).await?;
        let mut removed = 0usize;
        for checkpoint in checkpoints {
            if checkpoint.seq > after_seq {
                let path = self.checkpoint_path(conversation_id, checkpoint.seq);
                fs::remove_file(&path)
                    .await
                    .map_err(|e| AndaError::Other(e.into()))?;
                removed += 1;
            }
        }
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    use super::*;
    use crate::{CheckpointKind, PathwayCheckpoint, PathwayId};

    #[tokio::test]
    async fn test_file_store_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let store = FilePathwayStore::new(tmp.path());
        let conversation = omega_domain::Conversation::generate().title("file store");
        let conversation_id = conversation.id;
        let mut pathway = SessionPathway::new(conversation_id);
        let checkpoint = PathwayCheckpoint::from_conversation(
            pathway.pathway_id,
            conversation,
            1,
            PathwayCheckpoint::GENESIS_PARENT,
            CheckpointKind::TurnEnd,
            Some("agent".into()),
        );
        pathway.head_seq = 1;
        pathway.head_hash = checkpoint.content_hash.clone();

        store.upsert_pathway(pathway.clone()).await.unwrap();
        store.append_checkpoint(checkpoint.clone()).await.unwrap();

        let actual_pathway = store.get_pathway(&conversation_id).await.unwrap().unwrap();
        let actual_list = store.list_checkpoints(&conversation_id).await.unwrap();

        assert_eq!(actual_pathway.pathway_id, pathway.pathway_id);
        assert_eq!(actual_list.len(), 1);
        assert_eq!(actual_list[0].content_hash, checkpoint.content_hash);
    }
}
