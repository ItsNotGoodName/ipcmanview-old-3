use std::ops::AddAssign;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::scan::ScanKind;

#[derive(Deserialize, Debug)]
pub struct CreateCamera<'a> {
    pub ip: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct UpdateCamera<'a> {
    pub id: i64,
    pub ip: Option<&'a str>,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
}

#[derive(Serialize, Debug)]
pub struct Camera {
    pub id: i64,
    pub ip: String,
    pub username: String,
}

#[derive(Serialize, Debug)]
pub struct ShowCamera {
    pub id: i64,
    pub ip: String,
    pub username: String,
    pub detail: CameraDetail,
    pub software: CameraSoftware,
    pub file_count: i32,
}

#[derive(Serialize, Debug)]
pub struct CameraDetail {
    pub sn: Option<String>,
    pub device_class: Option<String>,
    pub device_type: Option<String>,
    pub hardware_version: Option<String>,
    pub market_area: Option<String>,
    pub process_info: Option<String>,
    pub vendor: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct CameraSoftware {
    pub build: Option<String>,
    pub build_date: Option<String>,
    pub security_base_line_version: Option<String>,
    pub version: Option<String>,
    pub web_version: Option<String>,
}

#[derive(Serialize, sqlx::FromRow, Debug)]
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

#[derive(Debug)]
pub enum QueryCameraFileCursor {
    Before((i64, DateTime<Utc>)),
    After((i64, DateTime<Utc>)),
    None,
}

#[derive(Debug)]
pub struct QueryCameraFileFilter<'a> {
    pub begin: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub camera_ids: Vec<i64>,
    pub kinds: Vec<&'a str>,
    pub events: Vec<&'a str>,
}

#[derive(Debug)]
pub struct QueryCameraFile<'a> {
    pub cursor: QueryCameraFileCursor,
    pub limit: i32,
    pub filter: &'a QueryCameraFileFilter<'a>,
}

#[derive(Serialize, Debug)]
pub struct QueryCameraFileResult {
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

impl AddAssign for CameraScanResult {
    fn add_assign(&mut self, rhs: Self) {
        self.upserted += rhs.upserted;
        self.deleted += rhs.deleted;
    }
}

#[derive(Serialize, Debug)]
pub struct ScanCompleted {
    pub id: i64,
    pub camera_id: i64,
    pub kind: ScanKind,
    pub range_start: DateTime<Utc>,
    pub range_end: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub duration: i64,
    pub error: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ScanActive {
    pub camera_id: i64,
    pub kind: ScanKind,
    pub range_start: DateTime<Utc>,
    pub range_end: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub percent: f64,
}
