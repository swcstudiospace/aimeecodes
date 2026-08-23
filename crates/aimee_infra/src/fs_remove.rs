use std::path::Path;

use aimee_app::FileRemoverInfra;

/// Low-level file remove service
///
/// Provides primitive file deletion operations without snapshot coordination.
/// Snapshot management should be handled at the service layer.
#[derive(Default)]
pub struct AimeeFileRemoveService;

impl AimeeFileRemoveService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FileRemoverInfra for AimeeFileRemoveService {
    async fn remove(&self, path: &Path) -> anyhow::Result<()> {
        Ok(aimee_fs::AimeeFS::remove_file(path).await?)
    }
}
