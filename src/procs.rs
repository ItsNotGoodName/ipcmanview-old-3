use anyhow::Result;
use sqlx::SqlitePool;

use crate::ipc::{IpcDetail, IpcManager, IpcManagerStore, IpcSoftware};
use crate::models::{
    Camera, CameraFile, CameraScanResult, CreateCamera, CursorCameraFile, QueryCameraFile,
    QueryCameraFileResult, UpdateCamera,
};
use crate::scan::{Scan, ScanHandle, ScanKindPending};

// -------------------- Camera

impl CreateCamera<'_> {
    pub async fn create(self, pool: &SqlitePool, store: &IpcManagerStore) -> Result<i64> {
        let id = self.create_db(pool).await?;
        store.refresh(pool, id).await?;
        store.get(id).await?.data_refresh(pool).await.ok();
        Scan::queue(pool, store, id, ScanKindPending::Full).await?;
        Ok(id)
    }
}

impl UpdateCamera<'_> {
    pub async fn update(self, pool: &SqlitePool, store: &IpcManagerStore) -> Result<()> {
        let id = self.id;
        self.update_db(pool).await?;
        store.refresh(pool, id).await
    }
}

impl Camera {
    pub async fn delete(pool: &SqlitePool, store: &IpcManagerStore, id: i64) -> Result<()> {
        Self::delete_db(pool, id).await?;
        store.refresh(pool, id).await
    }
}

impl IpcManager {
    pub async fn data_refresh(&self, pool: &SqlitePool) -> Result<()> {
        IpcDetail::get(&self).await?.save(pool, self.id).await?;
        IpcSoftware::get(&self).await?.save(pool, self.id).await?;

        Ok(())
    }
}

impl CameraFile {
    pub async fn query(
        pool: &SqlitePool,
        store: &IpcManagerStore,
        query: QueryCameraFile<'_>,
    ) -> Result<QueryCameraFileResult> {
        if let CursorCameraFile::None = query.cursor {
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
        Scan::queue_db(pool, camera_id, kind).await?;
        Scan::run_pending(pool, store).await;
        Ok(())
    }

    pub async fn queue_all(
        pool: &SqlitePool,
        store: &IpcManagerStore,
        kind: ScanKindPending,
    ) -> Result<()> {
        Self::queue_all_db(pool, kind).await?;
        Scan::run_pending(pool, store).await;
        Ok(())
    }

    pub async fn run_pending(pool: &SqlitePool, store: &IpcManagerStore) {
        // Get a pending scan
        let first_handle = if let Ok(Some(s)) = ScanHandle::next(pool).await {
            s
        } else {
            return;
        };

        // Get rest of the pending scans
        let mut handles = vec![first_handle];
        loop {
            match ScanHandle::next(&pool).await {
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
                match handle.run(&pool, &store).await {
                    Ok(res) => {
                        dbg!(res);
                    }
                    Err(err) => {
                        dbg!(err);
                    }
                }
                // Check for more scans and run them or exit
                loop {
                    match ScanHandle::next(&pool).await {
                        Ok(Some(handle)) => match handle.run(&pool, &store).await {
                            Ok(res) => {
                                dbg!(res);
                            }
                            Err(err) => {
                                dbg!(err);
                            }
                        },
                        Ok(None) => return,
                        Err(err) => {
                            dbg!(err);
                            return;
                        }
                    }
                }
            });
        }
    }
}

impl ScanHandle {
    async fn run_(&self, pool: &SqlitePool, man: &IpcManager) -> Result<CameraScanResult> {
        let mut res = CameraScanResult::default();
        for (range, percent) in self.range.iter() {
            res += man.scan_files(pool, range.start, range.end).await?;
            self.percent(pool, percent).await?
        }

        Ok(res)
    }

    async fn run(self, pool: &SqlitePool, store: &IpcManagerStore) -> Result<CameraScanResult> {
        let man = store.get(self.camera_id).await?;

        // Run
        let res = self.run_(pool, &man).await;

        let handle = match res {
            Ok(_) => self,
            Err(ref err) => self.with_error(err.to_string()),
        };

        // End
        if let Err(err) = handle.end(pool).await {
            dbg!(err);
        };

        res
    }
}
