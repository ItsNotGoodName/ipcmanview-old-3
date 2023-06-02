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
pub struct FilterQuery {
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    start: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    end: Option<DateTime<Utc>>,
    #[serde(default)]
    kinds: Vec<String>,
    #[serde(default)]
    events: Vec<String>,
    #[serde(default)]
    camera_ids: Vec<i64>,
}

#[derive(Deserialize, Debug)]
pub struct FileFilterQuery {
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    before: Option<String>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    after: Option<String>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    limit: Option<i32>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    start: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    end: Option<DateTime<Utc>>,
    #[serde(default)]
    kinds: Vec<String>,
    #[serde(default)]
    events: Vec<String>,
    #[serde(default)]
    camera_ids: Vec<i64>,
}

pub async fn query_by_camera(
    Path(id): Path<i64>,
    mut file_filter_query: Query<FileFilterQuery>,
    state: State<AppState>,
) -> Result<impl IntoResponse, Error> {
    file_filter_query.camera_ids = vec![id];

    query(file_filter_query, state).await
}

pub async fn query(
    Query(query): Query<FileFilterQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let filter = QueryCameraFileFilter::new()
        .start(query.start)
        .end(query.end)
        .kinds(query.kinds)
        .events(query.events)
        .camera_ids(query.camera_ids);
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
    mut filter_query: Query<FilterQuery>,
    state: State<AppState>,
) -> Result<impl IntoResponse, Error> {
    filter_query.camera_ids = vec![id];

    total(filter_query, state).await
}

pub async fn total(
    Query(query): Query<FilterQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let filter = QueryCameraFileFilter::new()
        .start(query.start)
        .end(query.end)
        .kinds(query.kinds)
        .events(query.events)
        .camera_ids(query.camera_ids);
    let total = CameraFile::total(&state.pool, &filter)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({ "total": total })))
}
