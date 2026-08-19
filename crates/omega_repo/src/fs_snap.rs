use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use omega_domain::{Environment, Snapshot, SnapshotRepository};

pub struct OmegaFileSnapshotService {
    inner: Arc<omega_snaps::SnapshotService>,
}

impl OmegaFileSnapshotService {
    pub fn new(env: Environment) -> Self {
        Self {
            inner: Arc::new(omega_snaps::SnapshotService::new(env.snapshot_path())),
        }
    }
}

#[async_trait::async_trait]
impl SnapshotRepository for OmegaFileSnapshotService {
    // Creation
    async fn insert_snapshot(&self, file_path: &Path) -> Result<Snapshot> {
        self.inner.create_snapshot(file_path.to_path_buf()).await
    }

    // Undo
    async fn undo_snapshot(&self, file_path: &Path) -> Result<()> {
        self.inner.undo_snapshot(file_path.to_path_buf()).await
    }
}
