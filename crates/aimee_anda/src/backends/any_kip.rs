use crate::{AndaError, AndaResult, KipBackend, KipReceipt, NexusHttpBackend, NoopKipBackend};

/// Type-erased KIP backend selection for app wiring.
#[derive(Debug, Clone)]
pub enum AnyKipBackend {
    /// Discard KIP side effects.
    Noop(NoopKipBackend),
    /// HTTP Cognitive Nexus client.
    Http(NexusHttpBackend),
}

impl AnyKipBackend {
    /// Builds a backend from an optional nexus base URL.
    pub fn from_nexus_url(nexus_url: Option<&str>) -> Self {
        match nexus_url {
            Some(url) if !url.trim().is_empty() => Self::Http(NexusHttpBackend::new(url)),
            _ => Self::Noop(NoopKipBackend),
        }
    }
}

#[async_trait::async_trait]
impl KipBackend for AnyKipBackend {
    async fn execute_kip(&self, command: &str) -> AndaResult<KipReceipt> {
        match self {
            Self::Noop(backend) => backend.execute_kip(command).await,
            Self::Http(backend) => backend.execute_kip(command).await,
        }
    }
}

/// Helper that never fails execute_kip mapping.
impl AnyKipBackend {
    /// Returns true when this backend talks to a real nexus.
    pub fn is_remote(&self) -> bool {
        matches!(self, Self::Http(_))
    }
}

/// Maps unexpected backend construction errors into [`AndaError`].
pub fn anda_other(err: impl Into<anyhow::Error>) -> AndaError {
    AndaError::Other(err.into())
}
