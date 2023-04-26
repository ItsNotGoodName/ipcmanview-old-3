use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rpc::modules::mediafilefind;
use sqlx::{sqlite::SqliteQueryResult, QueryBuilder, Sqlite, SqlitePool};

use crate::{
    ipc::{IpcDetail, IpcFileStream, IpcManager, IpcSoftwareVersion},
    models::CameraScanResult,
};

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
        let software = if let Some(ref s) = self.0 {
            s
        } else {
            sqlx::query!(
                r#"
                REPLACE INTO camera_software_versions
                (id)
                VALUES
                (?)
                "#,
                camera_id
            )
            .execute(pool)
            .await
            .with_context(|| {
                format!(
                    "Failed to update software version with camera {}",
                    camera_id
                )
            })?;

            return Ok(());
        };

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
            software.build,
            software.build_date,
            software.security_base_line_version,
            software.version,
            software.web_version
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
