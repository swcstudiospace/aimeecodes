use std::path::PathBuf;

use derive_setters::Setters;
use fake::Dummy;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Durability backend for eternal pathway export (mirrors `aimee_anda::EternalMode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema, Dummy)]
#[serde(rename_all = "snake_case")]
pub enum AndaEternalMode {
    /// Write content-addressed local receipts (offline-capable).
    #[default]
    Local,
    /// IC-OSS object storage on the Internet Computer.
    IcOss,
    /// Dedicated KIP/pathway canister.
    Canister,
    /// S3-compatible object storage.
    S3,
}

/// Anda / KIP eternal session pathway configuration.
///
/// When [`enabled`](Self::enabled) is true, Aimee appends hash-chained conversation
/// checkpoints on agent output so chats can be rolled back independently of files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Dummy, Setters)]
#[serde(rename_all = "snake_case")]
#[setters(strip_option, into)]
pub struct AndaConfig {
    /// Master switch for session pathway logging.
    #[serde(default)]
    pub enabled: bool,

    /// Directory for pathway metadata and checkpoints.
    /// Defaults to `{aimee_home}/pathways` when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_dir: Option<PathBuf>,

    /// Optional Cognitive Nexus base URL (e.g. `http://127.0.0.1:8091`).
    /// When set and [`kip_enabled`](Self::kip_enabled) is true, checkpoints are
    /// also recorded via KIP `execute_kip`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nexus_url: Option<String>,

    /// Execute KIP UPSERT side effects for each checkpoint.
    #[serde(default)]
    pub kip_enabled: bool,

    /// Export checkpoints to eternal storage (local receipts and/or ICP).
    #[serde(default = "default_true")]
    pub eternal_enabled: bool,

    /// Eternal durability backend mode.
    #[serde(default)]
    pub eternal_mode: AndaEternalMode,

    /// Root directory for local eternal capsules/receipts.
    /// Defaults to `{aimee_home}/pathways/eternal` when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eternal_dir: Option<PathBuf>,

    /// Label prefix used in eternal receipts.
    #[serde(default = "default_label_prefix")]
    pub eternal_label_prefix: String,

    /// Log a checkpoint after each LLM response.
    #[serde(default = "default_true")]
    pub log_responses: bool,

    /// Log a checkpoint when a turn ends.
    #[serde(default = "default_true")]
    pub log_turn_end: bool,

    /// When true, pathway log failures fail the agent turn; otherwise warn and continue.
    #[serde(default)]
    pub hard_fail: bool,
}

fn default_true() -> bool {
    true
}

fn default_label_prefix() -> String {
    "aimee".into()
}

impl Default for AndaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            pathway_dir: None,
            nexus_url: None,
            kip_enabled: false,
            eternal_enabled: true,
            eternal_mode: AndaEternalMode::Local,
            eternal_dir: None,
            eternal_label_prefix: default_label_prefix(),
            log_responses: true,
            log_turn_end: true,
            hard_fail: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_anda_config_defaults_disabled() {
        let actual = AndaConfig::default();
        assert!(!actual.enabled);
        assert!(!actual.kip_enabled);
        assert!(actual.eternal_enabled);
        assert_eq!(actual.eternal_mode, AndaEternalMode::Local);
        assert_eq!(actual.eternal_label_prefix, "aimee");
    }

    #[test]
    fn test_anda_config_toml_round_trip() {
        let fixture = r#"
enabled = true
kip_enabled = true
nexus_url = "http://127.0.0.1:8091"
eternal_mode = "local"
eternal_label_prefix = "aimee-dev"
hard_fail = false
"#;
        let actual: AndaConfig = toml_edit::de::from_str(fixture).unwrap();
        assert!(actual.enabled);
        assert!(actual.kip_enabled);
        assert_eq!(actual.nexus_url.as_deref(), Some("http://127.0.0.1:8091"));
        assert_eq!(actual.eternal_label_prefix, "aimee-dev");
    }
}
