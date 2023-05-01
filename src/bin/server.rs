use std::collections::HashMap;
use std::path::PathBuf;

use dotenvy::dotenv;
use ipcmanview::db;
use ipcmanview::ipc::{IpcManager, IpcManagerStore};
use ipcmanview::models::{
    Camera, CameraFile, CreateCamera, QueryCameraFile, QueryCameraFileFilter, ScanActive,
    ScanCompleted, ShowCamera, UpdateCamera,
};
use ipcmanview::scan::{Scan, ScanKindPending};
use rocket::form::Form;
use rocket::http::{ContentType, Header, Status};
use rocket::response::{self, Redirect};
use rocket::{Request, Response, State};
use rocket_dyn_templates::tera::{self, Tera};
use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;

type Store = State<IpcManagerStore>;
type Pool = State<sqlx::SqlitePool>;
type Client = State<reqwest::Client>;

#[main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or("sqlite://ipcmanview.db".to_string());

    let pool = db::new(&database_url).await.unwrap();
    let store = IpcManagerStore::new(&pool).await.unwrap();
    let client = reqwest::ClientBuilder::new()
        .no_deflate()
        // HACK: prevent connection reset when requesting too fast
        .pool_max_idle_per_host(0)
        .build()
        .unwrap();

    let res = rocket::build()
        .attach(Template::custom(|engines| customize(&mut engines.tera)))
        .manage(pool)
        .manage(store.clone())
        .manage(client)
        .mount(
            "/",
            routes![
                index,
                camera_list,
                camera_create,
                camera_show,
                camera_update,
                camera_delete,
                camera_data_refresh,
                camera_scan_full,
                camera_file,
                file_list,
                scan_list,
            ],
        )
        .launch()
        .await
        .map(|_| ());

    store.reset().await;

    res
}

pub fn customize(tera: &mut Tera) {
    tera.register_function(
        "camera_file_url",
        |args: &HashMap<String, serde_json::Value>| -> Result<serde_json::Value, tera::Error> {
            let id = args.get("camera_id").unwrap().as_i64().unwrap();
            let file_path = args.get("file_path").unwrap().as_str().unwrap();
            Ok(serde_json::to_value(uri!(camera_file(id, file_path))).unwrap())
        },
    );
}

// Homepage

#[get("/")]
async fn index() -> Template {
    Template::render("index", context!())
}

// Show Camera

#[get("/cameras/<id>")]
async fn camera_show(id: i64, pool: &Pool) -> Result<Template, Status> {
    let show_cam = ShowCamera::find(pool, id)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    Ok(Template::render("camera/show", context!(show_cam)))
}

// Update Camera

#[derive(FromForm)]
pub struct FormUpdateCamera<'a> {
    pub ip: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[patch("/cameras/<id>", data = "<form>")]
async fn camera_update(
    id: i64,
    form: Form<FormUpdateCamera<'_>>,
    pool: &Pool,
    store: &Store,
) -> Result<Redirect, Status> {
    UpdateCamera {
        id,
        ip: Utils::optional(form.ip),
        username: Utils::optional(form.username),
        password: Utils::optional(form.password),
    }
    .update(pool, store)
    .await
    .map_err(|_| Status::InternalServerError)?; // TODO: map to either NotFound or InternalServerError or Conflict

    Ok(Redirect::to(uri!(camera_show(id))))
}

// Refresh Camera Data

#[post("/cameras/<id>/data")]
async fn camera_data_refresh(id: i64, pool: &Pool, store: &Store) -> Result<Redirect, Status> {
    Utils::manager(store, id)
        .await?
        .data_refresh(pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Redirect::to(uri!(camera_show(id))))
}

// List Cameras

#[get("/cameras")]
async fn camera_list(pool: &Pool) -> Result<Template, Status> {
    let cams = Camera::list(pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Template::render("camera/list", context!(cams)))
}

// Create Camera

#[derive(FromForm)]
pub struct CameraCreateForm<'a> {
    pub ip: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[post("/cameras", data = "<form>")]
async fn camera_create(
    form: Form<CameraCreateForm<'_>>,
    pool: &Pool,
    store: &Store,
) -> Result<Redirect, Status> {
    let id = CreateCamera {
        ip: form.ip,
        username: form.username,
        password: form.password,
    }
    .create(pool, store)
    .await
    .map_err(|_| Status::InternalServerError)?; // TODO: map to either BadRequest, Conflict, or InternalServerError

    Ok(Redirect::to(uri!(camera_show(id))))
}

// Delete Camera

#[delete("/cameras/<id>")]
async fn camera_delete(id: i64, pool: &Pool, store: &Store) -> Result<Redirect, Status> {
    Camera::delete(pool, store, id)
        .await
        .map_err(|_| Status::InternalServerError)?; // TODO: map to either NotFound or InternalServerError

    Ok(Redirect::to(uri!(index)))
}

// Full Camera Scan

#[post("/cameras/<id>/scan/full")]
async fn camera_scan_full(id: i64, pool: &Pool, store: &Store) -> Result<Redirect, Status> {
    Scan::queue(pool, store, id, ScanKindPending::Full)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Redirect::to(uri!(scan_list())))
}

// TODO: Manual Camera Scan

// Get Camera File

struct FileStream(reqwest::Response, ContentType);

impl<'r> response::Responder<'r, 'r> for FileStream {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        let mut resp = Response::build();

        // Content-Type
        resp.header(self.1);

        // Content-Length
        if let Some(content_length) = self
            .0
            .headers()
            .get("content-length")
            .and_then(|f| f.to_str().ok())
            .map(|f| Header::new("content-length", f.to_owned()))
        {
            resp.header(content_length);
        };

        // Body
        use rocket::futures::stream::StreamExt;
        let mut stream = self.0.bytes_stream();
        let stream = response::stream::stream! {
            while let Some(Ok(item)) = stream.next().await {
                yield item;
            }
        };
        resp.streamed_body(response::stream::ReaderStream::from(
            stream.map(std::io::Cursor::new),
        ));

        resp.ok()
    }
}

#[get("/cameras/<id>/file/<file_path..>")]
async fn camera_file(
    id: i64,
    file_path: PathBuf,
    store: &Store,
    client: &Client,
) -> Result<FileStream, Status> {
    // Get file url and cookie from manager
    let file = Utils::manager(store, id)
        .await?
        .file(file_path.to_str().ok_or(Status::BadRequest)?)
        .await
        .map_err(|_| Status::InternalServerError)?;

    // Make request to camera and get the byte stream
    let resp = client
        .get(file.url)
        .header("Cookie", file.cookie)
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?
        .error_for_status()
        .map_err(|e| {
            e.status()
                .and_then(|s| Status::from_code(s.as_u16()))
                .unwrap_or(Status::InternalServerError)
        })?;

    // Get Content-Type via file path extension
    let content_type = file_path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Binary);

    Ok(FileStream(resp, content_type))
}

// List Files

#[get("/files?<before>&<after>&<limit>&<kind>&<camera_id>&<begin>&<end>")]
async fn file_list(
    before: Option<&str>,
    after: Option<&str>,
    limit: Option<i32>,
    kind: Vec<&str>,
    camera_id: Vec<i64>,
    begin: Option<&str>,
    end: Option<&str>,
    pool: &Pool,
    store: &Store,
) -> Result<Template, Status> {
    let filter = QueryCameraFileFilter::new()
        .maybe_begin(begin)
        .map_err(|_| Status::BadRequest)?
        .maybe_end(end)
        .map_err(|_| Status::BadRequest)?
        .kinds(kind)
        .camera_ids(camera_id);

    let query = QueryCameraFile::new(&filter)
        .maybe_before(before)
        .map_err(|_| Status::BadRequest)?
        .maybe_after(after)
        .map_err(|_| Status::BadRequest)?
        .maybe_limit(limit);
    let files = CameraFile::query(pool, store, query)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let files_total = CameraFile::count(pool, filter)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Template::render("files", context!(files, files_total)))
}

// List Scans

#[get("/scans")]
async fn scan_list(pool: &Pool) -> Result<Template, Status> {
    let active_scans = ScanActive::list(pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let completed_scans = ScanCompleted::list(pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Template::render(
        "scans",
        context!(active_scans, completed_scans),
    ))
}

struct Utils {}

impl Utils {
    async fn manager(store: &Store, id: i64) -> Result<IpcManager, Status> {
        store.get(id).await.map_err(|_| Status::NotFound)
    }

    fn optional<'a>(s: &'a str) -> Option<&'a str> {
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
}
