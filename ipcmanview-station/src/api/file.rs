use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::Query;
use chrono::{DateTime, Utc};
use ipcmanview::models::{CameraFile, QueryCameraFile, QueryCameraFileFilter};
use serde::Deserialize;
use serde_json::json;

use crate::{app::AppState, utils};

use super::api::{Error, ResultExt};

#[derive(Deserialize, Debug)]
pub struct Filter {
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

#[derive(Deserialize, Debug)]
pub struct FileQuery {
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub before: Option<String>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub after: Option<String>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub limit: Option<i32>,
    #[serde(flatten)]
    pub filter: Filter,
}

impl From<Filter> for QueryCameraFileFilter {
    fn from(value: Filter) -> Self {
        QueryCameraFileFilter::new()
            .start(value.start)
            .end(value.end)
            .kinds(value.kinds)
            .events(value.events)
            .camera_ids(value.camera_ids)
    }
}

pub async fn query_by_camera(
    Path(id): Path<i64>,
    mut file_query: Query<FileQuery>,
    state: State<AppState>,
) -> Result<impl IntoResponse, Error> {
    file_query.filter.camera_ids = vec![id];

    query(file_query, state).await
}

pub async fn query(
    Query(query): Query<FileQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let filter = QueryCameraFileFilter::from(query.filter);
    let query = QueryCameraFile::new(&filter)
        .maybe_before(query.before)
        .or_error(StatusCode::BAD_REQUEST)?
        .maybe_after(query.after)
        .or_error(StatusCode::BAD_REQUEST)?
        .maybe_limit(query.limit);
    let files = CameraFile::query(&state.pool, &state.store, query)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(files))
}

pub async fn total_by_camera(
    Path(id): Path<i64>,
    mut filter: Query<Filter>,
    state: State<AppState>,
) -> Result<impl IntoResponse, Error> {
    filter.camera_ids = vec![id];

    total(filter, state).await
}

pub async fn total(
    Query(filter): Query<Filter>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let filter = QueryCameraFileFilter::from(filter);
    let total = CameraFile::total(&state.pool, &filter)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({ "total": total })))
}
