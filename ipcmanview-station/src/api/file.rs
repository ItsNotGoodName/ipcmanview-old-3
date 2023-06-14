use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::Query;
use ipcmanview::models::{CameraFile, CameraFileQuery, CameraFileQueryFilter};

use crate::{app::AppState, dto};

use super::api::{Error, ResultExt};

pub async fn query_by_camera(
    Path(id): Path<i64>,
    mut file_filter_query: Query<dto::CameraFileQuery>,
    state: State<AppState>,
) -> Result<impl IntoResponse, Error> {
    file_filter_query.camera_ids = vec![id];

    query(file_filter_query, state).await
}

pub async fn query(
    Query(query): Query<dto::CameraFileQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let filter = CameraFileQueryFilter::new()
        .start(query.start)
        .end(query.end)
        .kinds(query.kinds)
        .events(query.events)
        .camera_ids(query.camera_ids);
    let query = CameraFileQuery::new(&filter)
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
    mut filter_query: Query<dto::CameraFileTotalQuery>,
    state: State<AppState>,
) -> Result<impl IntoResponse, Error> {
    filter_query.camera_ids = vec![id];

    total(filter_query, state).await
}

pub async fn total(
    Query(query): Query<dto::CameraFileTotalQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let filter = CameraFileQueryFilter::new()
        .start(query.start)
        .end(query.end)
        .kinds(query.kinds)
        .events(query.events)
        .camera_ids(query.camera_ids);
    let total = CameraFile::total(&state.pool, &filter)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(dto::TotalQueryResult { total }))
}
