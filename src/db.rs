use std::ops::AddAssign;

use anyhow::{Context, Result};

use chrono::{DateTime, Utc};

use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Acquire, QueryBuilder, Sqlite, SqliteConnection, Transaction};

use crate::core::{
    CameraDetail, CameraFileStream, CameraManager, CameraSoftwareVersion, CameraState,
};
use crate::models::{CameraScanCursor, Scan, ScanHandle, ScanTask};
use crate::rpc::mediafilefind::{self, FindNextFileInfo};
use crate::rpc::{self, rpclogin};

pub async fn camera_manager_find(
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

        let cursor = Scan::current_cursor();
        let camera_id = sqlx::query!(
            r#"
            INSERT INTO cameras
            (ip, username, password, scan_cursor)
            VALUES
            (?, ?, ?, ?)
            "#,
            self.client.ip,
            self.user.username,
            self.user.password,
            cursor
        )
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();

        sqlx::query!(
            r#"
            INSERT INTO camera_details
            (id)
            VALUES
            (?)
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
            (?)
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
    pub async fn scan_files(
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
            r#"
            DELETE FROM camera_files 
            WHERE updated_at < ?1 and camera_id = ?2 and start_time >= ?3 and end_time <= ?4
            "#,
            timestamp,
            self.id,
            start_time,
            end_time
        )
        .execute(&mut tx)
        .await
        .with_context(|| format!("Failed to delete stale files with camera {}", self.id))?
        .rows_affected();

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

pub async fn delete_active_scans(pool: &mut SqliteConnection) -> Result<()> {
    sqlx::query!("DELETE FROM active_scans")
        .execute(pool)
        .await
        .context("Failed to delete active scans")
        .ok();

    Ok(())
}

impl ScanTask {
    pub async fn start(self, pool: &mut SqliteConnection) -> Result<ScanHandle> {
        let runner = ScanHandle::new(self);

        sqlx::query!(
            r#"
            INSERT INTO active_scans 
            (camera_id, kind, range_start, range_end, started_at) 
            VALUES
            (?, ?, ?, ?, ?)
            "#,
            runner.camera_id,
            runner.kind,
            runner.range.start,
            runner.range.end,
            runner.started_at,
        )
        .execute(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to create active scan with camera {}",
                runner.camera_id
            )
        })?;

        Ok(runner)
    }
}

impl ScanHandle {
    pub async fn end(self, pool: &mut SqliteConnection) -> Result<()> {
        let mut tx = pool.begin().await?;
        let duration = self.instant.elapsed().as_millis() as i64;

        if self.should_save() {
            sqlx::query!(
                r#"
                INSERT INTO completed_scans 
                (camera_id, kind, range_start, range_end, started_at, duration) 
                VALUES 
                (?, ?, ?, ?, ?, ?)
                "#,
                self.camera_id,
                self.kind,
                self.range.start,
                self.range.end,
                self.started_at,
                duration
            )
            .execute(&mut tx)
            .await
            .with_context(|| {
                format!(
                    "Failed to insert into completed scans with camera {}",
                    self.camera_id
                )
            })?;
        }

        sqlx::query!(
            "DELETE FROM active_scans WHERE camera_id = ?",
            self.camera_id
        )
        .execute(&mut tx)
        .await
        .with_context(|| {
            format!(
                "Failed to delete active scan with camera {}",
                self.camera_id
            )
        })?;

        if self.should_update_scan_cursor() {
            let scan_cursor = self.range.scan_cursor();

            sqlx::query!(
                "UPDATE cameras set scan_cursor = ?2 WHERE id = ?1",
                self.camera_id,
                scan_cursor,
            )
            .execute(&mut tx)
            .await
            .with_context(|| {
                format!(
                    "Failed to update scan cursor with camera {}",
                    self.camera_id,
                )
            })?;
        }

        tx.commit().await.with_context(|| {
            format!(
                "Failed to commit end scan transaction with camera {}",
                self.camera_id
            )
        })?;

        Ok(())
    }
}

impl CameraScanCursor {
    pub async fn find(pool: &mut SqliteConnection, camera_id: i64) -> Result<Self> {
        sqlx::query_as_unchecked!(
            CameraScanCursor,
            "SELECT id, scan_cursor FROM cameras WHERE id = ?",
            camera_id
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to find scan_cursor with camera {}", camera_id))
    }
}
