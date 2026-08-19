use thiserror::Error;

/// Errors from eternal / ICP pathway export.
#[derive(Debug, Error)]
pub enum IcpError {
    /// Requested durability mode is not configured yet.
    #[error("eternal mode {mode} is not configured: {detail}")]
    NotConfigured { mode: String, detail: String },

    /// I/O failure while writing a local receipt.
    #[error("local eternal receipt i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization failure.
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Other failure.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Result alias for ICP crate operations.
pub type IcpResult<T> = std::result::Result<T, IcpError>;
