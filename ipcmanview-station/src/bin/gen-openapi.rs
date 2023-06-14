use std::{fs, path::Path};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(components(schemas(
    ipcmanview::models::Camera,
    ipcmanview::models::CameraShow,
    ipcmanview::models::CameraDetail,
    ipcmanview::models::CameraSoftware,
    ipcmanview::models::CameraLicense,
    ipcmanview::models::CameraFile,
    ipcmanview::models::ScanCompletedPageResult,
    ipcmanview::models::CameraFileQueryResult,
    ipcmanview::models::ScanCompleted,
    ipcmanview::models::ScanActive,
    ipcmanview::models::ScanPending,
    ipcmanview::models::CreateCameraRequest,
    ipcmanview::models::UpdateCameraRequest,
    ipcmanview_station::dto::PageQuery,
    ipcmanview_station::dto::DateTimeRange,
    ipcmanview_station::dto::CameraFileTotalQuery,
    ipcmanview_station::dto::CameraFileQuery,
    ipcmanview_station::dto::TotalQueryResult
)))]
struct ApiDoc;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("swagger.json");
    fs::write(path, ApiDoc::openapi().to_pretty_json().unwrap()).expect("Unable to write file");
}
