use std::path::Path;

use aimee_app::FileDirectoryInfra;

#[derive(Default)]
pub struct AimeeCreateDirsService;

#[async_trait::async_trait]
impl FileDirectoryInfra for AimeeCreateDirsService {
    async fn create_dirs(&self, path: &Path) -> anyhow::Result<()> {
        Ok(aimee_fs::AimeeFS::create_dir_all(path).await?)
    }
}
