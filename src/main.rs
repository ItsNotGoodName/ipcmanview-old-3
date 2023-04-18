use std::io;
use std::io::Write;

use anyhow::{bail, Result};
use dotenvy::dotenv;

use ipcmanview::core::{self, CameraManager, CameraManagerStore, CameraState};
use ipcmanview::rpc::utils::new_client;
use ipcmanview::rpc::{self, rpclogin};
use ipcmanview::{client_print, connect_database, procs, require_env};
use ipcmanview::{db, models};
use sqlx::SqliteConnection;

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
    let mut pool = connect_database(
        &std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string()),
    )
    .await?;

    let id = 1;
    let client = new_client();

    let man = match db::camera_manager_find(&mut pool, id, client.clone()).await {
        Ok(cam) => {
            println!("Found manager: {}", id);
            cam
        }
        Err(err) => {
            println!("Creating manager: {}", err);

            let user = rpclogin::User::new()
                .username(require_env("IPCMANVIEW_USERNAME")?)
                .password(require_env("IPCMANVIEW_PASSWORD")?)
                .unblock();
            let client = rpc::Client::new(require_env("IPCMANVIEW_IP")?, client.clone());
            let state = CameraState { user, client };

            state.create(&mut pool).await?
        }
    };

    let res = db_run(&man, pool).await;

    man.destroy().await;

    res
}

async fn db_run(
    man: &core::CameraManager,
    mut pool: SqliteConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    core::CameraDetail::get(man)
        .await?
        .save(&mut pool, man.id)
        .await?;
    core::CameraSoftwareVersion::get(man)
        .await?
        .save(&mut pool, man.id)
        .await?;
    procs::scan_task_run(&mut pool, man, models::ScanTaskBuilder::new(man.id).full()).await?;
    let cursor_scan_task = models::CameraScanCursor::find(&mut pool, man.id)
        .await?
        .to_scan_task();
    procs::scan_task_run(&mut pool, man, cursor_scan_task).await?;

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

fn store_from_env() -> Result<CameraManagerStore> {
    let password = require_env("IPCMANVIEW_PASSWORD")?;
    let ips = require_env("IPCMANVIEW_IPS")?;
    let ips = ips.trim();
    if ips == "" {
        bail!("IP_IPS is empty")
    }
    let ips = ips.split(" ");
    let username = std::env::var("IPCMANVIEW_USERNAME").unwrap_or("admin".to_string());

    let mut store = CameraManagerStore::new();
    let cl = new_client();
    for (id, ip) in ips.enumerate() {
        let user = rpclogin::User::new()
            .username(username.clone())
            .password(password.clone())
            .unblock();
        let client = rpc::Client::new(ip.to_string(), cl.clone());
        let cam = CameraManager::new(id as i64, CameraState { user, client });
        store.add(cam)?;
    }

    Ok(store)
}

async fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let store = store_from_env()?;
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

    store.destroy().await;

    Ok(())
}
