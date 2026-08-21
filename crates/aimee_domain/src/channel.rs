use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// Messaging / gateway surfaces Hermes supports that Aimee Codes will host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ChannelKind {
    /// Interactive CLI / TUI.
    Cli,
    /// Progressive web app.
    Pwa,
    /// Telegram bot / group.
    Telegram,
    /// Discord guild.
    Discord,
    /// Slack workspace.
    Slack,
    /// WhatsApp.
    Whatsapp,
    /// Signal.
    Signal,
    /// Matrix.
    Matrix,
    /// Microsoft Teams.
    Teams,
    /// Email ingress.
    Email,
}

impl ChannelKind {
    /// Parses a channel name (`telegram`, `discord`, …).
    pub fn parse(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "cli" | "tui" => Some(Self::Cli),
            "pwa" | "web" => Some(Self::Pwa),
            "telegram" | "tg" => Some(Self::Telegram),
            "discord" => Some(Self::Discord),
            "slack" => Some(Self::Slack),
            "whatsapp" => Some(Self::Whatsapp),
            "signal" => Some(Self::Signal),
            "matrix" => Some(Self::Matrix),
            "teams" | "msteams" => Some(Self::Teams),
            "email" | "mail" => Some(Self::Email),
            _ => None,
        }
    }

    /// Canonical slug used in config and slash commands.
    pub fn slug(self) -> &'static str {
        match self {
            Self::Cli => "cli",
            Self::Pwa => "pwa",
            Self::Telegram => "telegram",
            Self::Discord => "discord",
            Self::Slack => "slack",
            Self::Whatsapp => "whatsapp",
            Self::Signal => "signal",
            Self::Matrix => "matrix",
            Self::Teams => "teams",
            Self::Email => "email",
        }
    }
}

/// Configured endpoint for a messaging channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Setters)]
#[setters(strip_option, into)]
pub struct ChannelEndpoint {
    /// Channel kind.
    pub kind: ChannelKind,
    /// Address / chat id / webhook URL (never a secret token).
    pub address: String,
    /// Whether the gateway should deliver here.
    pub enabled: bool,
}

impl ChannelEndpoint {
    /// Creates a disabled endpoint that still records the address.
    pub fn new(kind: ChannelKind, address: impl Into<String>) -> Self {
        Self { kind, address: address.into(), enabled: false }
    }

    /// Renders a one-line status.
    pub fn render(&self) -> String {
        let flag = if self.enabled { "on" } else { "off" };
        format!("{} {flag} {}", self.kind.slug(), self.address)
    }
}

/// File-backed list of channel endpoints (addresses only, never tokens).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelStore {
    /// Persistence path.
    pub path: std::path::PathBuf,
    /// Configured endpoints.
    pub items: Vec<ChannelEndpoint>,
}

impl ChannelStore {
    /// Loads endpoints from `path`, or starts empty if the file is missing.
    pub fn load(path: std::path::PathBuf) -> Self {
        let items = std::fs::read_to_string(&path)
            .ok()
            .and_then(|raw| serde_json::from_str(&raw).ok())
            .unwrap_or_default();
        Self { path, items }
    }

    /// Replaces any existing endpoint of the same kind and persists.
    pub fn upsert(&mut self, endpoint: ChannelEndpoint) -> std::io::Result<()> {
        self.items.retain(|item| item.kind != endpoint.kind);
        self.items.push(endpoint);
        self.persist()
    }

    /// First enabled endpoint of `kind`.
    pub fn first_enabled(&self, kind: ChannelKind) -> Option<&ChannelEndpoint> {
        self.items
            .iter()
            .find(|item| item.kind == kind && item.enabled)
    }

    /// Writes the store as pretty JSON.
    pub fn persist(&self) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.path, serde_json::to_string_pretty(&self.items)?)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_channel_parse_aliases() {
        let actual = (
            ChannelKind::parse("tg"),
            ChannelKind::parse("web"),
            ChannelKind::parse("nope"),
        );
        let expected = (Some(ChannelKind::Telegram), Some(ChannelKind::Pwa), None);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_channel_endpoint_render() {
        let fixture = ChannelEndpoint::new(ChannelKind::Telegram, "-1004338629579").enabled(true);
        let actual = fixture.render();
        let expected = "telegram on -1004338629579".to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_channel_store_upsert_round_trip() {
        let path = std::env::temp_dir().join(format!(
            "aimee-channels-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        let mut store = ChannelStore::load(path.clone());
        store
            .upsert(ChannelEndpoint::new(ChannelKind::Telegram, "-1001").enabled(true))
            .unwrap();
        let reloaded = ChannelStore::load(path.clone());
        let actual = reloaded
            .first_enabled(ChannelKind::Telegram)
            .map(|e| e.address.as_str());
        let expected = Some("-1001");
        let _ = std::fs::remove_file(&path);
        assert_eq!(actual, expected);
    }
}
