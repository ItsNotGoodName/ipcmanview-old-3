use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};

use crate::ipc::{IpcManager, IpcManagerStore};

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub store: IpcManagerStore,
    pub client: reqwest::Client,
}

impl AppState {
    pub async fn manager(&self, id: i64) -> anyhow::Result<IpcManager> {
        self.store.get(id).await
    }
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(route::index_page))
        .route("/files", get(route::files_page))
        .route("/cameras", post(route::camera_create))
        .route(
            "/cameras/:id",
            get(route::camera_page).post(route::camera_update),
        )
        .route("/cameras/:id/delete", post(route::camera_delete))
        .route("/cameras/:id/data", post(route::camera_refresh))
        .route("/cameras/:id/file/*file_path", get(route::camera_file))
        .route("/cameras/:id/scan/full", post(route::camera_full_scan))
        .route("/scans", get(route::scans_page))
        .route("/scans/completed/:id", post(route::scan_completed_retry))
        .fallback(route::error_404)
        .with_state(app_state)
}

mod utils {
    use std::{fmt, str::FromStr};

    use serde::{de, Deserialize, Deserializer};

    pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: fmt::Display,
    {
        let opt = Option::<String>::deserialize(de)?;
        match opt.as_deref() {
            None | Some("") => Ok(None),
            Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
        }
    }
}

mod route {
    use askama::Template;
    use axum::{
        body::StreamBody,
        extract::{Path, State},
        http::{header, HeaderMap, StatusCode},
        response::{IntoResponse, Redirect},
        Form,
    };
    use axum_extra::extract::Query;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    use super::{utils, AppError, AppState};
    use crate::{
        db,
        models::{
            Camera, CameraFile, CreateCamera, QueryCameraFile, QueryCameraFileFilter,
            QueryCameraFileResult, ScanActive, ScanCompleted, ScanPending, ShowCamera,
            UpdateCamera,
        },
        scan::{Scan, ScanKindPending},
    };

    pub async fn error_404() -> impl IntoResponse {
        (StatusCode::NOT_FOUND, Error404Template {})
    }

    #[derive(Template)]
    #[template(path = "404.j2.html")]
    struct Error404Template {}

    pub async fn index_page(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
        let cameras = Camera::list(&state.pool).await?;

        Ok(IndexPageTemplate { cameras })
    }

    #[derive(Template)]
    #[template(path = "index.j2.html")]
    struct IndexPageTemplate {
        cameras: Vec<Camera>,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct FilesPageQuery {
        #[serde(default, deserialize_with = "utils::empty_string_as_none")]
        pub before: Option<String>,
        #[serde(default, deserialize_with = "utils::empty_string_as_none")]
        pub after: Option<String>,
        #[serde(default, deserialize_with = "utils::empty_string_as_none")]
        pub limit: Option<i32>,
        #[serde(default, deserialize_with = "utils::empty_string_as_none")]
        pub begin: Option<DateTime<Utc>>,
        #[serde(default, deserialize_with = "utils::empty_string_as_none")]
        pub end: Option<DateTime<Utc>>,
        #[serde(default)]
        pub kinds: Vec<String>,
        #[serde(default)]
        pub events: Vec<String>,
        #[serde(default)]
        pub camera_ids: Vec<i64>,
    }

    pub async fn files_page(
        Query(query): Query<FilesPageQuery>,
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse, AppError> {
        let mut query2 = query.clone();
        query2.after = None;
        query2.before = None;

        let filter = QueryCameraFileFilter::new()
            .begin(query.begin)
            .end(query.end)
            .kinds(query.kinds)
            .events(query.events)
            .camera_ids(query.camera_ids);

        let query = QueryCameraFile::new(&filter)
            .maybe_before(query.before)?
            .maybe_after(query.after)?
            .maybe_limit(query.limit);
        let files = CameraFile::query(&state.pool, &state.store, query).await?;

        let events = db::camera::events(&state.pool).await?;
        let cameras = Camera::list(&state.pool).await?;

        let files_total = CameraFile::count(&state.pool, &filter).await?;

        query2.after = Some(files.after.clone());
        let after_query = { serde_html_form::ser::to_string(&query2).unwrap_or_default() };
        query2.after = None;

        query2.before = Some(files.before.clone());
        let before_query = serde_html_form::ser::to_string(&query2).unwrap_or_default();
        query2.before = None;

        Ok(FilesPageTemplate {
            cameras,
            events,
            files_total,
            files,
            before_query,
            after_query,
        })
    }

    #[derive(Template)]
    #[template(path = "files.j2.html")]
    struct FilesPageTemplate {
        cameras: Vec<Camera>,
        events: Vec<String>,
        files_total: i64,
        files: QueryCameraFileResult,
        before_query: String,
        after_query: String,
    }

    pub async fn camera_file(
        Path((id, file_path)): Path<(i64, String)>,
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse, AppError> {
        let file = state.manager(id).await?.file(&file_path).await?;

        // TODO: maybe use hyper HTTP connector
        // Make request to camera
        let resp = state
            .client
            .get(file.url)
            .header("Cookie", file.cookie)
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
    pub async fn camera_page(
        Path(id): Path<i64>,
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse, AppError> {
        let show_cameras = ShowCamera::find(&state.pool, id).await?.unwrap(); // TODO: NotFound

        Ok(CameraPageTemplate { show_cameras })
    }

    #[derive(Template)]
    #[template(path = "camera/show.j2.html")]
    struct CameraPageTemplate {
        show_cameras: ShowCamera,
    }

    pub async fn camera_create(
        State(state): State<AppState>,
        Form(form): Form<CreateCamera>,
    ) -> Result<impl IntoResponse, AppError> {
        let id = form.create(&state.pool, &state.store).await?; // TODO: map to either BadRequest, Conflict, or InternalServerError

        Ok(Redirect::to(format!("/cameras/{id}").as_str()))
    }

    #[derive(Deserialize, Debug)]
    pub struct CameraUpdate {
        #[serde(default, deserialize_with = "utils::empty_string_as_none")]
        pub ip: Option<String>,
        #[serde(default, deserialize_with = "utils::empty_string_as_none")]
        pub username: Option<String>,
        #[serde(default, deserialize_with = "utils::empty_string_as_none")]
        pub password: Option<String>,
    }

    pub async fn camera_update(
        Path(id): Path<i64>,
        State(state): State<AppState>,
        Form(form): Form<CameraUpdate>,
    ) -> Result<impl IntoResponse, AppError> {
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

    pub async fn camera_full_scan(
        Path(id): Path<i64>,
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse, AppError> {
        Scan::queue(&state.pool, &state.store, id, ScanKindPending::Full).await?;

        Ok(Redirect::to("/scans"))
    }

    pub async fn camera_delete(
        Path(id): Path<i64>,
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse, AppError> {
        Camera::delete(&state.pool, &state.store, id).await?; // TODO: map to either NotFound or InternalServerError

        Ok(Redirect::to("/"))
    }

    pub async fn camera_refresh(
        Path(id): Path<i64>,
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse, AppError> {
        state.manager(id).await?.refresh(&state.pool).await?;

        Ok(Redirect::to(format!("/cameras/{id}").as_str()))
    }

    pub async fn scans_page(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
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
    #[template(path = "scans.j2.html")]
    struct ScansPageTemplate {
        active_scans: Vec<ScanActive>,
        completed_scans: Vec<ScanCompleted>,
        pending_scans: Vec<ScanPending>,
    }

    pub async fn scan_completed_retry(
        Path(id): Path<i64>,
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse, AppError> {
        ScanCompleted::retry(&state.pool, &state.store, id).await?;

        Ok(Redirect::to(format!("/scans").as_str()))
    }

    mod filters {
        use crate::models::CameraFile;

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
                Ok("https://bulma.io/images/placeholders/128x128.png".to_string())
            }
        }
    }
}
