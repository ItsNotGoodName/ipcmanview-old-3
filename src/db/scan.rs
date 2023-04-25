use anyhow::{bail, Context, Result};

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::models::{ScanActive, ScanCompleted};
use crate::scan::{Scan, ScanCamera, ScanHandle, ScanKindPending, ScanRange};

const MAX_PENDING_MANUAL_SCANS: i32 = 5;

struct ScanPending {
    id: i64,
    camera_id: i64,
    kind: ScanKindPending,
}

struct ScanManualPending {
    id: i64,
    camera_id: i64,
    range_start: DateTime<Utc>,
    range_end: DateTime<Utc>,
}

impl Scan {
    pub async fn queue_db(pool: &SqlitePool, camera_id: i64, kind: ScanKindPending) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO pending_scans 
            (camera_id, kind)
            VALUES
            (?, ?)
            ON CONFLICT DO NOTHING
            "#,
            camera_id,
            kind,
        )
        .execute(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to insert into pending_scans with camera {}",
                camera_id
            )
        })
        .map(|_| ())
    }

    pub async fn queue_manual_db(
        pool: &SqlitePool,
        camera_id: i64,
        range: ScanRange,
    ) -> Result<()> {
        let mut pool = pool
            .begin()
            .await
            .with_context(|| format!("Failed to start transaction with camera {}", camera_id))?;

        let count = sqlx::query!(
            "SELECT count(*) AS count FROM pending_manual_scans WHERE camera_id = ?",
            camera_id
        )
        .fetch_one(&mut pool)
        .await?;
        if count.count >= MAX_PENDING_MANUAL_SCANS {
            bail!("Too many pending manual scans {MAX_PENDING_MANUAL_SCANS}")
        }

        sqlx::query!(
            r#"
            INSERT INTO pending_manual_scans
            (camera_id, range_start, range_end)
            VALUES
            (?, ?, ?)
            "#,
            camera_id,
            range.start,
            range.end,
        )
        .execute(&mut pool)
        .await
        .with_context(|| {
            format!(
                "Failed to insert into pending_manual_scans with camera {}",
                camera_id
            )
        })?;

        pool.commit()
            .await
            .with_context(|| format!("Failed to commit transaction with camera {}", camera_id))
    }
}

impl ScanHandle {
    pub async fn next(pool: &SqlitePool) -> Result<Option<Self>> {
        let mut pool = pool
            .begin()
            .await
            .context(format!("Failed to start transaction"))?;

        // Create a handle from either pending_scans or pending_manual_scans, return if there none
        let handle = if let Some(pending) = sqlx::query_as_unchecked!(ScanPending,
            r#"
            SELECT * FROM pending_scans WHERE (camera_id) NOT IN (SELECT camera_id FROM active_scans) LIMIT 1
            "#,
        ).fetch_optional(&mut pool).await? {
            // Delete pending scan
            sqlx::query!("DELETE FROM pending_scans WHERE id = ?", pending.id)
                .execute(&mut pool)
                .await?;

            // Create handle from pending scan kind
            match pending.kind {
                ScanKindPending::Full => ScanHandle::full(pending.camera_id),
                ScanKindPending::Cursor => {
                    // Get scan camera
                    let scan_camera = sqlx::query_as_unchecked!(
                        ScanCamera,
                        "SELECT id, scan_cursor FROM cameras WHERE id = ?",
                        pending.camera_id
                    )
                    .fetch_one(&mut pool)
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to find scan_cursor with camera {}",
                            pending.camera_id
                        )
                    })?;

                    ScanHandle::cursor(scan_camera)
                }
            }
        } else if let Some(pending) = sqlx::query_as_unchecked!(ScanManualPending,
            r#"
            SELECT * FROM pending_manual_scans WHERE (camera_id) NOT IN (SELECT camera_id FROM active_scans) LIMIT 1
            "#,
        ).fetch_optional(&mut pool).await? {
            // Delete pending manual scan
            sqlx::query!("DELETE FROM pending_manual_scans WHERE id = ?", pending.id)
            .execute(&mut pool)
            .await?;

            // Create manual scan handle
            ScanHandle::manual(
                pending.camera_id,
                ScanRange {
                    start: pending.range_start,
                    end: pending.range_end,
                },
            )
        } else {
            return Ok(None);
        };

        // Insert handle into active scans
        sqlx::query!(
            r#"
            INSERT INTO active_scans
            (camera_id, kind, range_start, range_end, started_at)
            VALUES
            (?, ?, ?, ?, ?)
            "#,
            handle.camera_id,
            handle.kind,
            handle.range.start,
            handle.range.end,
            handle.started_at,
        )
        .execute(&mut pool)
        .await
        .with_context(|| {
            format!(
                "Failed to create active scan with camera {}",
                handle.camera_id
            )
        })?;

        pool.commit()
            .await
            .context(format!("Failed to commit transaction"))?;

        Ok(Some(handle))
    }

    pub async fn end(self, pool: &SqlitePool) -> Result<()> {
        let mut pool = pool.begin().await.with_context(|| {
            format!("Failed to start transaction with camera {}", self.camera_id)
        })?;
        let duration = self.instant.elapsed().as_millis() as i64;

        // Save scan handle to completed_scans
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
            .execute(&mut pool)
            .await
            .with_context(|| {
                format!(
                    "Failed to insert into completed scans with camera {}",
                    self.camera_id
                )
            })?;
        }

        // Delete scan handle from active_scans
        sqlx::query!(
            "DELETE FROM active_scans WHERE camera_id = ?",
            self.camera_id
        )
        .execute(&mut pool)
        .await
        .with_context(|| {
            format!(
                "Failed to delete active scan with camera {}",
                self.camera_id
            )
        })?;

        // Update camera scan cursor
        if self.should_update_scan_cursor() {
            let scan_cursor = self.range.scan_cursor();

            sqlx::query!(
                "UPDATE cameras SET scan_cursor = ?2 WHERE id = ?1",
                self.camera_id,
                scan_cursor,
            )
            .execute(&mut pool)
            .await
            .with_context(|| {
                format!(
                    "Failed to update scan cursor with camera {}",
                    self.camera_id,
                )
            })?;
        }

        pool.commit().await.with_context(|| {
            format!(
                "Failed to commit transaction with camera {}",
                self.camera_id
            )
        })
    }
}

impl ScanActive {
    pub async fn clear(pool: &SqlitePool) -> Result<()> {
        sqlx::query!("DELETE FROM active_scans")
            .execute(pool)
            .await
            .context("Failed to delete active scans")
            .ok();

        Ok(())
    }

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
