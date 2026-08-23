use std::path::Path;
use std::sync::Arc;

use aimee_domain::{Environment, Snapshot, SnapshotRepository};
use anyhow::Result;

pub struct AimeeFileSnapshotService {
    inner: Arc<aimee_snaps::SnapshotService>,
}

impl AimeeFileSnapshotService {
    pub fn new(env: Environment) -> Self {
        Self {
            inner: Arc::new(aimee_snaps::SnapshotService::new(env.snapshot_path())),
        }
    }
}

#[async_trait::async_trait]
impl SnapshotRepository for AimeeFileSnapshotService {
    // Creation
    async fn insert_snapshot(&self, file_path: &Path) -> Result<Snapshot> {
        self.inner.create_snapshot(file_path.to_path_buf()).await
    }

    // Undo
    async fn undo_snapshot(&self, file_path: &Path) -> Result<()> {
        self.inner.undo_snapshot(file_path.to_path_buf()).await
    }
}
