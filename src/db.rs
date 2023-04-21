use anyhow::{Context, Result};

use chrono::{DateTime, Utc};

use rpc::modules::mediafilefind;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::ipc::{IpcDetail, IpcFileStream, IpcManager, IpcSoftwareVersion};
use crate::models::{
    Camera, CameraScanResult, CreateCamera, ScanActive, ScanCompleted, UpdateCamera,
};
use crate::scan::{Scan, ScanCamera, ScanHandle, ScanTask};

impl Camera {
    pub async fn find(pool: &SqlitePool, camera_id: i64) -> Result<Self> {
        sqlx::query_as!(
            Camera,
            r#"
            SELECT id, ip, username, password FROM cameras WHERE id = ?
            "#,
            camera_id,
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to find camera {}", camera_id))
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as!(
            Camera,
            r#"
            SELECT id, ip, username, password FROM cameras
            "#
        )
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to list cameras"))
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM cameras 
            WHERE id = ?
            "#,
            id
        )
        .execute(pool)
        .await
        .with_context(|| format!("Failed to delete camera {}", id))?
        .rows_affected();

        Ok(())
    }
}

impl UpdateCamera {
    pub async fn update(self, pool: &SqlitePool) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE cameras SET 
            ip = coalesce(?, ip),
            username = coalesce(?, username),
            password = coalesce(?, password)
            WHERE id = ?
            "#,
            self.ip,
            self.username,
            self.password,
            self.id,
        )
        .execute(pool)
        .await
        .with_context(|| format!("Failed to update detail with camera {}", self.id))?;
        Ok(())
    }

    pub async fn update_and_find(self, pool: &SqlitePool) -> Result<Camera> {
        let id = self.id;
        self.update(pool).await?;
        Camera::find(pool, id).await
    }
}

impl CreateCamera {
    pub async fn create(self, pool: &SqlitePool) -> Result<Camera> {
        let mut tx = pool.begin().await?;

        let cursor = Scan::current_cursor();
        let camera_id = sqlx::query!(
            r#"
            INSERT INTO cameras
            (ip, username, password, scan_cursor)
            VALUES
            (?, ?, ?, ?)
            "#,
            self.ip,
            self.username,
            self.password,
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

        Ok(Camera {
            id: camera_id,
            ip: self.ip,
            username: self.username,
            password: self.password,
        })
    }
}

impl IpcDetail {
    pub async fn save(&self, pool: &SqlitePool, camera_id: i64) -> Result<()> {
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

impl IpcSoftwareVersion {
    pub async fn save(&self, pool: &SqlitePool, camera_id: i64) -> Result<()> {
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
            self.software.build,
            self.software.build_date,
            self.software.security_base_line_version,
            self.software.version,
            self.software.web_version
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

impl IpcManager {
    async fn upsert_files(
        pool: &SqlitePool,
        camera_id: i64,
        files: Vec<mediafilefind::FindNextFileInfo>,
        timestamp: &chrono::DateTime<Utc>,
    ) -> Result<SqliteQueryResult> {
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
            "INSERT INTO camera_files (camera_id, file_path, kind, size, updated_at, start_time, end_time) ",
        );

        qb.push_values(files, |mut b, file| {
            let (start_time, end_time) = file.unique_time();
            b.push_bind(camera_id)
                .push_bind(file.file_path)
                .push_bind(file.r#type)
                .push_bind(file.length)
                .push_bind(timestamp)
                .push_bind(start_time)
                .push_bind(end_time);
        })
        .push("ON CONFLICT (camera_id, file_path) DO UPDATE SET updated_at=excluded.updated_at ")
        // If for some reason the unique_time function generates a duplicate time then we should ignore the file being upserted and by a lottery ticket
        .push("ON CONFLICT (camera_id, start_time) DO NOTHING")
        .build()
        .execute(pool)
        .await
        .with_context(|| format!("Failed to upsert files with camera {}", camera_id))
    }

    async fn scan_files_condition(
        &self,
        pool: &SqlitePool,
        condition: mediafilefind::Condition,
        timestamp: DateTime<Utc>,
    ) -> Result<u64> {
        let mut stream = IpcFileStream::new(self, condition).await.with_context(|| {
            format!("Failed to create file info stream with camera {}", self.id)
        })?;

        let mut upserted: u64 = 0;
        while let Some(files) = stream.next().await {
            if files.len() > 0 {
                upserted += Self::upsert_files(pool, self.id, files, &timestamp)
                    .await?
                    .rows_affected();
            }
        }

        if let Some(err) = stream.error {
            Err(err).context(format!(
                "Error after streaming files with camera {}",
                self.id
            ))
        } else {
            Ok(upserted)
        }
    }

    // Make sure this is never ran concurrently
    pub async fn scan_files(
        &self,
        pool: &SqlitePool,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<CameraScanResult> {
        let timestamp = Utc::now();

        // Upsert videos (dav)
        let mut upserted = Self::scan_files_condition(
            &self,
            pool,
            mediafilefind::Condition::new(start_time, end_time).video(),
            timestamp,
        )
        .await?;

        // Upsert pictures (jpg)
        upserted += Self::scan_files_condition(
            &self,
            pool,
            mediafilefind::Condition::new(start_time, end_time).picture(),
            timestamp,
        )
        .await?;

        let deleted = sqlx::query!(
            r#"
            DELETE FROM camera_files 
            WHERE updated_at < ?1 and camera_id = ?2 and start_time >= ?3 and end_time <= ?4
            "#,
            timestamp,
            self.id,
            start_time,
            end_time
        )
        .execute(pool)
        .await
        .with_context(|| format!("Failed to delete stale files with camera {}", self.id))?
        .rows_affected();

        Ok(CameraScanResult { deleted, upserted })
    }
}

pub async fn active_scans_clear(pool: &SqlitePool) -> Result<()> {
    sqlx::query!("DELETE FROM active_scans")
        .execute(pool)
        .await
        .context("Failed to delete active scans")
        .ok();

    Ok(())
}

impl ScanTask {
    pub async fn start(self, pool: &SqlitePool) -> Result<ScanHandle> {
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
    pub async fn end(self, pool: &SqlitePool) -> Result<()> {
        let mut tx = pool.begin().await?;
        let duration = self.instant.elapsed().as_millis() as i64;

        if self.should_save() {
            sqlx::query!(
                r#"
                INSERT INTO completed_scans 
                (camera_id, kind, range_start, range_end, started_at, duration, error)
                VALUES 
                (?, ?, ?, ?, ?, ?, ?)
                "#,
                self.camera_id,
                self.kind,
                self.range.start,
                self.range.end,
                self.started_at,
                duration,
                self.error
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

impl ScanCamera {
    pub async fn find(pool: &SqlitePool, camera_id: i64) -> Result<Self> {
        sqlx::query_as_unchecked!(
            ScanCamera,
            "SELECT id, scan_cursor FROM cameras WHERE id = ?",
            camera_id
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to find scan_cursor with camera {}", camera_id))
    }
}

impl ScanActive {
    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as_unchecked!(
            Self,
            r#"
            SELECT camera_id, kind, range_start, range_end, started_at FROM active_scans
            "#
        )
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to list active scans"))
    }
}

impl ScanCompleted {
    // TODO: add filters
    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as_unchecked!(
            Self,
            r#"
            SELECT id, camera_id, kind, range_start, range_end, started_at, duration, error FROM completed_scans
            "#
        )
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to list completed scans"))
    }
}
