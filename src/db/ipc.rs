use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use dahua_rpc::modules::mediafilefind;
use sqlx::{sqlite::SqliteQueryResult, QueryBuilder, Sqlite, SqlitePool};

use crate::{
    ipc::{IpcDetail, IpcFileStream, IpcLicenses, IpcManager, IpcSoftware},
    models::{CameraScanResult, IpcEvent},
};

use super::NotFound;

impl IpcDetail {
    pub async fn save(&self, pool: &SqlitePool, camera_id: i64) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE camera_details SET 
            sn = ?2,
            device_class = ?3,
            device_type = ?4,
            hardware_version = ?5,
            market_area = ?6,
            process_info = ?7,
            vendor = ?8
            WHERE id = ?1
            "#,
            camera_id,
            self.sn,
            self.device_class,
            self.device_type,
            self.hardware_version,
            self.market_area,
            self.process_info,
            self.vendor,
        )
        .execute(pool)
        .await
        .with_context(|| format!("Failed to update camera detail with camera id {camera_id}."))
        .map(NotFound::check_query)?
        .with_context(|| format!("Failed to find camera detail with camera id {camera_id}."))
    }
}

impl IpcSoftware {
    pub async fn save(&self, pool: &SqlitePool, camera_id: i64) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE camera_softwares SET 
            build = ?2,
            build_date = ?3,
            security_base_line_version = ?4,
            version = ?5,
            web_version = ?6
            WHERE id = ?1
            "#,
            camera_id,
            self.0.build,
            self.0.build_date,
            self.0.security_base_line_version,
            self.0.version,
            self.0.web_version,
        )
        .execute(pool)
        .await
        .with_context(|| format!("Failed to update camera software with camera id {camera_id}."))
        .map(NotFound::check_query)?
        .with_context(|| format!("Failed to find camera software with camera id {camera_id}."))
    }
}

impl IpcLicenses {
    pub async fn save(&self, pool: &SqlitePool, camera_id: i64) -> Result<()> {
        let mut pool = pool.begin().await?;

        sqlx::query!("DELETE FROM camera_licenses WHERE camera_id = ?", camera_id,)
            .execute(&mut pool)
            .await
            .with_context(|| {
                format!("Failed to delete camera licenses with camera id {camera_id}.")
            })?;

        for license in self.0.iter() {
            sqlx::query!(
                r#"
                INSERT INTO camera_licenses
                (
                camera_id,
                abroad_info,
                all_type,
                digit_channel,
                effective_days,
                effective_time,
                license_id,
                product_type,
                status,
                username
                )
                VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                camera_id,
                license.info.abroad_info,
                license.info.all_type,
                license.info.digit_channel,
                license.info.effective_days,
                license.info.effective_time,
                license.info.license_id,
                license.info.product_type,
                license.info.status,
                license.info.username
            )
            .execute(&mut pool)
            .await
            .with_context(|| {
                format!("Failed to insert camera licenses with camera id {camera_id}.")
            })?;
        }

        pool.commit().await?;

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
            "INSERT INTO camera_files (camera_id, file_path, kind, size, updated_at, start_time, end_time, events) ",
        );
        let mut unique_events: Vec<String> = vec![];

        // Upsert files
        let res = qb
            .push_values(files, |mut b, file| {
                let (start_time, end_time) = file.unique_time();
                let events = serde_json::json!(file.events).to_string();
                b.push_bind(camera_id)
                    .push_bind(file.file_path)
                    .push_bind(file.r#type)
                    .push_bind(file.length)
                    .push_bind(timestamp)
                    .push_bind(start_time)
                    .push_bind(end_time)
                    .push_bind(events);
                for event in file.events {
                    if !unique_events.contains(&event) {
                        unique_events.push(event);
                    };
                }
            })
            .push(
                "ON CONFLICT (camera_id, file_path) DO UPDATE SET updated_at=excluded.updated_at ",
            )
            // If for some reason the unique_time function generates a duplicate time then we should ignore the file being upserted and buy a lottery ticket
            .push("ON CONFLICT (camera_id, start_time) DO NOTHING")
            .build()
            .execute(pool)
            .await
            .with_context(|| format!("Failed to upsert files with camera id {camera_id}."))?;

        // Upsert events
        QueryBuilder::<Sqlite>::new("INSERT OR IGNORE INTO ipc_events (name)")
            .push_values(unique_events, |mut b, event| {
                b.push_bind(event);
            })
            .build()
            .execute(pool)
            .await
            .with_context(|| format!("Failed to upsert ipc events with camera id {camera_id}."))?;

        Ok(res)
    }

    async fn scan_files_condition(
        &self,
        pool: &SqlitePool,
        condition: mediafilefind::Condition,
        timestamp: DateTime<Utc>,
    ) -> Result<u64> {
        let mut stream = IpcFileStream::new(self, condition).await.with_context(|| {
            format!(
                "Failed to create ipc file stream with camera id {}.",
                self.id
            )
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
                "Error after streaming ipc files with camera id {}.",
                self.id
            ))
        } else {
            Ok(upserted)
        }
    }

    /// This should never run concurrently with overlapping start and end times. But it is best to
    /// only run it once per camera as it increases load on the camera which leads to HTTP
    /// connection resets.
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
            WHERE updated_at < ? AND camera_id = ? AND start_time >= ? AND start_time <= ?
            "#,
            timestamp,
            self.id,
            start_time,
            end_time
        )
        .execute(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to delete stale camera files with camera id {}.",
                self.id
            )
        })?
        .rows_affected();

        Ok(CameraScanResult { deleted, upserted })
    }
}

impl IpcEvent {
    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as_unchecked!(Self, "SELECT name FROM ipc_events")
            .fetch_all(pool)
            .await
            .context("Failed to list ipc events.")
    }
}
