use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils;

#[derive(Deserialize, ToSchema, Debug)]
pub struct PageQuery {
    #[serde(default)]
    pub page: i32,
    #[serde(default)]
    pub per_page: i32,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct DateTimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct Total {
    pub total: i32,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct TotalFileFilterQuery {
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(default)]
    pub kinds: Vec<String>,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub camera_ids: Vec<i64>,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct FileFilterQuery {
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub before: Option<String>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub after: Option<String>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub limit: Option<i32>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(default)]
    pub kinds: Vec<String>,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub camera_ids: Vec<i64>,
}
