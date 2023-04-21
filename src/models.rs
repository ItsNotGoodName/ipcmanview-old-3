use std::ops::AddAssign;

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::scan::ScanKind;

pub struct CreateCamera {
    pub ip: String,
    pub username: String,
    pub password: String,
}

pub struct UpdateCamera {
    pub id: i64,
    pub ip: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub struct Camera {
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
    #[serde(with = "ts_milliseconds")]
    pub range_start: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub range_end: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub started_at: DateTime<Utc>,
    pub duration: i64,
    pub error: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ScanActive {
    pub camera_id: i64,
    pub kind: ScanKind,
    #[serde(with = "ts_milliseconds")]
    pub range_start: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub range_end: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub started_at: DateTime<Utc>,
}
