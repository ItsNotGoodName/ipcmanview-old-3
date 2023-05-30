use std::fmt::{Debug, Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use ipcmanview::{ipc::IpcManager, models::Page};
use serde::Deserialize;
use serde_json::json;

use crate::app::AppState;

pub struct Error(StatusCode, String, serde_json::Value);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(json!({
                "code": self.0.as_u16(),
                "message": self.1.to_string(),
                "data": self.2
            })),
        )
            .into_response()
    }
}

impl<U> From<(StatusCode, U)> for Error
where
    U: Display + Debug,
{
    fn from(value: (StatusCode, U)) -> Self {
        if value.0.is_server_error() {
            tracing::error!("{:?}", value.1)
        }

        Self(value.0, value.1.to_string(), json!({}))
    }
}

impl From<StatusCode> for Error {
    fn from(value: StatusCode) -> Self {
        if value.is_server_error() {
            tracing::error!("{:?}", value)
        }

        Self(value, value.to_string(), json!({}))
    }
}

pub trait ResultExt<T> {
    fn or_error(self, code: StatusCode) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Display + Debug,
{
    fn or_error(self, code: StatusCode) -> Result<T, Error> {
        match self {
            Ok(o) => Ok(o),
            Err(err) => Err(Error::from((code, err))),
        }
    }
}

impl AppState {
    pub async fn manager(&self, id: i64) -> Result<IpcManager, Error> {
        self.store
            .get_optional(id)
            .await
            .or_error(StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(Error::from((
                StatusCode::NOT_FOUND,
                format!("Failed to find camera in store with camera id {id}."),
            )))
    }
}

pub async fn fallback() -> impl IntoResponse {
    Error::from(StatusCode::NOT_FOUND)
}

#[derive(Deserialize, Debug)]
pub struct PageQuery {
    #[serde(default)]
    pub page: i32,
    #[serde(default)]
    pub per_page: i32,
}

impl From<PageQuery> for Page {
    fn from(value: PageQuery) -> Self {
        Page::new(value.page, value.per_page)
    }
}
