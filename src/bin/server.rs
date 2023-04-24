use dotenvy::dotenv;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use sqlx::SqlitePool;

use ipcmanview::ipc::{IpcManager, IpcManagerStore};
use ipcmanview::models::{Camera, CreateCamera, ShowCamera, UpdateCamera};
use ipcmanview::procs;

#[macro_use]
extern crate rocket;

type Store = State<IpcManagerStore>;
type Pool = State<SqlitePool>;

#[main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();

    let pool = procs::setup_database(&database_url).await.unwrap();
    let store = IpcManagerStore::new(&pool).await.unwrap();

    let res = rocket::build()
        .attach(Template::fairing())
        .manage(pool)
        .manage(store.clone())
        .mount(
            "/",
            routes![
                index,
                camera_add,
                camera_delete,
                camera_show,
                camera_refresh,
                camera_update
            ],
        )
        .launch()
        .await
        .map(|_| ());

    store.reset().await;

    res
}

async fn get_manager(store: &Store, id: i64) -> Result<IpcManager, Status> {
    store.get(id).await.map_err(|_| Status::NotFound)
}

#[get("/")]
async fn index(pool: &State<SqlitePool>) -> Result<Template, Status> {
    let cams = Camera::list(pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Template::render("index", context!(cams)))
}

#[derive(FromForm)]
pub struct FormCreateCamera<'a> {
    pub ip: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[post("/cameras", data = "<form>")]
async fn camera_add(
    form: Form<FormCreateCamera<'_>>,
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
    .map_err(|_| Status::InternalServerError)?;

    Ok(Redirect::to(uri!(camera_show(id))))
}

#[delete("/cameras/<id>")]
async fn camera_delete(id: i64, pool: &Pool, store: &Store) -> Result<Redirect, Status> {
    Camera::delete(pool, store, id)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Redirect::to(uri!(index)))
}

#[post("/cameras/<id>/all")]
async fn camera_refresh(id: i64, pool: &Pool, store: &Store) -> Result<Redirect, Status> {
    get_manager(store, id)
        .await?
        .data_refresh(pool)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Redirect::to(uri!(camera_show(id))))
}

#[get("/cameras/<id>")]
async fn camera_show(id: i64, pool: &Pool) -> Result<Template, Status> {
    let show_cam = ShowCamera::find(pool, id)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Template::render("camera/show", context!(show_cam)))
}

#[derive(FromForm)]
pub struct FormUpdateCamera<'a> {
    pub ip: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> FormUpdateCamera<'a> {
    fn to_option(s: &'a str) -> Option<&'a str> {
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
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
        ip: FormUpdateCamera::to_option(form.ip),
        username: FormUpdateCamera::to_option(form.username),
        password: FormUpdateCamera::to_option(form.password),
    }
    .update(pool, store)
    .await
    .map_err(|_| Status::InternalServerError)?;

    Ok(Redirect::to(uri!(camera_show(id))))
}

// -------------------- API

// use rocket::serde::json::Json;
// use serde::Serialize;
// #[get("/cameras")]
// async fn camera_list(pool: &State<SqlitePool>) -> Result<Json<Vec<Camera>>, Status> {
//     Ok(Camera::list(pool)
//         .await
//         .map_err(|_| Status::InternalServerError)
//         .map(|cams| Json(cams))?)
// }
//
// #[get("/cameras/<id>")]
// async fn camera_get(pool: &State<SqlitePool>, id: i64) -> Result<Option<Json<Camera>>, Status> {
//     Ok(Camera::find(pool, id)
//         .await
//         .map_err(|_| Status::InternalServerError)?
//         .map(|cam| Json(cam)))
// }
//
// #[derive(Serialize)]
// struct JsonError {
//     code: u16,
//     message: String,
// }
//
// #[catch(default)]
// fn json_catch(status: Status, _: &Request) -> Json<JsonError> {
//     Json(JsonError {
//         code: status.code,
//         message: status.to_string(),
//     })
// }
