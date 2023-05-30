use axum::{
    routing::{get, post},
    Router,
};

use crate::app::AppState;

mod api;
mod camera;
mod file;
mod scan;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/cameras", get(camera::list).post(camera::create))
        .route("/cameras-total", get(camera::total))
        .route(
            "/cameras/:id",
            get(camera::show)
                .post(camera::update)
                .delete(camera::delete),
        )
        .route("/cameras/:id/detail", get(camera::detail))
        .route("/cameras/:id/software", get(camera::software))
        .route("/cameras/:id/licenses", get(camera::licenses))
        .route("/cameras/:id/ipc", post(camera::refresh))
        .route("/cameras/:id/ipc/detail", post(camera::refresh_detail))
        .route("/cameras/:id/ipc/licenses", post(camera::refresh_licenses))
        .route("/cameras/:id/ipc/software", post(camera::refresh_software))
        .route("/cameras/:id/fs/*file_path", get(camera::fs))
        .route("/cameras/:id/files", get(file::query_by_camera))
        .route("/cameras/:id/files/total", get(file::total_by_camera))
        .route("/cameras/:id/scans/full", post(scan::full))
        .route("/cameras/:id/scans/manual", post(scan::manual))
        .route("/files", get(file::query))
        .route("/files/total", get(file::total))
        .route("/scans/pending", get(scan::pending_list))
        .route("/scans/active", get(scan::active_list))
        .route("/scans/completed", get(scan::completed_list))
        .route("/scans/completed/:id", get(scan::completed_show))
        .route("/scans/completed/:id/retry", post(scan::completed_retry))
        .fallback(api::fallback)
}
