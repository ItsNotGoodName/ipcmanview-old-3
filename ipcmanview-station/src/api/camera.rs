use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use ipcmanview::{
    db,
    dto::{CreateCamera, UpdateCamera},
    models::{Camera, ShowCamera},
};
use serde_json::json;

use crate::app::AppState;

use super::api::{Error, ResultExt};

pub async fn list(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    let cameras = Camera::list(&state.pool)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!(cameras)))
}

pub async fn fs(
    Path((id, file_path)): Path<(i64, String)>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let file = state
        .manager(id)
        .await?
        .file(&file_path)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: maybe use hyper HTTP connector
    // Make request to camera
    let resp = state
        .client
        .get(file.url)
        .header(header::COOKIE, file.cookie)
        .send()
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?
        .error_for_status()
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

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

pub async fn create(
    State(state): State<AppState>,
    Json(json): Json<CreateCamera>,
) -> Result<impl IntoResponse, Error> {
    // TODO: validate request
    let id = json.create(&state.pool, &state.store).await.map_err(|e| {
        if db::Conflict == e {
            Error::from((StatusCode::CONFLICT, e))
        } else {
            Error::from((StatusCode::INTERNAL_SERVER_ERROR, e))
        }
    })?;

    Ok((StatusCode::CREATED, Json(json!({ "id": id }))))
}

pub async fn delete(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    Camera::delete(&state.pool, &state.store, id)
        .await
        .map_err(|e| {
            if db::NotFound == e {
                Error::from((StatusCode::NOT_FOUND, e))
            } else {
                Error::from((StatusCode::INTERNAL_SERVER_ERROR, e))
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn show(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let show_camera = ShowCamera::find(&state.pool, id).await.map_err(|e| {
        if db::NotFound == e {
            Error::from((StatusCode::NOT_FOUND, e))
        } else {
            Error::from((StatusCode::INTERNAL_SERVER_ERROR, e))
        }
    })?;

    Ok(Json(show_camera))
}

pub async fn update(
    State(state): State<AppState>,
    Json(json): Json<UpdateCamera>,
) -> Result<impl IntoResponse, Error> {
    // TODO: validate request
    json.update(&state.pool, &state.store).await.map_err(|e| {
        if db::NotFound == e {
            Error::from((StatusCode::NOT_FOUND, e))
        } else if db::Conflict == e {
            Error::from((StatusCode::CONFLICT, e))
        } else {
            Error::from((StatusCode::INTERNAL_SERVER_ERROR, e))
        }
    })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn refresh(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    state
        .manager(id)
        .await?
        .refresh(&state.pool)
        .await
        .map_err(|e| {
            if db::NotFound == e {
                Error::from((StatusCode::NOT_FOUND, e))
            } else {
                Error::from((StatusCode::INTERNAL_SERVER_ERROR, e))
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn refresh_detail(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    state
        .manager(id)
        .await?
        .refresh_detail(&state.pool)
        .await
        .map_err(|e| {
            if db::NotFound == e {
                Error::from((StatusCode::NOT_FOUND, e))
            } else {
                Error::from((StatusCode::INTERNAL_SERVER_ERROR, e))
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn refresh_licenses(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    state
        .manager(id)
        .await?
        .refresh_licenses(&state.pool)
        .await
        .map_err(|e| {
            if db::NotFound == e {
                Error::from((StatusCode::NOT_FOUND, e))
            } else {
                Error::from((StatusCode::INTERNAL_SERVER_ERROR, e))
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn refresh_software(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    state
        .manager(id)
        .await?
        .refresh_software(&state.pool)
        .await
        .map_err(|e| {
            if db::NotFound == e {
                Error::from((StatusCode::NOT_FOUND, e))
            } else {
                Error::from((StatusCode::INTERNAL_SERVER_ERROR, e))
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}
