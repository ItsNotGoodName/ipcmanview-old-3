use anyhow::{bail, Result};
use dotenvy::dotenv;
use ipcmanview::db;
use ipcmanview::rpc::utils::new_client;
use ipcmanview::rpc::{self, rpclogin};
use ipcmanview::{client_print, require_env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    match std::env::args().skip(1).next() {
        Some(command) => match command.as_str() {
            "cli" => cli().await,
            "db" => db().await,
            "http" => http().await,
            _ => Err("Invalid Command".into()),
        },
        None => Err("No Command".into()),
    }
}

async fn http() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

use ipcmanview::core::{self, CameraManager, CameraManagerStore, CameraState};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, SqliteConnection};
use std::str::FromStr;

async fn db() -> Result<(), Box<dyn std::error::Error>> {
    // Connect
    let mut pool = SqliteConnectOptions::from_str(
        &std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string()),
    )?
    .create_if_missing(true)
    .connect()
    .await?;

    // Migrate
    sqlx::migrate!().run(&mut pool).await?;
    db::camera_tasks_delete_running(&mut pool).await?;

    let client = new_client();

    let man = match db::camera_manager_get(&mut pool, 1, client.clone()).await {
        Ok(cam) => cam,
        Err(err) => {
            println!("Creating client due to err: {}", err);
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
    ipcmanview::scan::full(&mut pool, man).await?;

    Ok(())
}

use std::io;
use std::io::Write;

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
