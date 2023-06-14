use std::{fs, path::Path};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(components(schemas(
    ipcmanview::models::Camera,                         // Camera
    ipcmanview::models::ShowCamera,                     // CameraShow
    ipcmanview::models::CameraDetail,                   // CameraDetail
    ipcmanview::models::CameraSoftware,                 // CameraSoftware
    ipcmanview::models::CameraLicense,                  // CameraLicense
    ipcmanview::models::CameraFile,                     // CameraFile
    ipcmanview::models::PageResultScanCompleted,        // ScanCompletedPageResult
    ipcmanview::models::QueryCameraFileResult,          // CameraFileQueryResult
    ipcmanview::models::ScanCompleted,                  // ScanCompleted
    ipcmanview::models::ScanActive,                     // ScanActive
    ipcmanview::models::ScanPending,                    // ScanPending
    ipcmanview::dto::CreateCamera,                      // CreateCameraRequest
    ipcmanview::dto::UpdateCamera,                      // UpdateCameraRequest
    ipcmanview_station::models::PageQuery,              // PageQuery
    ipcmanview_station::models::DateTimeRange,          // DateTimeRange
    ipcmanview_station::models::TotalFileFilterQuery,   // CameraFileTotalQuery
    ipcmanview_station::models::FileFilterQuery,        // CameraFileQuery
    ipcmanview_station::models::Total                   // TotalQueryResult
)))]
struct ApiDoc;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("swagger.json");
    fs::write(path, ApiDoc::openapi().to_pretty_json().unwrap()).expect("Unable to write file");
}
