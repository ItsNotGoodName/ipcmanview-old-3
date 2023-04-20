use std::str::FromStr;

use sqlx::SqlitePool;

use anyhow::Result;

use crate::core::{CameraDetail, CameraManager, CameraManagerStore, CameraSoftwareVersion};
use crate::db::{self, CameraScanResult};
use crate::models::{Camera, CameraCreate, CameraUpdate, ScanRange, ScanTask};
use crate::rpc;

// -------------------- Setup

pub async fn setup_database(url: &str) -> Result<sqlx::SqlitePool> {
    // Connect
    let options = sqlx::sqlite::SqliteConnectOptions::from_str(url)?.create_if_missing(true);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(options)
        .await?;

    // Migrate
    sqlx::migrate!().run(&pool).await?;

    db::delete_active_scans(&pool).await?;

    Ok(pool)
}

pub async fn setup_camera(
    pool: &SqlitePool,
    store: &mut CameraManagerStore,
    client: reqwest::Client,
) -> Result<()> {
    for cam in Camera::list(pool).await? {
        let man = cam.new_camera_manager(client.clone());
        store.add(man)?;
    }

    Ok(())
}

// -------------------- Camera

impl Camera {
    pub fn new_camera_manager(self, client: reqwest::Client) -> CameraManager {
        CameraManager::new(
            self.id,
            rpc::Client::new(client, self.ip, self.username, self.password),
        )
    }
}

pub async fn camera_create(
    pool: &SqlitePool,
    store: &mut CameraManagerStore,
    cam: CameraCreate,
    client: reqwest::Client,
) -> Result<i64> {
    let man = cam.create(pool).await?.new_camera_manager(client);
    let id = man.id;
    store.add(man)?;

    Ok(id)
}

pub async fn camera_delete(pool: &SqlitePool, store: CameraManagerStore, id: i64) -> Result<()> {
    // Delete camera in database
    Camera::delete(pool, id).await?;
    // Delete manager in store
    store.delete(id).await.ok();

    Ok(())
}

pub async fn camera_update(
    pool: &SqlitePool,
    store: CameraManagerStore,
    cam: CameraUpdate,
) -> Result<()> {
    let id = cam.id;
    let man = store.get(id)?;
    let mut client = man.client.lock().unwrap();
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

pub async fn camera_refresh_all(
    pool: &SqlitePool,
    store: CameraManagerStore,
    id: i64,
) -> Result<()> {
    let man = store.get(id)?;

    CameraDetail::get(&man).await?.save(pool, id).await?;
    CameraSoftwareVersion::get(&man)
        .await?
        .save(pool, id)
        .await?;

    Ok(())
}

// -------------------- Scan

async fn scan_range_run(
    pool: &SqlitePool,
    man: &CameraManager,
    range: &ScanRange,
) -> Result<CameraScanResult> {
    let mut res = CameraScanResult::default();
    for range in range.iter() {
        dbg!(range.start);
        dbg!(range.end);
        res += man.scan_files(pool, range.start, range.end).await?;
        dbg!(&res);
    }

    Ok(res)
}

pub async fn scan_task_run(
    pool: &SqlitePool,
    man: &CameraManager,
    task: ScanTask,
) -> Result<CameraScanResult> {
    let task = task.start(pool).await?;
    let res = scan_range_run(pool, man, &task.range).await;
    if let Err(err) = task.end(pool).await {
        dbg!(err);
    };

    res
}
