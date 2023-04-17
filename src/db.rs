use std::ops::AddAssign;
use std::time::Instant;

use anyhow::{Context, Result};

use chrono::{DateTime, Utc};

use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Acquire, QueryBuilder, Sqlite, SqliteConnection, Transaction};

use crate::core::{
    CameraDetail, CameraFileStream, CameraManager, CameraSoftwareVersion, CameraState,
};
use crate::rpc::mediafilefind::{self, FindNextFileInfo};
use crate::rpc::{self, rpclogin};

pub async fn camera_manager_get(
    pool: &mut SqliteConnection,
    camera_id: i64,
    client: reqwest::Client,
) -> Result<CameraManager> {
    let camera = sqlx::query!(
        r#"
        SELECT ip, username, password FROM cameras WHERE id = ?
        "#,
        camera_id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| format!("Failed to find camera {}", camera_id))?;

    let state = CameraState {
        user: rpclogin::User::new()
            .username(camera.username)
            .password(camera.password)
            .unblock(),
        client: rpc::Client::new(camera.ip, client),
    };

    Ok(CameraManager::new(camera_id, state))
}

impl CameraState {
    pub async fn create(self, pool: &mut SqliteConnection) -> Result<CameraManager> {
        let mut tx = pool.begin().await?;

        let camera_id = sqlx::query!(
            r#"
        INSERT INTO cameras
        (ip, username, password)
        VALUES
        (?1, ?2, ?3)
        "#,
            self.client.ip,
            self.user.username,
            self.user.password,
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
            camera_id
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
            camera_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(CameraManager::new(camera_id, self))
    }
}

impl CameraDetail {
    pub async fn save(&self, pool: &mut SqliteConnection, camera_id: i64) -> Result<()> {
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
            self.sn,
            self.device_class,
            self.device_type,
            self.hardware_version,
            self.market_area,
            self.process_info,
            self.vendor
        )
        .execute(pool)
        .await
        .with_context(|| format!("Failed to update detail with camera {}", camera_id))?;
        Ok(())
    }
}

impl CameraSoftwareVersion {
    pub async fn save(&self, pool: &mut SqliteConnection, camera_id: i64) -> Result<()> {
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
            self.build,
            self.build_date,
            self.security_base_line_version,
            self.version,
            self.web_version
        )
        .execute(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to update software version with camera {}",
                camera_id
            )
        })
        .ok();
        Ok(())
    }
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

impl CameraManager {
    pub async fn scan(
        &self,
        pool: &mut SqliteConnection,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<CameraScanResult> {
        let mut stream = CameraFileStream::new(
            self,
            mediafilefind::Condition::new(start_time, end_time).picture(),
        )
        .await
        .with_context(|| format!("Failed to create file info stream with camera {}", self.id))?;
        let mut tx = pool.begin().await?;

        let timestamp = Utc::now();

        let mut rows_upserted: u64 = 0;
        while let Some(files) = stream.next().await {
            rows_upserted += camera_scan_files(&mut tx, self.id, files, &timestamp)
                .await?
                .rows_affected();
        }

        if let Some(err) = stream.error {
            return Err(err).context(format!(
                "Error after scanning files with camera {}",
                self.id
            ));
        }

        let rows_deleted = sqlx::query!(
        "DELETE FROM camera_files WHERE updated_at < ?1 and camera_id = ?2 and start_time >= ?3 and end_time <= ?4",
        timestamp,
        self.id,
        start_time,
        end_time
    )
    .execute(&mut tx)
    .await.with_context(||format!("Failed to delete stale files with camera {}", self.id))?.rows_affected();

        tx.commit().await.with_context(|| {
            format!(
                "Failed to commit file info transaction with camera {}",
                self.id
            )
        })?;

        Ok(CameraScanResult {
            deleted: rows_deleted,
            upserted: rows_upserted,
        })
    }
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

pub async fn camera_tasks_delete_running(pool: &mut SqliteConnection) -> Result<()> {
    sqlx::query!("DELETE FROM camera_running_tasks")
        .execute(pool)
        .await
        .context("Failed to delete running tasks")
        .ok();
    Ok(())
}

struct CameraRunningTask {
    started_at: DateTime<Utc>,
}

impl CameraManager {
    pub async fn tasks_start(&self, pool: &mut SqliteConnection) -> Result<Instant> {
        let started_at = Utc::now();

        sqlx::query!(
            "INSERT INTO camera_running_tasks (camera_id, started_at) VALUES(?, ?)",
            self.id,
            started_at
        )
        .execute(pool)
        .await
        .with_context(|| format!("Failed to create task with camera {}", self.id))?;

        Ok(Instant::now())
    }

    pub async fn tasks_end(&self, pool: &mut SqliteConnection, instant: Instant) -> Result<()> {
        let mut tx = pool.begin().await?;

        let running_task = sqlx::query_as_unchecked!(
            CameraRunningTask,
            "SELECT started_at FROM camera_running_tasks WHERE camera_id = ?",
            self.id
        )
        .fetch_one(&mut tx)
        .await
        .with_context(|| format!("Failed to find running tasks with camera {}", self.id))?;

        let duration = instant.elapsed().as_millis() as i64;

        sqlx::query!(
            "INSERT INTO camera_past_tasks (camera_id, started_at, duration) VALUES (?, ?, ?)",
            self.id,
            running_task.started_at,
            duration
        )
        .execute(&mut tx)
        .await
        .with_context(|| format!("Failed to insert into past tasks with camera {}", self.id))?;

        sqlx::query!(
            "DELETE FROM camera_running_tasks WHERE camera_id = ?",
            self.id
        )
        .execute(&mut tx)
        .await
        .with_context(|| format!("Failed to delete running task with camera {}", self.id))?;

        tx.commit().await.with_context(|| {
            format!(
                "Failed to commit end task transaction with camera {}",
                self.id
            )
        })?;

        Ok(())
    }
}
