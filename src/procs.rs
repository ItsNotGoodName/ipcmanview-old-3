use anyhow::Result;
use sqlx::SqlitePool;

use crate::ipc::{IpcDetail, IpcManager, IpcManagerStore, IpcSoftwareVersion};
use crate::models::{Camera, CameraScanResult, CreateCamera, UpdateCamera};
use crate::scan::{Scan, ScanHandle, ScanKindPending, ScanRange};

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
        IpcSoftwareVersion::get(&self)
            .await?
            .save(pool, self.id)
            .await?;

        Ok(())
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

    pub async fn run_pending(pool: &SqlitePool, store: &IpcManagerStore) {
        let pool = pool.clone();
        let store = store.clone();
        tokio::spawn(async move {
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

impl ScanHandle {
    async fn scan_range(
        pool: &SqlitePool,
        man: &IpcManager,
        range: &ScanRange,
    ) -> Result<CameraScanResult> {
        let mut res = CameraScanResult::default();
        for range in range.iter() {
            res += man.scan_files(pool, range.start, range.end).await?;
        }

        Ok(res)
    }

    async fn run(self, pool: &SqlitePool, store: &IpcManagerStore) -> Result<CameraScanResult> {
        let man = store.get(self.camera_id).await?;

        // Run
        let res = Self::scan_range(pool, &man, &self.range).await;
        let handle = match res {
            Ok(_) => self,
            Err(ref err) => self.with_error(err.to_string()),
        };

        // End
        let camera_id = handle.camera_id;
        if let Err(err) = handle.end(pool).await {
            panic!("Failed to end active task with camera {camera_id} and error {err}",);
        };

        res
    }
}
