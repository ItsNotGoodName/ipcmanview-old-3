use anyhow::{Context, Result};

use sqlx::SqlitePool;

use crate::models::{ScanActive, ScanCompleted};
use crate::scan::{ScanCamera, ScanHandle, ScanTask};

impl ScanActive {
    pub async fn clear(pool: &SqlitePool) -> Result<()> {
        sqlx::query!("DELETE FROM active_scans")
            .execute(pool)
            .await
            .context("Failed to delete active scans")
            .ok();

        Ok(())
    }
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
