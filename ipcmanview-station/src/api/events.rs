use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use ipcmanview::models::IpcEvent;
use serde_json::json;

use super::api::{Error, ResultExt};
use crate::app::AppState;

pub async fn list(State(state): State<AppState>) -> Result<impl IntoResponse, Error> {
    let events = IpcEvent::list(&state.pool)
        .await
        .or_error(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!(events)))
}
