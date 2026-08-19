use std::path::Path;

use omega_app::FileDirectoryInfra;

#[derive(Default)]
pub struct OmegaCreateDirsService;

#[async_trait::async_trait]
impl FileDirectoryInfra for OmegaCreateDirsService {
    async fn create_dirs(&self, path: &Path) -> anyhow::Result<()> {
        Ok(omega_fs::OmegaFS::create_dir_all(path).await?)
    }
}
