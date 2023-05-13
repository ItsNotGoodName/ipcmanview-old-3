use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::app::AppState;

mod api;
mod camera;
mod file;
mod scan;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/cameras", get(camera::list))
        .route("/cameras", post(camera::create))
        .route("/cameras/:id", post(camera::update))
        .route("/cameras/:id", get(camera::show))
        .route("/cameras/:id", delete(camera::delete))
        .route("/cameras/:id/refresh", post(camera::refresh))
        .route("/cameras/:id/fs/*file_path", get(camera::fs))
        .route("/cameras/:id/files", get(file::query_by_camera))
        .route("/cameras/:id/files/total", get(file::total_by_camera))
        .route("/cameras/:id/scans/full", post(scan::full))
        .route("/cameras/:id/scans/manual", post(scan::manual))
        .route("/files", get(file::query))
        .route("/files/total", get(file::total))
        .route("/scans/cameras/:id/full", post(scan::full))
        .route("/scans/cameras/:id/manual", post(scan::manual))
        .route("/scans/pending", get(scan::pending_list))
        .route("/scans/active", get(scan::active_list))
        .route("/scans/completed", get(scan::completed_list))
        .route("/scans/completed/:id/retry", post(scan::completed_retry))
        .fallback(api::fallback)
}
