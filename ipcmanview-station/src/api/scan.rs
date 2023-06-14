use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::Query;
use ipcmanview::{
    models::{Page, ScanActive, ScanCompleted, ScanPending},
    scan::{Scan, ScanKindPending, ScanRange},
};

use crate::{
    app::AppState,
    models::{DateTimeRange, PageQuery},
};

use super::api::{Error, ResultExt};

pub async fn full(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    Scan::queue(&state.pool, &state.store, id, ScanKindPending::Full)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::ACCEPTED)
}

pub async fn manual(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(range): Json<DateTimeRange>,
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

    Ok(StatusCode::ACCEPTED)
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

pub async fn completed_list(
    Query(query): Query<PageQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let page = Page::from(query);
    let completed_scans = ScanCompleted::list(&state.pool, page)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(completed_scans))
}

pub async fn completed_show(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let completed_scan = ScanCompleted::find(&state.pool, id)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(completed_scan))
}

pub async fn completed_retry(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    ScanCompleted::retry(&state.pool, &state.store, id)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::ACCEPTED)
}
