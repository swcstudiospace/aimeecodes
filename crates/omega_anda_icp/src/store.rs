use std::path::PathBuf;
use std::sync::Arc;

use omega_anda::{EternalMode, EternalStore, NoopEternalStore};

use crate::{IcpError, IcpResult, LocalReceiptEternalStore};

/// Configuration for constructing an eternal store backend.
#[derive(Debug, Clone)]
pub struct EternalStoreConfig {
    pub mode: EternalMode,
    /// Root directory for local receipts (required for [`EternalMode::Local`]).
    pub local_root: Option<PathBuf>,
    /// IC-OSS endpoint (future).
    pub ic_oss_endpoint: Option<String>,
    /// Canister id (future).
    pub canister_id: Option<String>,
    /// S3 bucket (future).
    pub s3_bucket: Option<String>,
}

impl Default for EternalStoreConfig {
    fn default() -> Self {
        Self {
            mode: EternalMode::Local,
            local_root: None,
            ic_oss_endpoint: None,
            canister_id: None,
            s3_bucket: None,
        }
    }
}

impl EternalStoreConfig {
    /// Local receipt mode under `root`.
    pub fn local(root: impl Into<PathBuf>) -> Self {
        Self {
            mode: EternalMode::Local,
            local_root: Some(root.into()),
            ..Default::default()
        }
    }
}

/// Builds a boxed eternal store from config.
///
/// Non-local modes currently return [`IcpError::NotConfigured`] until ICP
/// clients are wired.
pub fn build_eternal_store(
    config: EternalStoreConfig,
) -> IcpResult<Arc<dyn EternalStore>> {
    match config.mode {
        EternalMode::Local => {
            let root = config.local_root.ok_or_else(|| IcpError::NotConfigured {
                mode: "local".into(),
                detail: "local_root is required".into(),
            })?;
            Ok(Arc::new(LocalReceiptEternalStore::new(root)))
        }
        EternalMode::IcOss => Err(IcpError::NotConfigured {
            mode: "ic_oss".into(),
            detail: config
                .ic_oss_endpoint
                .map(|e| format!("endpoint={e} set but client not enabled; enable feature ic-oss"))
                .unwrap_or_else(|| "set ic_oss_endpoint and enable feature ic-oss".into()),
        }),
        EternalMode::Canister => Err(IcpError::NotConfigured {
            mode: "canister".into(),
            detail: config
                .canister_id
                .map(|id| format!("canister_id={id} set but client not enabled"))
                .unwrap_or_else(|| "set canister_id when canister client is available".into()),
        }),
        EternalMode::S3 => Err(IcpError::NotConfigured {
            mode: "s3".into(),
            detail: config
                .s3_bucket
                .map(|b| format!("bucket={b} set but client not enabled"))
                .unwrap_or_else(|| "set s3_bucket when S3 client is available".into()),
        }),
    }
}

/// Returns a no-op eternal store (disabled durability).
pub fn noop_eternal_store() -> Arc<dyn EternalStore> {
    Arc::new(NoopEternalStore)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_build_local_store() {
        let tmp = TempDir::new().unwrap();
        let store = build_eternal_store(EternalStoreConfig::local(tmp.path())).unwrap();
        // trait object constructed
        let _ = store;
    }

    #[test]
    fn test_build_ic_oss_not_configured() {
        let actual = build_eternal_store(EternalStoreConfig {
            mode: EternalMode::IcOss,
            ..Default::default()
        });
        assert!(actual.is_err());
        let err = actual.err().unwrap().to_string();
        assert_eq!(err.contains("ic_oss"), true);
    }
}
