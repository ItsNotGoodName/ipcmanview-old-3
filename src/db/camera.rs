use anyhow::{Context, Result};
use chrono::Utc;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::dto::{CreateCamera, UpdateCamera};
use crate::{
    models::{
        Camera, CameraDetail, CameraFile, CameraLicense, CameraSoftware, ICamera, QueryCameraFile,
        QueryCameraFileCursor, QueryCameraFileFilter, QueryCameraFileResult, ShowCamera,
    },
    scan::Scan,
};

impl CreateCamera {
    pub(crate) async fn create_db(self, pool: &SqlitePool) -> Result<i64> {
        let mut pool = pool.begin().await?;

        let cursor = Scan::cursor();
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
        .execute(&mut *pool)
        .await
        .context("Failed to create camera")?
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
        .execute(&mut *pool)
        .await
        .context("Failed to create camera_details")?;

        sqlx::query!(
            r#"
            INSERT INTO camera_softwares
            (id)
            VALUES
            (?)
            "#,
            camera_id
        )
        .execute(&mut *pool)
        .await
        .context("Failed to create camera_softwares")?;

        pool.commit().await?;

        Ok(camera_id)
    }
}

impl UpdateCamera {
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
        .with_context(|| format!("Failed to update camera {}", self.id))
        .map(|_| ())
    }
}

impl Camera {
    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as_unchecked!(
            Self,
            r#"
            SELECT id, ip, username, refreshed_at, created_at
            FROM cameras
            "#
        )
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to list cameras"))
    }

    pub async fn find(pool: &SqlitePool, camera_id: i64) -> Result<Option<Self>> {
        sqlx::query_as_unchecked!(
            Self,
            r#"
            SELECT id, ip, username, refreshed_at, created_at
            FROM cameras
            WHERE id = ?
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
        .with_context(|| format!("Failed to delete camera {}", id))
        .map(|_| ())
    }

    pub(crate) async fn update_refreshed_at(pool: &SqlitePool, id: i64) -> Result<()> {
        let refreshed_at = Utc::now();
        sqlx::query!(
            "UPDATE cameras SET refreshed_at = ? WHERE id = ?",
            refreshed_at,
            id
        )
        .execute(pool)
        .await
        .with_context(|| format!("Failed to update refreshed_at with camera {}", id))
        .map(|_| ())
    }
}

impl ICamera {
    pub async fn find(pool: &SqlitePool, camera_id: i64) -> Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, ip, username, password
            FROM cameras
            WHERE id = ?
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
            SELECT id, ip, username, password
            FROM cameras
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
            SELECT sn, device_class, device_type, hardware_version, market_area, process_info, vendor
            FROM camera_details
            WHERE id = ?
            "#,
            camera_id,
        )
        .fetch_optional(pool)
        .await
        .with_context(|| format!("Failed to find camera_details with camera {}", camera_id))
    }
}

impl CameraSoftware {
    pub async fn find(pool: &SqlitePool, camera_id: i64) -> Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT build, build_date, security_base_line_version, version, web_version
            FROM camera_softwares
            WHERE id = ?
            "#,
            camera_id,
        )
        .fetch_optional(pool)
        .await
        .with_context(|| format!("Failed to find camera_softwares with camera {}", camera_id))
    }
}

impl CameraLicense {
    pub async fn list(pool: &SqlitePool, camera_id: i64) -> Result<Vec<Self>> {
        sqlx::query_as_unchecked!(
            Self,
            r#"
            SELECT
                abroad_info,
                all_type,
                digit_channel,
                effective_days,
                effective_time,
                license_id,
                product_type,
                status,
                username
            FROM camera_licenses
            WHERE camera_id = ?
            "#,
            camera_id,
        )
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to list camera_licenses with camera {}", camera_id))
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

        let licenses = CameraLicense::list(pool, id).await?;

        let camera = match Camera::find(pool, id).await? {
            Some(s) => s,
            None => return Ok(None),
        };

        let file = sqlx::query!(
            "SELECT count(*) AS total FROM camera_files WHERE camera_id = ?",
            id
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to count camera_files with camera {id}"))?;

        Ok(Some(ShowCamera {
            id: camera.id,
            ip: camera.ip,
            username: camera.username,
            refreshed_at: camera.refreshed_at,
            created_at: camera.created_at,
            detail,
            software,
            licenses,
            file_total: file.total,
        }))
    }
}

trait CameraFileFilter<'a> {
    fn push_camera_file_filter(self, query: &'a QueryCameraFileFilter) -> QueryBuilder<'a, Sqlite>;
}

impl<'a> CameraFileFilter<'a> for QueryBuilder<'a, Sqlite> {
    fn push_camera_file_filter(
        mut self,
        filter: &'a QueryCameraFileFilter,
    ) -> QueryBuilder<'a, Sqlite> {
        self.push(" WHERE 1=1");

        if let Some(start) = filter.start {
            self.push(" AND start_time > ");
            self.push_bind(start);
        }

        if let Some(end) = filter.end {
            self.push(" AND start_time < ");
            self.push_bind(end);
        }

        if filter.camera_ids.len() > 0 {
            self.push(" AND camera_id in (");
            let mut sep = self.separated(",");
            for id in filter.camera_ids.iter() {
                sep.push_bind(id.clone());
            }
            sep.push_unseparated(")");
        }

        if filter.kinds.len() > 0 {
            self.push(" AND kind in (");
            let mut sep = self.separated(",");
            for kind in filter.kinds.iter() {
                sep.push_bind(kind.clone());
            }
            sep.push_unseparated(")");
        }

        if filter.events.len() > 0 {
            self.push(" AND (");
            for (idx, event) in filter.events.iter().enumerate() {
                if idx != 0 {
                    self.push(" OR");
                };
                self.push(" events LIKE '%\"'||");
                self.push_bind(event);
                self.push("||'\"%'");
            }
            self.push(")");
        }

        self
    }
}

#[derive(sqlx::FromRow)]
struct CameraFileCount {
    count: i64,
}

impl CameraFile {
    pub async fn count(pool: &SqlitePool, filter: &QueryCameraFileFilter) -> Result<i64> {
        let count = QueryBuilder::new("SELECT COUNT(id) AS count FROM camera_files")
            .push_camera_file_filter(&filter)
            .build_query_as::<CameraFileCount>()
            .fetch_one(pool)
            .await
            .context("Failed to count camera_files")?
            .count;

        Ok(count)
    }

    pub(crate) async fn query_db(
        pool: &SqlitePool,
        query: QueryCameraFile<'_>,
    ) -> Result<QueryCameraFileResult> {
        let mut has_after = false;
        let mut has_before = false;

        let limit = query.limit + 1;
        let mut qb =
            QueryBuilder::new("SELECT * FROM camera_files").push_camera_file_filter(query.filter);
        let files = match query.cursor {
            QueryCameraFileCursor::After((id, time)) => {
                let mut files = qb
                    .push(" AND (start_time < ")
                    .push_bind(time)
                    .push(" OR (start_time = ")
                    .push_bind(time)
                    .push(" AND camera_id < ")
                    .push_bind(id)
                    .push(")) ORDER BY start_time DESC, camera_id DESC LIMIT ")
                    .push_bind(limit)
                    .build_query_as::<CameraFile>()
                    .fetch_all(pool)
                    .await?;

                if files.len() == limit as usize {
                    has_after = true;
                    files.pop();
                }

                has_before = QueryBuilder::new("SELECT id FROM camera_files")
                    .push_camera_file_filter(query.filter)
                    .push(" AND (start_time > ")
                    .push_bind(time)
                    .push(" OR (start_time = ")
                    .push_bind(time)
                    .push(" AND camera_id > ")
                    .push_bind(id)
                    .push(")) LIMIT 1")
                    .build()
                    .fetch_optional(pool)
                    .await?
                    .is_some();

                files
            }
            QueryCameraFileCursor::Before((id, time)) => {
                let mut files = qb
                    .push(" AND (start_time > ")
                    .push_bind(time)
                    .push(" OR (start_time = ")
                    .push_bind(time)
                    .push(" AND camera_id > ")
                    .push_bind(id)
                    .push(")) ORDER BY start_time ASC, camera_id ASC LIMIT ")
                    .push_bind(limit)
                    .build_query_as::<CameraFile>()
                    .fetch_all(pool)
                    .await?;

                if files.len() == limit as usize {
                    has_before = true;
                    files.pop();
                }

                has_after = QueryBuilder::new("SELECT id FROM camera_files")
                    .push_camera_file_filter(query.filter)
                    .push(" AND (start_time < ")
                    .push_bind(time)
                    .push(" OR (start_time = ")
                    .push_bind(time)
                    .push(" AND camera_id < ")
                    .push_bind(id)
                    .push(")) LIMIT 1")
                    .build()
                    .fetch_optional(pool)
                    .await?
                    .is_some();

                files.reverse();

                files
            }
            QueryCameraFileCursor::None => {
                let mut files = qb
                    .push(" ORDER BY start_time DESC, camera_id DESC LIMIT ")
                    .push_bind(limit)
                    .build_query_as::<CameraFile>()
                    .fetch_all(pool)
                    .await?;

                if files.len() == limit as usize {
                    has_after = true;
                    files.pop();
                }

                files
            }
        };

        let before = match files.first() {
            Some(first) => QueryCameraFileCursor::to(first.camera_id, first.start_time),
            None => "".to_string(),
        };

        let after = match files.last() {
            Some(last) => QueryCameraFileCursor::to(last.camera_id, last.start_time),
            None => "".to_string(),
        };

        let count = files.len() as i32;

        Ok(QueryCameraFileResult {
            files,
            has_before,
            before,
            has_after,
            after,
            count,
        })
    }
}
