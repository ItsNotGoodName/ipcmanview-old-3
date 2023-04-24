use anyhow::Result;
use dotenvy::dotenv;
use ipcmanview::scan::ScanTaskBuilder;
use sqlx::SqlitePool;

use ipcmanview::ipc::IpcManagerStore;
use ipcmanview::models::CreateCamera;
use ipcmanview::procs::setup_database;
use ipcmanview::require_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let pool =
        setup_database(&std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string()))
            .await?;
    let store = IpcManagerStore::new(&pool).await?;

    let password = require_env("IPCMANVIEW_PASSWORD")?;
    let ips = require_env("IPCMANVIEW_IPS")?;
    let ips = ips.trim();
    if ips == "" {
        panic!("IP_IPS is empty")
    }
    let ips = ips.split(" ");
    let username = std::env::var("IPCMANVIEW_USERNAME").unwrap_or("admin".to_string());
    for ip in ips {
        let req = CreateCamera {
            ip: &ip,
            username: &username,
            password: &password,
        };
        if let Err(err) = req.create(&pool, &store).await {
            dbg!(err);
        };
    }

    db_run(&pool, &store).await.ok();

    store.reset().await;

    Ok(())
}

async fn db_run(
    pool: &SqlitePool,
    store: &IpcManagerStore,
) -> Result<(), Box<dyn std::error::Error>> {
    // let value = models::ScanCompleted::list(&pool).await?;
    // let print = serde_json::to_string(&value).unwrap();
    // println!("{print}");
    // return Ok(());

    // procs::scan_task_run(pool, man, models::ScanTaskBuilder::new(man.id).full()).await?;
    // let cursor_scan_task = models::CameraScanCursor::find(pool, man.id)
    //     .await?
    //     .to_scan_task();
    // procs::scan_task_run(pool, man, cursor_scan_task).await?;

    let mut handles = Vec::new();

    for man in store.list().await {
        let pool = pool.clone();
        let handle =
            tokio::spawn(async move { ScanTaskBuilder::new(man.id).full().run(&pool, &man).await });
        handles.push(handle);
    }

    for handle in handles {
        dbg!(handle.await.unwrap().err());
    }

    println!("All threads finished");

    Ok(())
}
