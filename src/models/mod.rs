use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

mod page;
mod query;
mod scan;

#[derive(Deserialize, ToSchema, Debug)]
pub struct CreateCameraRequest {
    pub ip: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct UpdateCameraRequest {
    pub id: i64,
    pub ip: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct Camera {
    pub id: i64,
    pub ip: String,
    pub username: String,
    pub refreshed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct CameraShow {
    pub id: i64,
    pub ip: String,
    pub username: String,
    pub refreshed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub detail: CameraDetail,
    pub software: CameraSoftware,
    pub file_total: i32,
    pub licenses: Vec<CameraLicense>,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct CameraDetail {
    pub sn: String,
    pub device_class: String,
    pub device_type: String,
    pub hardware_version: String,
    pub market_area: String,
    pub process_info: String,
    pub vendor: String,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct CameraSoftware {
    pub build: String,
    pub build_date: String,
    pub security_base_line_version: String,
    pub version: String,
    pub web_version: String,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct CameraLicense {
    pub abroad_info: String,
    pub all_type: bool,
    pub digit_channel: u32,
    pub effective_days: u32,
    pub effective_time: DateTime<Utc>,
    pub license_id: u32,
    pub product_type: String,
    pub status: u32,
    pub username: String,
}

#[derive(Serialize, ToSchema, sqlx::FromRow, Debug)]
pub struct CameraFile {
    pub id: i64,
    pub camera_id: i64,
    pub file_path: String,
    pub kind: String,
    pub size: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub events: sqlx::types::Json<Vec<String>>,
}

pub struct IpcEvent {}

pub struct Page {
    pub page: i32,
    pub per_page: i32,
}

#[derive(Serialize, ToSchema, Debug)]
#[aliases(ScanCompletedPageResult = PageResult<ScanCompleted>)]
pub struct PageResult<T> {
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub total_items: i32,
    pub items: Vec<T>,
}

#[derive(Debug)]
pub enum CameraFileQueryCursor {
    Before((i64, DateTime<Utc>)),
    After((i64, DateTime<Utc>)),
    None,
}

#[derive(Debug)]
pub struct CameraFileQueryFilter {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub camera_ids: Vec<i64>,
    pub kinds: Vec<String>,
    pub events: Vec<String>,
}

#[derive(Debug)]
pub struct CameraFileQuery<'a> {
    pub cursor: CameraFileQueryCursor,
    pub limit: i32,
    pub filter: &'a CameraFileQueryFilter,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct CameraFileQueryResult {
    pub files: Vec<CameraFile>,
    pub has_before: bool,
    pub before: String,
    pub has_after: bool,
    pub after: String,
    pub count: i32,
}

pub struct ICamera {
    pub id: i64,
    pub ip: String,
    pub username: String,
    pub password: String,
}

#[derive(Default, Debug)]
pub struct CameraScanResult {
    pub upserted: u64,
    pub deleted: u64,
}

#[derive(sqlx::Type, serde::Serialize, Debug)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ScanKind {
    Full,
    Cursor,
    Manual,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ScanCompleted {
    pub id: i64,
    pub camera_id: i64,
    pub kind: ScanKind,
    pub range_start: DateTime<Utc>,
    pub range_end: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub range_cursor: DateTime<Utc>,
    pub duration: i64,
    pub error: String,
    pub percent: f64,
    pub upserted: i64,
    pub deleted: i64,
    pub success: bool,
    pub retry_pending: bool,
    pub can_retry: bool,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ScanActive {
    pub camera_id: i64,
    pub kind: ScanKind,
    pub range_start: DateTime<Utc>,
    pub range_end: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub range_cursor: DateTime<Utc>,
    pub percent: f64,
    pub upserted: i64,
    pub deleted: i64,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ScanPending {
    pub id: i64,
    pub camera_id: i64,
    pub range_start: DateTime<Utc>,
    pub range_end: DateTime<Utc>,
    pub kind: ScanKind,
}
