use std::str::FromStr;

use anyhow::Result;
use sqlx::SqlitePool;

use crate::ipc::{IpcDetail, IpcManager, IpcManagerStore, IpcSoftwareVersion};
use crate::models::{Camera, CameraScanResult, CreateCamera, ScanActive, UpdateCamera};
use crate::scan::{ScanRange, ScanTask};

// -------------------- Setup

pub async fn setup_database(url: &str) -> Result<sqlx::SqlitePool> {
    // Connect
    let options = sqlx::sqlite::SqliteConnectOptions::from_str(url)?.create_if_missing(true);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(options)
        .await?;

    // Migrate
    sqlx::migrate!().run(&pool).await?;

    ScanActive::clear(&pool).await?;

    Ok(pool)
}

// -------------------- Camera

impl CreateCamera<'_> {
    pub async fn create(self, pool: &SqlitePool, store: &IpcManagerStore) -> Result<i64> {
        let id = self.create_db(pool).await?;
        store.refresh(pool, id).await?;
        store.get(id).await?.data_refresh(pool).await.ok();
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

impl ScanTask {
    async fn range_run(
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

    pub async fn run(self, pool: &SqlitePool, man: &IpcManager) -> Result<CameraScanResult> {
        // Start
        let scan_task_handle = self.start(pool).await?;

        // Run
        let res = Self::range_run(pool, man, &scan_task_handle.range).await;
        let scan_task_handle = match res {
            Ok(_) => scan_task_handle,
            Err(ref err) => scan_task_handle.with_error(err.to_string()),
        };

        // End
        if let Err(err) = scan_task_handle.end(pool).await {
            dbg!(err);
        };

        res
    }
}
