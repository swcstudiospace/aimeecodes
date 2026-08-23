use std::path::Path;

use aimee_app::FileInfoInfra;
use anyhow::Result;

pub struct AimeeFileMetaService;
#[async_trait::async_trait]
impl FileInfoInfra for AimeeFileMetaService {
    async fn is_file(&self, path: &Path) -> Result<bool> {
        Ok(aimee_fs::AimeeFS::is_file(path))
    }

    async fn is_binary(&self, path: &Path) -> Result<bool> {
        aimee_fs::AimeeFS::is_binary_file(path).await
    }

    async fn exists(&self, path: &Path) -> Result<bool> {
        Ok(aimee_fs::AimeeFS::exists(path))
    }

    async fn file_size(&self, path: &Path) -> Result<u64> {
        aimee_fs::AimeeFS::file_size(path).await
    }
}
