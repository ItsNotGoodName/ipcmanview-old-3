use std::str::FromStr;

use anyhow::Result;
use sqlx::SqlitePool;

use crate::db;
use crate::ipc::{IpcDetail, IpcManager, IpcManagerStore, IpcSoftwareVersion};
use crate::models::{Camera, CameraScanResult, CreateCamera, UpdateCamera};
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

    db::active_scans_clear(&pool).await?;

    Ok(pool)
}

pub async fn setup_store(pool: &SqlitePool, client: dhrpc::HttpClient) -> Result<IpcManagerStore> {
    let store = IpcManagerStore::new();
    for cam in Camera::list(pool).await? {
        let man = cam.new_camera_manager(client.clone());
        store.add(man).await?;
    }

    Ok(store)
}

// -------------------- Camera

impl Camera {
    pub fn new_camera_manager(self, client: dhrpc::HttpClient) -> IpcManager {
        IpcManager::new(
            self.id,
            dhrpc::Client::new(client, self.ip, self.username, self.password),
        )
    }
}

pub async fn camera_create(
    pool: &SqlitePool,
    store: &mut IpcManagerStore,
    client: dhrpc::HttpClient,
    cam: CreateCamera,
) -> Result<i64> {
    let man = cam.create(pool).await?.new_camera_manager(client);
    let id = man.id;
    store.add(man).await?;

    Ok(id)
}

pub async fn camera_delete(pool: &SqlitePool, store: IpcManagerStore, id: i64) -> Result<()> {
    Camera::delete(pool, id).await?;
    store.delete(id).await.ok();

    Ok(())
}

pub async fn camera_update(
    pool: &SqlitePool,
    store: IpcManagerStore,
    cam: UpdateCamera,
) -> Result<()> {
    let id = cam.id;
    let man = store.get(id).await?;
    let mut client = man.client.lock().await;

    let cam = cam.update_and_find(pool).await?;

    // Logout
    client.logout().await.ok();

    // Update store camera from database camera
    client.username = cam.username;
    client.password = cam.password;
    client.ip = cam.ip;
    client.blocked = false;

    Ok(())
}

pub async fn camera_data_refresh(pool: &SqlitePool, man: &IpcManager) -> Result<()> {
    IpcDetail::get(&man).await?.save(pool, man.id).await?;
    IpcSoftwareVersion::get(&man)
        .await?
        .save(pool, man.id)
        .await?;

    Ok(())
}

// -------------------- Scan

async fn scan_range_run(
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

pub async fn scan_task_run(
    pool: &SqlitePool,
    man: &IpcManager,
    scan_task: ScanTask,
) -> Result<CameraScanResult> {
    // Start
    let scan_task_handle = scan_task.start(pool).await?;

    // Run
    let res = scan_range_run(pool, man, &scan_task_handle.range).await;
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
