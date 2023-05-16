use askama::Template;
use axum::response::Response;
use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use axum_extra::extract::Query;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{app::AppState, utils};
use ipcmanview::{
    dto::{CreateCamera, UpdateCamera},
    models::{
        Camera, CameraFile, IpcEvent, QueryCameraFile, QueryCameraFileFilter,
        QueryCameraFileResult, ScanActive, ScanCompleted, ScanPending, ShowCamera,
    },
    scan::{Scan, ScanKindPending},
};

struct MpaError(anyhow::Error);

impl IntoResponse for MpaError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for MpaError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index_page))
        .route("/files", get(files_page))
        .route("/cameras", post(camera_create))
        .route("/cameras/:id", get(camera_page).post(camera_update))
        .route("/cameras/:id/delete", post(camera_delete))
        .route("/cameras/:id/data", post(camera_refresh))
        .route("/cameras/:id/file/*file_path", get(camera_file))
        .route("/cameras/:id/scan/full", post(camera_full_scan))
        .route("/scans", get(scans_page))
        .route("/scans/completed/:id", post(scan_completed_retry))
        .fallback(fallback)
}

async fn fallback() -> impl IntoResponse {
    (StatusCode::BAD_REQUEST, Error400Template {})
}

#[derive(Template)]
#[template(path = "400.jinja.html")]
struct Error400Template {}

async fn index_page(State(state): State<AppState>) -> Result<impl IntoResponse, MpaError> {
    let cameras = Camera::list(&state.pool).await?;

    Ok(IndexPageTemplate { cameras })
}

#[derive(Template)]
#[template(path = "index.jinja.html")]
struct IndexPageTemplate {
    cameras: Vec<Camera>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct FilesPageQuery {
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

async fn files_page(
    Query(query): Query<FilesPageQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, MpaError> {
    let mut current_query = query.clone();
    current_query.after = None;
    current_query.before = None;

    let filter = QueryCameraFileFilter::new()
        .start(query.start)
        .end(query.end)
        .kinds(query.kinds)
        .events(query.events)
        .camera_ids(query.camera_ids);

    let query = QueryCameraFile::new(&filter)
        .maybe_before(query.before)?
        .maybe_after(query.after)?
        .maybe_limit(query.limit);
    let files = CameraFile::query(&state.pool, &state.store, query).await?;

    let ipc_events = IpcEvent::list(&state.pool).await?;
    let cameras = Camera::list(&state.pool).await?;

    let files_total = CameraFile::count(&state.pool, &filter).await?;

    current_query.after = Some(files.after.clone());
    let after_query = serde_html_form::ser::to_string(&current_query).unwrap_or_default();
    current_query.after = None;

    current_query.before = Some(files.before.clone());
    let before_query = serde_html_form::ser::to_string(&current_query).unwrap_or_default();
    current_query.before = None;

    Ok(FilesPageTemplate {
        cameras,
        ipc_events,
        files_total,
        files,
        before_query,
        after_query,
    })
}

#[derive(Template)]
#[template(path = "files.jinja.html")]
struct FilesPageTemplate {
    cameras: Vec<Camera>,
    ipc_events: Vec<IpcEvent>,
    files_total: i64,
    files: QueryCameraFileResult,
    before_query: String,
    after_query: String,
}

async fn camera_file(
    Path((id, file_path)): Path<(i64, String)>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, MpaError> {
    let file = state.manager(id).await?.file(&file_path).await?;

    // TODO: maybe use hyper HTTP connector
    // Make request to camera
    let resp = state
        .client
        .get(file.url)
        .header(header::COOKIE, file.cookie)
        .send()
        .await?
        .error_for_status()?;

    let mut headers = HeaderMap::new();

    // Get Content-Type from file path
    if let Some(content_type) = mime_guess::from_path(file_path).first() {
        headers.insert(
            header::CONTENT_TYPE,
            content_type.to_string().parse().unwrap(),
        );
    };

    // Get Content-Length from request
    if let Some(content_length) = resp
        .headers()
        .get("content-length")
        .and_then(|f| f.to_str().ok())
    {
        headers.insert(header::CONTENT_LENGTH, content_length.parse().unwrap());
    }

    // Get stream from request body
    let stream = resp.bytes_stream();
    let body = StreamBody::new(stream);

    Ok((headers, body))
}

async fn camera_page(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, MpaError> {
    let show_cameras = ShowCamera::find(&state.pool, id).await?; // TODO: NotFound

    Ok(CameraPageTemplate { show_cameras })
}

#[derive(Template)]
#[template(path = "camera/show.jinja.html")]
struct CameraPageTemplate {
    show_cameras: ShowCamera,
}

async fn camera_create(
    State(state): State<AppState>,
    Form(form): Form<CreateCamera>,
) -> Result<impl IntoResponse, MpaError> {
    let id = form.create(&state.pool, &state.store).await?; // TODO: map to either BadRequest, Conflict, or InternalServerError

    Ok(Redirect::to(format!("/cameras/{id}").as_str()))
}

#[derive(Deserialize, Debug)]
struct CameraUpdate {
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub ip: Option<String>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub username: Option<String>,
    #[serde(default, deserialize_with = "utils::empty_string_as_none")]
    pub password: Option<String>,
}

async fn camera_update(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Form(form): Form<CameraUpdate>,
) -> Result<impl IntoResponse, MpaError> {
    UpdateCamera {
        id,
        ip: form.ip,
        username: form.username,
        password: form.password,
    }
    .update(&state.pool, &state.store)
    .await?; // TODO: map to either BadRequest, Conflict, or InternalServerError

    Ok(Redirect::to(format!("/cameras/{id}").as_str()))
}

async fn camera_full_scan(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, MpaError> {
    Scan::queue(&state.pool, &state.store, id, ScanKindPending::Full).await?;

    Ok(Redirect::to("/scans"))
}

async fn camera_delete(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, MpaError> {
    Camera::delete(&state.pool, &state.store, id).await?; // TODO: map to either NotFound or InternalServerError

    Ok(Redirect::to("/"))
}

async fn camera_refresh(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, MpaError> {
    state.manager(id).await?.refresh(&state.pool).await?;

    Ok(Redirect::to(format!("/cameras/{id}").as_str()))
}

async fn scans_page(State(state): State<AppState>) -> Result<impl IntoResponse, MpaError> {
    let active_scans = ScanActive::list(&state.pool).await?;
    let completed_scans = ScanCompleted::list(&state.pool).await?;
    let pending_scans = ScanPending::list(&state.pool).await?;

    Ok(ScansPageTemplate {
        active_scans,
        completed_scans,
        pending_scans,
    })
}

#[derive(Template)]
#[template(path = "scans.jinja.html")]
struct ScansPageTemplate {
    active_scans: Vec<ScanActive>,
    completed_scans: Vec<ScanCompleted>,
    pending_scans: Vec<ScanPending>,
}

async fn scan_completed_retry(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, MpaError> {
    ScanCompleted::retry(&state.pool, &state.store, id).await?;

    Ok(Redirect::to(format!("/scans").as_str()))
}

mod filters {
    use chrono::{DateTime, Local, Utc};
    use humantime::format_duration;

    use ipcmanview::models::CameraFile;

    pub fn url_camera_file(file: &CameraFile) -> ::askama::Result<String> {
        Ok(format!(
            "/cameras/{}/file/{}",
            file.camera_id, file.file_path
        ))
    }

    pub fn url_camera_file_image(file: &CameraFile) -> ::askama::Result<String> {
        if file.kind == "jpg" {
            url_camera_file(file)
        } else {
            Ok("https://placehold.co/600x400.png".to_string())
        }
    }

    pub fn format_date(date: &DateTime<Utc>) -> ::askama::Result<String> {
        Ok(date
            .with_timezone(&Local)
            .format("%d/%m/%Y %I:%M %p")
            .to_string())
    }

    pub fn duration(start: &DateTime<Utc>, end: &DateTime<Utc>) -> ::askama::Result<String> {
        Ok((end.clone() - start.clone())
            .to_std()
            .map_or("".to_string(), |d| format_duration(d).to_string()))
    }
}
