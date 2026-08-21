//! Builds pathway logging hooks from Aimee Anda configuration.

use std::path::PathBuf;
use std::sync::Arc;

use aimee_anda::{
    AnyKipBackend, EternalStore, FilePathwayStore, NoopEternalStore, PathwayCheckpoint,
    PathwayLogHook, PathwayLogOptions, SessionPathwayService,
};
use aimee_anda_icp::LocalReceiptEternalStore;
use aimee_config::{AndaConfig, AndaEternalMode};
use aimee_domain::{EndPayload, EventData, EventHandle, ResponsePayload};
use tracing::{info, warn};

/// Eternal backend used by the app pathway service.
#[derive(Clone)]
pub(crate) enum AppEternalStore {
    Noop(NoopEternalStore),
    Local(LocalReceiptEternalStore),
}

#[async_trait::async_trait]
impl EternalStore for AppEternalStore {
    async fn export_checkpoint(
        &self,
        checkpoint: &PathwayCheckpoint,
        label: &str,
    ) -> aimee_anda::AndaResult<aimee_anda::EternalReceipt> {
        match self {
            Self::Noop(store) => store.export_checkpoint(checkpoint, label).await,
            Self::Local(store) => store.export_checkpoint(checkpoint, label).await,
        }
    }
}

/// Pair of lifecycle handlers that log session pathways.
pub struct PathwayHooks {
    /// Handler for LLM response events.
    pub(crate) on_response: PathwayLogHook<FilePathwayStore, AnyKipBackend, AppEternalStore>,
    /// Handler for turn-end events.
    pub(crate) on_end: PathwayLogHook<FilePathwayStore, AnyKipBackend, AppEternalStore>,
}

/// Resolves pathway directories from config + Aimee home.
pub fn resolve_pathway_dirs(anda: &AndaConfig, aimee_home: PathBuf) -> (PathBuf, PathBuf) {
    let pathway_dir = anda
        .pathway_dir
        .clone()
        .unwrap_or_else(|| aimee_home.join("pathways"));
    let eternal_dir = anda
        .eternal_dir
        .clone()
        .unwrap_or_else(|| pathway_dir.join("eternal"));
    (pathway_dir, eternal_dir)
}

/// Builds pathway hooks when Anda is enabled; returns `None` when disabled.
///
/// # Arguments
/// * `anda` - Anda configuration section
/// * `aimee_home` - Aimee base directory (`~/.aimee` by default)
/// * `agent_id` - Agent id recorded on checkpoints
pub fn maybe_pathway_hooks(
    anda: &AndaConfig,
    aimee_home: PathBuf,
    agent_id: impl Into<String>,
) -> Option<PathwayHooks> {
    if !anda.enabled {
        return None;
    }

    let (pathway_dir, eternal_dir) = resolve_pathway_dirs(anda, aimee_home);
    let agent_id = agent_id.into();

    let store = Arc::new(FilePathwayStore::new(pathway_dir.clone()));
    let kip_url = if anda.kip_enabled {
        anda.nexus_url.as_deref()
    } else {
        None
    };
    let kip = Arc::new(AnyKipBackend::from_nexus_url(kip_url));

    if matches!(
        anda.eternal_mode,
        AndaEternalMode::IcOss | AndaEternalMode::Canister | AndaEternalMode::S3
    ) && anda.eternal_enabled
    {
        warn!(
            mode = ?anda.eternal_mode,
            "anda eternal mode is not fully wired yet; using local receipts"
        );
    }

    let eternal = Arc::new(if anda.eternal_enabled {
        AppEternalStore::Local(LocalReceiptEternalStore::new(eternal_dir.clone()))
    } else {
        AppEternalStore::Noop(NoopEternalStore)
    });

    let options = PathwayLogOptions {
        kip_enabled: anda.kip_enabled && kip.is_remote(),
        eternal_enabled: anda.eternal_enabled,
        eternal_label_prefix: anda.eternal_label_prefix.clone(),
        hard_fail: anda.hard_fail,
    };

    let service = Arc::new(SessionPathwayService::new(store, kip, eternal, options));
    let base = PathwayLogHook::new(service)
        .agent_id(agent_id.clone())
        .hard_fail(anda.hard_fail);

    info!(
        agent_id = %agent_id,
        pathway_dir = %pathway_dir.display(),
        eternal_dir = %eternal_dir.display(),
        kip = anda.kip_enabled,
        eternal = anda.eternal_enabled,
        "anda session pathway logging enabled"
    );

    Some(PathwayHooks {
        on_response: base
            .clone()
            .log_responses(anda.log_responses)
            .log_turn_end(false),
        on_end: base.log_responses(false).log_turn_end(anda.log_turn_end),
    })
}

/// Summary of a pathway checkpoint for CLI / API surfaces.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PathwayCheckpointSummary {
    pub seq: u64,
    pub kind: String,
    pub content_hash: String,
    pub parent_hash: String,
    pub message_count: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub agent_id: Option<String>,
    pub kip_ok: Option<bool>,
    pub eternal_ok: Option<bool>,
}

impl PathwayCheckpointSummary {
    fn from_checkpoint(checkpoint: &PathwayCheckpoint) -> Self {
        Self {
            seq: checkpoint.seq,
            kind: checkpoint.kind.to_string(),
            content_hash: checkpoint.content_hash.clone(),
            parent_hash: checkpoint.parent_hash.clone(),
            message_count: checkpoint.message_count,
            created_at: checkpoint.created_at,
            agent_id: checkpoint.agent_id.clone(),
            kip_ok: checkpoint.kip_receipt.as_ref().map(|r| r.ok),
            eternal_ok: checkpoint.eternal_receipt.as_ref().map(|r| r.ok),
        }
    }
}

/// Opens the file-backed pathway store for CLI inspection even when Anda is disabled.
pub fn open_pathway_store(anda: Option<&AndaConfig>, aimee_home: PathBuf) -> FilePathwayStore {
    let defaults = AndaConfig::default();
    let anda = anda.unwrap_or(&defaults);
    let (pathway_dir, _) = resolve_pathway_dirs(anda, aimee_home);
    FilePathwayStore::new(pathway_dir)
}

/// Lists hash-chained checkpoints for a conversation.
pub async fn list_session_pathway(
    anda: Option<&AndaConfig>,
    aimee_home: PathBuf,
    conversation_id: &aimee_domain::ConversationId,
) -> anyhow::Result<Vec<PathwayCheckpointSummary>> {
    use aimee_anda::PathwayStore;
    let store = open_pathway_store(anda, aimee_home);
    let checkpoints = store.list_checkpoints(conversation_id).await?;
    Ok(checkpoints
        .iter()
        .map(PathwayCheckpointSummary::from_checkpoint)
        .collect())
}

/// Returns one checkpoint summary by sequence number.
pub async fn show_session_pathway(
    anda: Option<&AndaConfig>,
    aimee_home: PathBuf,
    conversation_id: &aimee_domain::ConversationId,
    seq: u64,
) -> anyhow::Result<Option<PathwayCheckpointSummary>> {
    use aimee_anda::PathwayStore;
    let store = open_pathway_store(anda, aimee_home);
    Ok(store
        .get_checkpoint(conversation_id, seq)
        .await?
        .as_ref()
        .map(PathwayCheckpointSummary::from_checkpoint))
}

/// Rolls chat state back to `seq` and returns the restored conversation snapshot.
pub async fn rollback_session_pathway(
    anda: Option<&AndaConfig>,
    aimee_home: PathBuf,
    conversation_id: aimee_domain::ConversationId,
    seq: u64,
) -> anyhow::Result<aimee_domain::Conversation> {
    let defaults = AndaConfig::default();
    let anda = anda.cloned().unwrap_or(defaults);
    let (pathway_dir, eternal_dir) = resolve_pathway_dirs(&anda, aimee_home);
    let store = Arc::new(FilePathwayStore::new(pathway_dir));
    let kip = Arc::new(AnyKipBackend::from_nexus_url(None));
    let eternal = Arc::new(if anda.eternal_enabled {
        AppEternalStore::Local(LocalReceiptEternalStore::new(eternal_dir))
    } else {
        AppEternalStore::Noop(NoopEternalStore)
    });
    let service = SessionPathwayService::new(
        store,
        kip,
        eternal,
        PathwayLogOptions {
            kip_enabled: false,
            eternal_enabled: anda.eternal_enabled,
            eternal_label_prefix: anda.eternal_label_prefix.clone(),
            hard_fail: true,
        },
    );
    Ok(service.rollback_to(conversation_id, seq).await?)
}

/// Chains pathway response logging onto an existing response handler.
pub fn chain_on_response(
    base: impl EventHandle<EventData<ResponsePayload>> + 'static,
    pathway: Option<&PathwayHooks>,
) -> Box<dyn EventHandle<EventData<ResponsePayload>>> {
    use aimee_domain::EventHandleExt;
    match pathway {
        Some(hooks) if hooks.on_response_enabled() => base.and(hooks.on_response.clone()),
        _ => Box::new(base),
    }
}

/// Chains pathway turn-end logging onto an existing end handler.
pub fn chain_on_end(
    base: impl EventHandle<EventData<EndPayload>> + 'static,
    pathway: Option<&PathwayHooks>,
) -> Box<dyn EventHandle<EventData<EndPayload>>> {
    use aimee_domain::EventHandleExt;
    match pathway {
        Some(hooks) if hooks.on_end_enabled() => base.and(hooks.on_end.clone()),
        _ => Box::new(base),
    }
}

impl PathwayHooks {
    fn on_response_enabled(&self) -> bool {
        // PathwayLogHook fields are private; always chain — hook no-ops when disabled.
        true
    }

    fn on_end_enabled(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    use super::*;
    use aimee_config::AndaConfig;

    #[test]
    fn test_maybe_pathway_hooks_none_when_disabled() {
        let actual = maybe_pathway_hooks(&AndaConfig::default(), PathBuf::from("/tmp"), "agent");
        assert!(actual.is_none());
    }

    #[test]
    fn test_maybe_pathway_hooks_some_when_enabled() {
        let tmp = TempDir::new().unwrap();
        let anda = AndaConfig {
            enabled: true,
            pathway_dir: Some(tmp.path().join("pathways")),
            eternal_dir: Some(tmp.path().join("eternal")),
            ..AndaConfig::default()
        };
        let actual = maybe_pathway_hooks(&anda, tmp.path().to_path_buf(), "aimee");
        assert!(actual.is_some());
        assert_eq!(anda.enabled, true);
    }

    #[tokio::test]
    async fn test_list_session_pathway_empty() {
        let tmp = TempDir::new().unwrap();
        let anda = AndaConfig {
            enabled: true,
            pathway_dir: Some(tmp.path().join("pathways")),
            eternal_dir: Some(tmp.path().join("eternal")),
            ..AndaConfig::default()
        };
        let conversation_id = aimee_domain::ConversationId::generate();
        let actual = list_session_pathway(Some(&anda), tmp.path().to_path_buf(), &conversation_id)
            .await
            .unwrap();
        assert!(actual.is_empty());
    }
}
