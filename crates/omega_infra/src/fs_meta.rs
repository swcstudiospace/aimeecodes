use std::path::Path;

use anyhow::Result;
use omega_app::FileInfoInfra;

pub struct OmegaFileMetaService;
#[async_trait::async_trait]
impl FileInfoInfra for OmegaFileMetaService {
    async fn is_file(&self, path: &Path) -> Result<bool> {
        Ok(omega_fs::OmegaFS::is_file(path))
    }

    async fn is_binary(&self, path: &Path) -> Result<bool> {
        omega_fs::OmegaFS::is_binary_file(path).await
    }

    async fn exists(&self, path: &Path) -> Result<bool> {
        Ok(omega_fs::OmegaFS::exists(path))
    }

    async fn file_size(&self, path: &Path) -> Result<u64> {
        omega_fs::OmegaFS::file_size(path).await
    }
}
