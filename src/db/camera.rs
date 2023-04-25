use anyhow::{Context, Result};
use sqlx::SqlitePool;

use crate::{
    models::{
        Camera, CameraDetail, CameraFile, CameraSoftware, CreateCamera, ICamera, ShowCamera,
        UpdateCamera,
    },
    query::{Cursor, QueryCameraFile, QueryCameraFileResult},
    scan::Scan,
};

impl CreateCamera<'_> {
    pub async fn create_db(self, pool: &SqlitePool) -> Result<i64> {
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

        Ok(camera_id)
    }
}

impl UpdateCamera<'_> {
    pub async fn update_db(self, pool: &SqlitePool) -> Result<()> {
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
}

impl Camera {
    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, ip, username FROM cameras
            "#
        )
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to list cameras"))
    }

    pub async fn find(pool: &SqlitePool, camera_id: i64) -> Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, ip, username FROM cameras WHERE id = ?
            "#,
            camera_id,
        )
        .fetch_optional(pool)
        .await
        .with_context(|| format!("Failed to find camera {}", camera_id))
    }

    pub async fn delete_db(pool: &SqlitePool, id: i64) -> Result<()> {
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

impl ICamera {
    pub async fn find(pool: &SqlitePool, camera_id: i64) -> Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, ip, username, password FROM cameras WHERE id = ?
            "#,
            camera_id,
        )
        .fetch_optional(pool)
        .await
        .with_context(|| format!("Failed to find camera {}", camera_id))
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, ip, username, password FROM cameras
            "#
        )
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to list cameras"))
    }
}

impl CameraDetail {
    pub async fn find(pool: &SqlitePool, camera_id: i64) -> Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT 
            sn, device_class, device_type, hardware_version, market_area, process_info, vendor 
            FROM camera_details 
            WHERE id = ?
            "#,
            camera_id,
        )
        .fetch_optional(pool)
        .await
        .with_context(|| format!("Failed to find camera {}", camera_id))
    }
}

impl CameraSoftware {
    pub async fn find(pool: &SqlitePool, camera_id: i64) -> Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT 
            build,
            build_date,
            security_base_line_version,
            version,
            web_version
            FROM camera_software_versions 
            WHERE id = ?
            "#,
            camera_id,
        )
        .fetch_optional(pool)
        .await
        .with_context(|| format!("Failed to find camera {}", camera_id))
    }
}

impl ShowCamera {
    // TODO: make this into a single query
    pub async fn find(pool: &SqlitePool, id: i64) -> Result<Option<Self>> {
        let detail = match CameraDetail::find(pool, id).await? {
            Some(s) => s,
            None => return Ok(None),
        };

        let software = match CameraSoftware::find(pool, id).await? {
            Some(s) => s,
            None => return Ok(None),
        };

        let camera = match Camera::find(pool, id).await? {
            Some(s) => s,
            None => return Ok(None),
        };

        let file_count = sqlx::query!(
            "SELECT count(*) AS count FROM camera_files WHERE camera_id = ?",
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Some(ShowCamera {
            id: camera.id,
            ip: camera.ip,
            username: camera.username,
            detail,
            software,
            file_count: file_count.count,
        }))
    }
}

impl CameraFile {
    pub async fn query(
        pool: &SqlitePool,
        query: QueryCameraFile<'_>,
    ) -> Result<QueryCameraFileResult> {
        let res = match query.cursor {
            Cursor::After(cursor) => {
                let (id, time) = Cursor::from(&cursor)?;
                sqlx::query_as_unchecked!(
                    CameraFile,
                    r#"
                    SELECT * FROM camera_files
                    WHERE (start_time < ?2 OR (start_time = ?2 AND camera_id < ?1))
                    ORDER BY start_time DESC, camera_id DESC LIMIT ?3
                    "#,
                    id,
                    time,
                    query.limit
                )
                .fetch_all(pool)
                .await?
            }
            Cursor::Before(cursor) => {
                let (id, time) = Cursor::from(&cursor)?;
                sqlx::query_as_unchecked!(
                    CameraFile,
                    r#"
                    SELECT * FROM (
                        SELECT * FROM camera_files 
                        WHERE (start_time > ?2 OR (start_time = ?2 AND camera_id > ?1)) 
                        ORDER BY start_time ASC, camera_id ASC LIMIT ?3
                    ) ORDER BY start_time DESC, camera_id DESC;
                    "#,
                    id,
                    time,
                    query.limit
                )
                .fetch_all(pool)
                .await?
            }
            Cursor::None => {
                sqlx::query_as_unchecked!(
                    CameraFile,
                    r#"
                    SELECT * FROM camera_files
                    ORDER BY start_time DESC, camera_id DESC LIMIT ?
                    "#,
                    query.limit
                )
                .fetch_all(pool)
                .await?
            }
        };

        let before = if let Some(first) = res.first() {
            Cursor::to(first.camera_id, first.start_time)
        } else {
            "".to_string()
        };

        let after = if let Some(last) = res.last() {
            Cursor::to(last.camera_id, last.start_time)
        } else {
            "".to_string()
        };

        Ok(QueryCameraFileResult {
            files: res,
            before,
            after,
        })
    }
}
