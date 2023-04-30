use anyhow::{Context, Result};
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::{
    models::{
        Camera, CameraDetail, CameraFile, CameraSoftware, CreateCamera, CursorCameraFile, ICamera,
        QueryCameraFile, QueryCameraFileResult, ShowCamera, UpdateCamera,
    },
    scan::Scan,
};

impl CreateCamera<'_> {
    pub(crate) async fn create_db(self, pool: &SqlitePool) -> Result<i64> {
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
            INSERT INTO camera_softwares
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
    pub(crate) async fn update_db(self, pool: &SqlitePool) -> Result<()> {
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

    pub(crate) async fn delete_db(pool: &SqlitePool, id: i64) -> Result<()> {
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
            FROM camera_softwares 
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
    pub(crate) async fn query_db(
        pool: &SqlitePool,
        query: QueryCameraFile<'_>,
    ) -> Result<QueryCameraFileResult> {
        let mut has_after = false;
        let mut has_before = false;

        let limit = query.limit + 1;
        let files = match query.cursor {
            CursorCameraFile::After((id, time)) => {
                let mut res = sqlx::query_as_unchecked!(
                    CameraFile,
                    r#"
                    SELECT * FROM camera_files
                    WHERE (start_time < ?2 OR (start_time = ?2 AND camera_id < ?1))
                    ORDER BY start_time DESC, camera_id DESC LIMIT ?3
                    "#,
                    id,
                    time,
                    limit
                )
                .fetch_all(pool)
                .await?;

                has_before = sqlx::query!(
                    r#"
                    SELECT id FROM camera_files
                    WHERE (start_time > ?2 OR (start_time = ?2 AND camera_id > ?1))
                    LIMIT 1
                    "#,
                    id,
                    time,
                )
                .fetch_optional(pool)
                .await?
                .is_some();

                if res.len() == limit as usize {
                    has_after = true;
                    res.pop();
                }

                res
            }
            CursorCameraFile::Before((id, time)) => {
                let mut res = sqlx::query_as_unchecked!(
                    CameraFile,
                    r#"
                    SELECT * FROM camera_files 
                    WHERE (start_time > ?2 OR (start_time = ?2 AND camera_id > ?1)) 
                    ORDER BY start_time ASC, camera_id ASC LIMIT ?3
                    "#,
                    id,
                    time,
                    limit
                )
                .fetch_all(pool)
                .await?;

                has_after = sqlx::query!(
                    r#"
                    SELECT id FROM camera_files
                    WHERE (start_time < ?2 OR (start_time = ?2 AND camera_id < ?1)) 
                    LIMIT 1
                    "#,
                    id,
                    time,
                )
                .fetch_optional(pool)
                .await?
                .is_some();

                if res.len() == limit as usize {
                    has_before = true;
                    res.pop();
                }

                res.reverse();

                res
            }
            CursorCameraFile::None => {
                let mut res = sqlx::query_as_unchecked!(
                    CameraFile,
                    r#"
                    SELECT * FROM camera_files
                    ORDER BY start_time DESC, camera_id DESC LIMIT ?
                    "#,
                    limit
                )
                .fetch_all(pool)
                .await?;

                if res.len() == limit as usize {
                    has_after = true;
                    res.pop();
                }

                res
            }
        };

        let before = match files.first() {
            Some(first) => CursorCameraFile::to(first.camera_id, first.start_time),
            None => "".to_string(),
        };

        let after = match files.last() {
            Some(last) => CursorCameraFile::to(last.camera_id, last.start_time),
            None => "".to_string(),
        };

        Ok(QueryCameraFileResult {
            files,
            has_before,
            before,
            has_after,
            after,
        })
    }
}

impl<'a> QueryCameraFile<'a> {
    fn push_where(&self, mut qb: QueryBuilder<'a, Sqlite>) -> QueryBuilder<'a, Sqlite> {
        if self.camera_ids.len() > 0 {
            qb.push_bind(" AND camera_id in (");
            let mut sep = qb.separated(",");
            for id in self.camera_ids.iter() {
                sep.push_bind(id.clone());
            }
            sep.push_unseparated(")");
        }

        if self.kinds.len() > 0 {
            qb.push(" AND kind in (");
            let mut sep = qb.separated(",");
            for kind in self.kinds.iter() {
                sep.push_bind(kind.clone());
            }
            sep.push_unseparated(")");
        }

        if let Some(range_start) = self.range_start {
            qb.push(" AND start_time > ");
            qb.push_bind(range_start);
        }

        if let Some(range_end) = self.range_end {
            qb.push(" AND start_time < ");
            qb.push_bind(range_end);
        }

        qb
    }
}
