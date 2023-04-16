use std::ops::AddAssign;
use std::sync::Mutex;
use std::time::Instant;

use anyhow::{Context, Result};

use chrono::{DateTime, Utc};

use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Acquire, QueryBuilder, Sqlite, SqliteConnection, Transaction};

use crate::core;
use crate::rpc::mediafilefind::{self, FindNextFileInfo};
use crate::rpc::{
    self, magicbox,
    rpclogin::{self, Manager},
};

pub async fn camera_manager_get(
    pool: &mut SqliteConnection,
    camera_id: i64,
    client: reqwest::Client,
) -> Result<core::Camera> {
    let camera = sqlx::query!(
        r#"
        SELECT ip, username, password FROM cameras WHERE id = ?
        "#,
        camera_id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| format!("Failed to find camera {}", camera_id))?;

    let man = rpclogin::Manager::new(rpc::Client::new(camera.ip, client))
        .username(camera.username)
        .password(camera.password)
        .unblock();

    Ok(core::Camera {
        id: camera_id,
        man: Mutex::new(man),
    })
}

pub async fn camera_add(
    pool: &mut SqliteConnection,
    man: Manager,
) -> Result<core::Camera, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let id = sqlx::query!(
        r#"
        INSERT INTO cameras
        (ip, username, password)
        VALUES
        (?1, ?2, ?3)
        "#,
        man.client.ip,
        man.username,
        man.password,
    )
    .execute(&mut *tx)
    .await?
    .last_insert_rowid();

    sqlx::query!(
        r#"
        INSERT INTO camera_details
        (id)
        VALUES
        (?1)
        "#,
        id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO camera_software_versions
        (id)
        VALUES
        (?1)
        "#,
        id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(core::Camera {
        id,
        man: Mutex::new(man),
    })
}

pub async fn camera_detail_update(
    pool: &mut SqliteConnection,
    camera_id: i64,
    data: core::CameraDetail,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE camera_details SET 
        sn = coalesce(?2, sn),
        device_class = coalesce(?3, device_class),
        device_type = coalesce(?4, device_type),
        hardware_version = coalesce(?5, hardware_version),
        market_area = coalesce(?6, market_area),
        process_info = coalesce(?7, process_info),
        vendor = coalesce(?8, vendor)
        WHERE id = ?1
        "#,
        camera_id,
        data.sn,
        data.device_class,
        data.device_type,
        data.hardware_version,
        data.market_area,
        data.process_info,
        data.vendor
    )
    .execute(pool)
    .await
}

pub async fn camera_software_version_update(
    pool: &mut SqliteConnection,
    camera_id: i64,
    data: magicbox::GetSoftwareVersion,
) -> Result<SqliteQueryResult> {
    sqlx::query!(
        r#"
        UPDATE camera_software_versions SET 
        build = ?2,
        build_date = ?3,
        security_base_line_version = ?4,
        version = ?5,
        web_version = ?6
        WHERE id = ?1
        "#,
        camera_id,
        data.build,
        data.build_date,
        data.security_base_line_version,
        data.version,
        data.web_version
    )
    .execute(pool)
    .await
    .with_context(|| format!("Failed to update software version with id {}", camera_id))
}

#[derive(Default, Debug)]
pub struct CameraScanResult {
    pub upserted: u64,
    pub deleted: u64,
}

impl AddAssign for CameraScanResult {
    fn add_assign(&mut self, rhs: Self) {
        self.upserted += rhs.upserted;
        self.deleted += rhs.deleted;
    }
}

pub async fn camera_scan(
    pool: &mut SqliteConnection,
    cam: &core::Camera,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<CameraScanResult> {
    let mut man = cam.man.lock().unwrap();
    let mut stream = mediafilefind::find_next_file_info_stream(
        &mut man,
        mediafilefind::Condition::new(start_time, end_time).picture(),
    )
    .await?;
    let mut tx = pool.begin().await?;

    let timestamp = Utc::now();

    let mut rows_upserted: u64 = 0;
    while let Some(files) = stream.next().await {
        rows_upserted += camera_scan_files(&mut tx, cam.id, files, &timestamp)
            .await?
            .rows_affected();
    }

    if let Some(err) = stream.error {
        return Err(err).context(format!("Error after scanning files with camera {}", cam.id));
    }

    let rows_deleted = sqlx::query!(
        "DELETE FROM camera_files WHERE updated_at < ?1 and camera_id = ?2 and start_time >= ?3 and end_time <= ?4",
        timestamp,
        cam.id,
        start_time,
        end_time
    )
    .execute(&mut tx)
    .await?.rows_affected();

    tx.commit().await?;

    Ok(CameraScanResult {
        deleted: rows_deleted,
        upserted: rows_upserted,
    })
}

async fn camera_scan_files(
    tx: &mut Transaction<'_, Sqlite>,
    camera_id: i64,
    files: Vec<FindNextFileInfo>,
    timestamp: &chrono::DateTime<Utc>,
) -> Result<SqliteQueryResult> {
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT INTO camera_files (camera_id, file_path, updated_at, start_time, end_time) ",
    );

    qb.push_values(files, |mut b, file| {
        let (start_time, end_time) = file.unique_time();
        b.push_bind(camera_id)
            .push_bind(file.file_path)
            .push_bind(timestamp)
            .push_bind(start_time)
            .push_bind(end_time);
    })
    .push("ON CONFLICT (camera_id, file_path) DO UPDATE SET updated_at=excluded.updated_at")
    .build()
    .execute(tx)
    .await
    .with_context(|| format!("Failed to upsert files with camera {}", camera_id))
}

pub async fn camera_tasks_delete_running(pool: &mut SqliteConnection) -> Result<SqliteQueryResult> {
    sqlx::query!("DELETE FROM camera_running_tasks")
        .execute(pool)
        .await
        .context("Failed to delete running tasks")
}

pub async fn camera_tasks_start(
    pool: &mut SqliteConnection,
    cam: &core::Camera,
) -> Result<Instant> {
    let started_at = Utc::now();

    sqlx::query!(
        "INSERT INTO camera_running_tasks (camera_id, started_at) VALUES(?, ?)",
        cam.id,
        started_at
    )
    .execute(pool)
    .await
    .with_context(|| format!("Failed to create task with camera {}", cam.id))?;

    Ok(Instant::now())
}

struct CameraRunningTask {
    started_at: DateTime<Utc>,
}

pub async fn camera_tasks_end(
    pool: &mut SqliteConnection,
    cam: &core::Camera,
    instant: Instant,
) -> Result<()> {
    let mut tx = pool.begin().await?;

    let running_task = sqlx::query_as_unchecked!(
        CameraRunningTask,
        "SELECT started_at FROM camera_running_tasks WHERE camera_id = ?",
        cam.id
    )
    .fetch_one(&mut tx)
    .await
    .with_context(|| format!("Failed to find running tasks with camera {}", cam.id))?;

    let duration = instant.elapsed().as_millis() as i64;

    sqlx::query!(
        "INSERT INTO camera_past_tasks (camera_id, started_at, duration) VALUES (?, ?, ?)",
        cam.id,
        running_task.started_at,
        duration
    )
    .execute(&mut tx)
    .await
    .with_context(|| format!("Failed to insert into past tasks with camera {}", cam.id))?;

    sqlx::query!(
        "DELETE FROM camera_running_tasks WHERE camera_id = ?",
        cam.id
    )
    .execute(&mut tx)
    .await
    .with_context(|| format!("Failed to delete running task with camera {}", cam.id))?;

    tx.commit().await.with_context(|| {
        format!(
            "Failed to commit end task transaction with camera {}",
            cam.id
        )
    })?;

    Ok(())
}
