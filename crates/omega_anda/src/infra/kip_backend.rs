use crate::{AndaResult, KipReceipt};

/// Backend capable of executing KIP commands against a Cognitive Nexus.
#[async_trait::async_trait]
pub trait KipBackend: Send + Sync {
    /// Executes a single KIP command string.
    ///
    /// # Arguments
    /// * `command` - KQL, KML, or META command text
    ///
    /// # Errors
    /// Returns [`crate::AndaError::Kip`] when the backend rejects the command.
    async fn execute_kip(&self, command: &str) -> AndaResult<KipReceipt>;
}

/// No-op KIP backend used when nexus integration is disabled.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopKipBackend;

#[async_trait::async_trait]
impl KipBackend for NoopKipBackend {
    async fn execute_kip(&self, command: &str) -> AndaResult<KipReceipt> {
        Ok(KipReceipt::new(command, true, Some("noop".into())))
    }
}
