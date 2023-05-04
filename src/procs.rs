use anyhow::Result;
use sqlx::SqlitePool;

use crate::ipc::{IpcDetail, IpcLicenses, IpcManager, IpcManagerStore, IpcSoftware};
use crate::models::{
    Camera, CameraFile, CameraScanResult, CreateCamera, QueryCameraFile, QueryCameraFileCursor,
    QueryCameraFileResult, ScanCompleted, UpdateCamera,
};
use crate::scan::{Scan, ScanActor, ScanKindPending};

// -------------------- Camera

impl CreateCamera {
    pub async fn create(self, pool: &SqlitePool, store: &IpcManagerStore) -> Result<i64> {
        // Create in database
        let id = self.create_db(pool).await?;
        // Refresh in store
        store.refresh(pool, id).await?;
        // Get from store and refresh in database
        store.get(id).await?.refresh(pool).await.ok();
        // Queue a full scan
        Scan::queue(pool, store, id, ScanKindPending::Full).await?;

        Ok(id)
    }
}

impl UpdateCamera {
    pub async fn update(self, pool: &SqlitePool, store: &IpcManagerStore) -> Result<()> {
        let id = self.id;
        self.update_db(pool).await?;
        // Refresh in store
        store.refresh(pool, id).await?;
        // Get from store and refresh in database
        store.get(id).await?.refresh(pool).await.ok();

        Ok(())
    }
}

impl Camera {
    pub async fn delete(pool: &SqlitePool, store: &IpcManagerStore, id: i64) -> Result<()> {
        Self::delete_db(pool, id).await?;
        // Refresh in store
        store.refresh(pool, id).await?;

        Ok(())
    }
}

impl IpcManager {
    pub async fn refresh(&self, pool: &SqlitePool) -> Result<()> {
        IpcDetail::get(&self).await?.save(pool, self.id).await?;
        IpcSoftware::get(&self).await?.save(pool, self.id).await?;
        IpcLicenses::get(&self).await?.save(pool, self.id).await?;
        Camera::update_refreshed_at(pool, self.id).await?;

        Ok(())
    }
}

impl CameraFile {
    pub async fn query(
        pool: &SqlitePool,
        store: &IpcManagerStore,
        query: QueryCameraFile<'_>,
    ) -> Result<QueryCameraFileResult> {
        if let QueryCameraFileCursor::None = query.cursor {
            Scan::queue_all(pool, store, ScanKindPending::Cursor).await?;
        }

        Self::query_db(pool, query).await
    }
}

// -------------------- Scan

impl Scan {
    pub async fn queue(
        pool: &SqlitePool,
        store: &IpcManagerStore,
        camera_id: i64,
        kind: ScanKindPending,
    ) -> Result<()> {
        Self::queue_db(pool, kind, camera_id).await?;
        Self::run_pending(pool, store).await;
        Ok(())
    }

    pub async fn queue_all(
        pool: &SqlitePool,
        store: &IpcManagerStore,
        kind: ScanKindPending,
    ) -> Result<()> {
        Self::queue_all_db(pool, kind).await?;
        Self::run_pending(pool, store).await;
        Ok(())
    }

    // TODO: return database access errors
    pub async fn run_pending(pool: &SqlitePool, store: &IpcManagerStore) {
        // Get a pending scan
        let first_handle = if let Ok(Some(s)) = ScanActor::next(pool).await {
            s
        } else {
            return;
        };

        // Get rest of the pending scans
        let mut handles = vec![first_handle];
        loop {
            match ScanActor::next(&pool).await {
                Ok(Some(handle)) => handles.push(handle),
                Ok(None) | Err(_) => break,
            }
        }

        // Start worker for each scan
        for handle in handles {
            let pool = pool.clone();
            let store = store.clone();
            tokio::spawn(async move {
                // Run pending scan
                if let Err(err) = handle.run(&pool, &store).await {
                    tracing::error!("{err:?}");
                }
                // Check for more scans and run them or exit
                loop {
                    match ScanActor::next(&pool).await {
                        Ok(Some(handle)) => {
                            if let Err(err) = handle.run(&pool, &store).await {
                                tracing::error!("{err:?}");
                            }
                        }
                        Ok(None) => return,
                        Err(err) => {
                            tracing::error!("{err:?}");
                            return;
                        }
                    }
                }
            });
        }
    }
}

impl ScanCompleted {
    pub async fn retry(pool: &SqlitePool, store: &IpcManagerStore, id: i64) -> Result<()> {
        Self::retry_db(pool, id).await?;
        Scan::run_pending(pool, store).await;
        Ok(())
    }
}

impl ScanActor {
    async fn runner(&self, pool: &SqlitePool, man: &IpcManager) -> Result<()> {
        let mut res = CameraScanResult::default();
        for (range, percent) in self.range.iter() {
            res += man.scan_files(pool, range.start, range.end).await?;
            self.update_status(
                pool,
                range.start,
                percent,
                res.upserted as i64,
                res.deleted as i64,
            )
            .await?
        }

        Ok(())
    }

    async fn run(mut self, pool: &SqlitePool, store: &IpcManagerStore) -> Result<()> {
        // Get manager
        let man = store.get(self.camera_id).await?;

        // Run scan
        let res = self.runner(pool, &man).await;
        if let Err(ref err) = res {
            self.error = format!("{:?}", err)
        }

        // End scan
        self.end(pool).await?;

        res
    }
}
