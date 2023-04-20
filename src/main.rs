use std::io;
use std::io::Write;

use anyhow::{bail, Result};
use dotenvy::dotenv;
use sqlx::SqlitePool;

use ipcmanview::core::{self, CameraManager, CameraManagerStore};
use ipcmanview::models::{Camera, CameraCreate};
use ipcmanview::procs::setup_database;
use ipcmanview::rpc::utils::new_client;
use ipcmanview::{client_print, procs, require_env};
use ipcmanview::{models, rpc};

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
    let pool =
        setup_database(&std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string()))
            .await?;

    let id = 1;
    let client = new_client();

    let man = match Camera::find(&pool, id).await {
        Ok(cam) => {
            println!("Found manager: {}", id);
            cam.new_camera_manager(client)
        }
        Err(err) => {
            println!("Creating manager: {}", err);

            CameraCreate {
                ip: require_env("IPCMANVIEW_IP")?,
                username: require_env("IPCMANVIEW_USERNAME")?,
                password: require_env("IPCMANVIEW_PASSWORD")?,
            }
            .create(&pool)
            .await?
            .new_camera_manager(client)
        }
    };

    let res = db_run(&man, &pool).await;

    man.close().await;

    res
}

async fn db_run(
    man: &core::CameraManager,
    pool: &SqlitePool,
) -> Result<(), Box<dyn std::error::Error>> {
    core::CameraDetail::get(man)
        .await?
        .save(pool, man.id)
        .await?;
    core::CameraSoftwareVersion::get(man)
        .await?
        .save(pool, man.id)
        .await?;
    procs::scan_task_run(pool, man, models::ScanTaskBuilder::new(man.id).full()).await?;
    let cursor_scan_task = models::CameraScanCursor::find(pool, man.id)
        .await?
        .to_scan_task();
    procs::scan_task_run(pool, man, cursor_scan_task).await?;

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

async fn store_from_env() -> Result<CameraManagerStore> {
    let password = require_env("IPCMANVIEW_PASSWORD")?;
    let ips = require_env("IPCMANVIEW_IPS")?;
    let ips = ips.trim();
    if ips == "" {
        bail!("IP_IPS is empty")
    }
    let ips = ips.split(" ");
    let username = std::env::var("IPCMANVIEW_USERNAME").unwrap_or("admin".to_string());

    let store = CameraManagerStore::new();
    let client = new_client();
    for (id, ip) in ips.enumerate() {
        let client = rpc::Client::new(
            client.clone(),
            ip.to_string(),
            username.clone(),
            password.clone(),
        );

        let cam = CameraManager::new(id as i64, client);
        store.add(cam)?;
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
                for s in store.list() {
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
