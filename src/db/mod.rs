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

#[derive(thiserror::Error, Default, Debug)]
#[error("Not Found")]
pub struct NotFound;

impl NotFound {
    fn check_query(r: sqlx::sqlite::SqliteQueryResult) -> Result<(), NotFound> {
        if r.rows_affected() == 0 {
            Err(Self)
        } else {
            Ok(())
        }
    }
}

impl PartialEq<anyhow::Error> for NotFound {
    fn eq(&self, other: &anyhow::Error) -> bool {
        other.downcast_ref::<Self>().is_some()
    }
}

pub mod camera;
pub mod ipc;
pub mod scan;
