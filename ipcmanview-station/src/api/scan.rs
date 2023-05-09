use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use ipcmanview::{
    models::{ScanActive, ScanCompleted, ScanPending},
    scan::{Scan, ScanKindPending, ScanRange},
};
use serde::Deserialize;
use serde_json::json;

use crate::app::AppState;

use super::api::{Error, ResultExt};

pub async fn full(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    Scan::queue(&state.pool, &state.store, id, ScanKindPending::Full)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({})))
}

#[derive(Deserialize, Debug)]
pub struct Range {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

pub async fn manual(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(range): Json<Range>,
) -> Result<impl IntoResponse, Error> {
    let range = ScanRange::new(range.start, range.end).or_error(StatusCode::BAD_REQUEST)?;
    Scan::queue(
        &state.pool,
        &state.store,
        id,
        ScanKindPending::Manual(range),
    )
    .await
    .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({})))
}

pub async fn active_list(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    let active_scans = ScanActive::list(&state.pool)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(active_scans))
}

pub async fn pending_list(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    let pending_scans = ScanPending::list(&state.pool)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(pending_scans))
}

pub async fn completed_list(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    let completed_scans = ScanCompleted::list(&state.pool)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(completed_scans))
}

pub async fn completed_retry(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    ScanCompleted::retry(&state.pool, &state.store, id)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({})))
}