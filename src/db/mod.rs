use std::str::FromStr;

use crate::models::ScanActive;

pub async fn new(url: &str) -> anyhow::Result<sqlx::SqlitePool> {
    // Connect
    let options = sqlx::sqlite::SqliteConnectOptions::from_str(url)?.create_if_missing(true);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(options)
        .await?;

    // Migrate
    sqlx::migrate!().run(&pool).await?;

    ScanActive::clear(&pool).await?;

    Ok(pool)
}

pub mod camera;
pub mod ipc;
pub mod scan;
