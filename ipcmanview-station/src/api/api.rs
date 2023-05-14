use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use ipcmanview::ipc::IpcManager;
use serde_json::json;

use crate::app::AppState;

pub struct Error(StatusCode, String);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(json!({
                "code": self.0.as_u16(),
                "message": self.1.to_string(),
                "data": {}
            })),
        )
            .into_response()
    }
}

impl From<StatusCode> for Error {
    fn from(value: StatusCode) -> Self {
        Error(value, value.to_string())
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
            Err(err) => Err(Error(code, err.to_string())),
        }
    }
}

pub trait OptionExt<T> {
    fn or_option_error(self) -> Result<T, Error>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_option_error(self) -> Result<T, Error> {
        match self {
            Some(s) => Ok(s),
            None => Err(Error::from(StatusCode::NOT_FOUND)),
        }
    }
}

impl AppState {
    pub async fn manager_api(&self, id: i64) -> Result<IpcManager, Error> {
        self.store
            .get_optional(id)
            .await
            .or_error(StatusCode::NOT_FOUND)?
            .or_option_error()
    }
}

pub async fn fallback() -> impl IntoResponse {
    Error::from(StatusCode::NOT_FOUND)
}
