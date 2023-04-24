use std::io;
use std::io::Write;

use anyhow::{bail, Result};
use dotenvy::dotenv;
use ipcmanview::scan::ScanTaskBuilder;
use sqlx::SqlitePool;

use ipcmanview::ipc::{IpcManager, IpcManagerStore};
use ipcmanview::models::{self, CreateCamera};
use ipcmanview::procs::{setup_database, setup_store};
use ipcmanview::{client_print, procs, require_env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    match std::env::args().skip(1).next() {
        Some(command) => match command.as_str() {
            "http" => http().await,
            "db" => db().await,
            "cli" => cli().await,
            _ => Err("Invalid Command".into()),
        },
        None => Err("No Command".into()),
    }
}

async fn http() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn db() -> Result<(), Box<dyn std::error::Error>> {
    let client = rpc::new_http_client();
    let pool =
        setup_database(&std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string()))
            .await?;
    let mut store = setup_store(&pool, client.clone()).await?;

    let password = require_env("IPCMANVIEW_PASSWORD")?;
    let ips = require_env("IPCMANVIEW_IPS")?;
    let ips = ips.trim();
    if ips == "" {
        panic!("IP_IPS is empty")
    }
    let ips = ips.split(" ");
    let username = std::env::var("IPCMANVIEW_USERNAME").unwrap_or("admin".to_string());
    for ip in ips {
        let create = CreateCamera {
            ip: ip.to_string(),
            username: username.clone(),
            password: password.clone(),
        };
        if let Err(err) = procs::camera_create(&pool, &mut store, create).await {
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
        let handle = tokio::spawn(async move {
            procs::scan_task_run(&pool, &man, ScanTaskBuilder::new(man.id).full()).await
        });
        handles.push(handle);
    }

    for handle in handles {
        dbg!(handle.await.unwrap().err());
    }

    println!("All threads finished");

    Ok(())
}

fn cli_get_input(input: &mut String, message: &str) -> Result<(), io::Error> {
    print!("{}", message);
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(input)?;
    *input = input.trim().to_string();
    Ok(())
}

async fn store_from_env() -> Result<IpcManagerStore> {
    let password = require_env("IPCMANVIEW_PASSWORD")?;
    let ips = require_env("IPCMANVIEW_IPS")?;
    let ips = ips.trim();
    if ips == "" {
        bail!("IP_IPS is empty")
    }
    let ips = ips.split(" ");
    let username = std::env::var("IPCMANVIEW_USERNAME").unwrap_or("admin".to_string());

    let store = IpcManagerStore::new().await;
    let client = rpc::new_http_client();
    for (id, ip) in ips.enumerate() {
        let client = rpc::Client::new(
            client.clone(),
            ip.to_string(),
            username.clone(),
            password.clone(),
        );

        let cam = IpcManager::new(id as i64, client);
        store.add(cam).await?;
    }

    Ok(store)
}

async fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let store = store_from_env().await?;
    let mut input = String::new();

    loop {
        cli_get_input(&mut input, "> ")?;

        match input.as_str() {
            "print" | "p" => {
                for s in store.list().await {
                    client_print(s).await?;
                    break;
                }
            }
            // "file" | "f" => {
            //     println!("whoops");
            // }
            // "keepalive" | "k" => {
            //     let cam: &mut CameraState = if let Some(ref mut cam) = cam {
            //         cam
            //     } else {
            //         println!("Error: No Camera");
            //         continue;
            //     };
            //
            //     match cam.user.keep_alive_or_login(&mut cam.client).await {
            //         Ok(_) => println!("Keep Alive: true"),
            //         Err(err) => println!("Error: {:?}", err),
            //     }
            // }
            // "login" | "l" => {
            //     let cam: &mut CameraState = if let Some(ref mut cam) = cam {
            //         cam
            //     } else {
            //         println!("Error: No Camera");
            //         continue;
            //     };
            //
            //     match cam.user.login(&mut cam.client).await {
            //         Ok(_) => println!("Login: true"),
            //         Err(err) => println!("Error: {:?}", err),
            //     }
            // }
            // "logout" | "L" => {
            //     let cam: &mut CameraState = if let Some(ref mut cam) = cam {
            //         cam
            //     } else {
            //         println!("Error: No Camera");
            //         continue;
            //     };
            //
            //     match User::logout(&mut cam.client).await {
            //         Ok(_) => println!("Logout: true"),
            //         Err(err) => println!("Error: {:?}", err),
            //     }
            // }
            // "config" | "c" => {
            //     let cam: &mut CameraState = if let Some(ref mut cam) = cam {
            //         cam
            //     } else {
            //         println!("Error: No Camera");
            //         continue;
            //     };
            //
            //     println!("{:?}", cam.client.config)
            // }
            "quit" | "q" => {
                break;
            }
            _ => println!("Error: Unknown Command: {input}",),
        }
    }

    store.reset().await;

    Ok(())
}
