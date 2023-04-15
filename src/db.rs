use std::sync::Mutex;

use chrono::DateTime;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Acquire, QueryBuilder, Sqlite, SqliteConnection, Transaction};

use anyhow::{Context, Result};

use crate::core;

use crate::rpc::mediafilefind::{self, FindNextFileInfo};
use crate::rpc::{
    self, magicbox,
    rpclogin::{self, Manager},
};

struct Camera {
    ip: String,
    username: String,
    password: String,
}

pub async fn camera_manager_get(
    pool: &mut SqliteConnection,
    id: i64,
    agent: ureq::Agent,
) -> Result<core::Camera> {
    let camera = sqlx::query_as!(
        Camera,
        r#"
        SELECT ip, username, password FROM cameras WHERE id = ?
        "#,
        id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| format!("Could not find camera with id {}", id))?;

    let man = rpclogin::Manager::new(rpc::Client::new(camera.ip, agent))
        .username(camera.username)
        .password(camera.password)
        .unlock();

    Ok(core::Camera {
        id,
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
        INSERT INTO camera_software_version
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
    id: i64,
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
        id,
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
    id: i64,
    data: magicbox::GetSoftwareVersion,
) -> Result<SqliteQueryResult> {
    sqlx::query!(
        r#"
        UPDATE camera_software_version SET 
        build = ?2,
        build_date = ?3,
        security_base_line_version = ?4,
        version = ?5,
        web_version = ?6
        WHERE id = ?1
        "#,
        id,
        data.build,
        data.build_date,
        data.security_base_line_version,
        data.version,
        data.web_version
    )
    .execute(pool)
    .await
    .with_context(|| format!("Could not update software version with id {}", id))
}

#[derive(Debug)]
pub struct CameraScan {
    pub rows_upserted: u64,
    pub rows_deleted: u64,
}

pub async fn camera_scan(
    pool: &mut SqliteConnection,
    cam: &core::Camera,
    start_time: DateTime<chrono::Utc>,
    end_time: DateTime<chrono::Utc>,
) -> Result<CameraScan> {
    let mut man = cam.man.lock().unwrap();
    let mut iter = mediafilefind::find_next_file_info_iterator(
        &mut man.client,
        mediafilefind::Condition::new(start_time, end_time).picture(),
    )?;
    let mut tx = pool.begin().await?;

    let timestamp = chrono::Utc::now();

    let mut rows_upserted: u64 = 0;
    while let Some(files) = iter.next() {
        rows_upserted += camera_scan_files(&mut tx, cam.id, files, &timestamp)
            .await?
            .rows_affected();
    }

    if let Some(err) = iter.error {
        return Err(err).context("Error after scanning files");
    }

    let rows_deleted = sqlx::query!(
        "DELETE FROM camera_files WHERE updated_at < ?1 and camera_id = ?2 and start_time > ?3 and end_time < ?4",
        timestamp,
        cam.id,
        start_time,
        end_time
    )
    .execute(&mut tx)
    .await?.rows_affected();

    tx.commit().await?;

    Ok(CameraScan {
        rows_deleted,
        rows_upserted,
    })
}

// const MAX_FILE_QUERY_RANGE: i32 = 29; // Max number of days in a file query scan range.
// const MIN_FILE_QUERY_RANGE: i32 = 1; // Min number of hours in a file query scan range.
// const FILE_QUERY_VOLATILE_RANGE: i32 = 5; // Number of minutes at end of the file query's endTime that could have files still being written.

async fn camera_scan_files(
    tx: &mut Transaction<'_, Sqlite>,
    id: i64,
    files: Vec<FindNextFileInfo>,
    timestamp: &chrono::DateTime<chrono::Utc>,
) -> Result<SqliteQueryResult> {
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT INTO camera_files (camera_id, file_path, updated_at, start_time, end_time) ",
    );

    qb.push_values(files, |mut b, file| {
        b.push_bind(id)
            .push_bind(file.file_path)
            .push_bind(timestamp)
            .push_bind(file.start_time)
            .push_bind(file.end_time);
    })
    .push("ON CONFLICT (camera_id, file_path) DO UPDATE SET updated_at=excluded.updated_at")
    .build()
    .execute(tx)
    .await
    .with_context(|| format!("Could not upsert files with id {}", id))
}
