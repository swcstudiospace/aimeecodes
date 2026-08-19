use chrono::{DateTime, Utc};
use derive_more::Display;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// Durability backend mode for eternal pathway export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Default)]
#[serde(rename_all = "snake_case")]
pub enum EternalMode {
    /// Write a local content-addressed receipt (always available offline).
    #[default]
    Local,
    /// Export via IC-OSS object storage on the Internet Computer.
    IcOss,
    /// Export via a dedicated KIP/pathway canister.
    Canister,
    /// Export via S3-compatible object storage.
    S3,
}

/// Receipt proving a checkpoint (or capsule) was exported to eternal storage.
#[derive(Debug, Clone, Serialize, Deserialize, Setters, PartialEq, Eq)]
#[setters(into, strip_option)]
pub struct EternalReceipt {
    pub mode: EternalMode,
    pub label: String,
    pub content_hash: String,
    /// Backend-specific location (file path, object key, canister id + path, etc.).
    pub location: String,
    pub created_at: DateTime<Utc>,
    pub ok: bool,
    pub detail: Option<String>,
}

impl EternalReceipt {
    /// Creates a successful receipt.
    pub fn ok(
        mode: EternalMode,
        label: impl Into<String>,
        content_hash: impl Into<String>,
        location: impl Into<String>,
    ) -> Self {
        Self {
            mode,
            label: label.into(),
            content_hash: content_hash.into(),
            location: location.into(),
            created_at: Utc::now(),
            ok: true,
            detail: None,
        }
    }

    /// Creates a failed receipt.
    pub fn failed(
        mode: EternalMode,
        label: impl Into<String>,
        content_hash: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            mode,
            label: label.into(),
            content_hash: content_hash.into(),
            location: String::new(),
            created_at: Utc::now(),
            ok: false,
            detail: Some(detail.into()),
        }
    }
}

/// Receipt from executing a KIP command related to a checkpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Setters, PartialEq, Eq)]
#[setters(into, strip_option)]
pub struct KipReceipt {
    pub command_digest: String,
    pub ok: bool,
    pub response_summary: Option<String>,
    pub executed_at: DateTime<Utc>,
}

impl KipReceipt {
    /// Builds a receipt from a command string and success flag.
    pub fn new(command: &str, ok: bool, response_summary: Option<String>) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(command.as_bytes());
        Self {
            command_digest: hex::encode(hasher.finalize()),
            ok,
            response_summary,
            executed_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_eternal_receipt_ok() {
        let actual = EternalReceipt::ok(
            EternalMode::Local,
            "omega-test",
            "abc",
            "/tmp/receipt.json",
        );
        assert!(actual.ok);
        assert_eq!(actual.mode, EternalMode::Local);
        assert_eq!(actual.content_hash, "abc");
    }

    #[test]
    fn test_kip_receipt_digest_is_hex_sha256() {
        let actual = KipReceipt::new("DESCRIBE PRIMER", true, Some("ok".into()));
        assert!(actual.ok);
        assert_eq!(actual.command_digest.len(), 64);
    }
}
