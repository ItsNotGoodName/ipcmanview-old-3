use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use ipcmanview::ipc::IpcManager;
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

impl Error {
    fn new(code: StatusCode, message: String) -> Self {
        Self(code, message, json!({}))
    }

    pub fn code(mut self, code: StatusCode) -> Self {
        self.0 = code;
        self
    }

    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.2 = data;
        self
    }
}

impl<U> From<U> for Error
where
    U: Display,
{
    fn from(value: U) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, value.to_string())
    }
}

pub trait ResultExt<T> {
    fn or_error(self, code: StatusCode) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Display,
{
    fn or_error(self, code: StatusCode) -> Result<T, Error> {
        match self {
            Ok(o) => Ok(o),
            Err(err) => Err(Error::new(code, err.to_string())),
        }
    }
}

impl AppState {
    pub async fn manager_api(&self, id: i64) -> Result<IpcManager, Error> {
        self.store
            .get_optional(id)
            .await
            .or_error(StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(Error::new(
                StatusCode::NOT_FOUND,
                "Failed to find camera in store.".to_string(),
            ))
    }
}

pub async fn fallback() -> impl IntoResponse {
    Error::from(StatusCode::NOT_FOUND)
}
