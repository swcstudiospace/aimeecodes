use std::path::Path;

use omega_app::FileRemoverInfra;

/// Low-level file remove service
///
/// Provides primitive file deletion operations without snapshot coordination.
/// Snapshot management should be handled at the service layer.
#[derive(Default)]
pub struct OmegaFileRemoveService;

impl OmegaFileRemoveService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FileRemoverInfra for OmegaFileRemoveService {
    async fn remove(&self, path: &Path) -> anyhow::Result<()> {
        Ok(omega_fs::OmegaFS::remove_file(path).await?)
    }
}
