use ipcmanview::{
    ipc::{IpcManager, IpcStore},
    sqlx,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub store: IpcStore,
    pub client: reqwest::Client,
}

impl AppState {
    pub async fn manager(&self, id: i64) -> anyhow::Result<IpcManager> {
        self.store.get(id).await
    }
}
