use dotenvy::dotenv;
use ipcmanview::db;
use ipcmanview::rpc::rpclogin::User;
use ipcmanview::rpc::utils::new_client;
use ipcmanview::rpc::{self, rpclogin};
use ipcmanview::{client_print, require_env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    match std::env::args().skip(1).next() {
        Some(command) => match command.as_str() {
            "cli" => cli().await,
            "debug" => debug().await,
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

use ipcmanview::core::{self, CameraState};
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

    let cam = match db::camera_manager_get(&mut pool, 1, client.clone()).await {
        Ok(cam) => cam,
        Err(err) => {
            println!("Creating client due to err: {}", err);
            let user = rpclogin::User::new()
                .username(require_env("IPCMANVIEW_USERNAME")?)
                .password(require_env("IPCMANVIEW_PASSWORD")?)
                .unblock();
            let client = rpc::Client::new(require_env("IPCMANVIEW_IP")?, client.clone());
            let state = CameraState { user, client };
            db::camera_add(&mut pool, state).await?
        }
    };

    let res = db_run(&cam, pool).await;

    cam.logout().await.ok();

    res
}

async fn db_run(
    cam: &core::Camera,
    mut pool: SqliteConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    db::camera_detail_update(&mut pool, cam.id, core::CameraDetail::get(cam).await?).await?;
    db::camera_software_version_update(
        &mut pool,
        cam.id,
        core::CameraSoftwareVersion::get(cam).await?,
    )
    .await?;
    ipcmanview::scan::full(&mut pool, cam).await?;

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

async fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let mut cam: Option<CameraState> = if let Some(ip) = std::env::var("IPCMANVIEW_IP").ok() {
        let username = require_env("IPCMANVIEW_USERNAME")?;
        let password = require_env("IPCMANVIEW_PASSWORD")?;
        let client = rpc::Client::new(ip, new_client());
        let user = rpclogin::User::new()
            .username(username)
            .password(password)
            .unblock();
        Some(CameraState { client, user })
    } else {
        println!("Camera: None");
        None
    };
    let mut input = String::new();

    loop {
        cli_get_input(&mut input, "> ")?;

        match input.as_str() {
            "add" | "a" => {
                if let Some(ref cam) = cam {
                    println!("Error: already added: {}", cam.client.ip);
                    continue;
                }

                cli_get_input(&mut input, "IP: ")?;
                let ip = input.clone();
                cli_get_input(&mut input, "Username: ")?;
                let username = input.clone();
                cli_get_input(&mut input, "Password: ")?;
                let password = input.clone();
                let client = rpc::Client::new(ip, new_client());
                let user = rpclogin::User::new()
                    .username(username)
                    .password(password)
                    .unblock();
                cam = Some(CameraState { client, user })
            }
            "print" | "p" => {
                let cam: &mut CameraState = if let Some(ref mut cam) = cam {
                    cam
                } else {
                    println!("Error: No Camera");
                    continue;
                };

                client_print(&mut cam.client).await;
            }
            "file" | "f" => {
                println!("whoops");
            }
            "keepalive" | "k" => {
                let cam: &mut CameraState = if let Some(ref mut cam) = cam {
                    cam
                } else {
                    println!("Error: No Camera");
                    continue;
                };

                match cam.user.keep_alive_or_login(&mut cam.client).await {
                    Ok(_) => println!("Keep Alive: true"),
                    Err(err) => println!("Error: {:?}", err),
                }
            }
            "login" | "l" => {
                let cam: &mut CameraState = if let Some(ref mut cam) = cam {
                    cam
                } else {
                    println!("Error: No Camera");
                    continue;
                };

                match cam.user.login(&mut cam.client).await {
                    Ok(_) => println!("Login: true"),
                    Err(err) => println!("Error: {:?}", err),
                }
            }
            "logout" | "L" => {
                let cam: &mut CameraState = if let Some(ref mut cam) = cam {
                    cam
                } else {
                    println!("Error: No Camera");
                    continue;
                };

                match User::logout(&mut cam.client).await {
                    Ok(_) => println!("Logout: true"),
                    Err(err) => println!("Error: {:?}", err),
                }
            }
            "config" | "c" => {
                let cam: &mut CameraState = if let Some(ref mut cam) = cam {
                    cam
                } else {
                    println!("Error: No Camera");
                    continue;
                };

                println!("{:?}", cam.client.config)
            }
            "quit" | "q" => {
                break;
            }
            _ => println!("Error: Unknown Command: {input}",),
        }
    }

    if let Some(mut cam) = cam {
        println!("...Logging out {}", cam.client.ip);
        if let Err(err) = User::logout(&mut cam.client).await {
            println!("Error: {ip}: {err:?}", ip = cam.client.ip)
        }
    }

    Ok(())
}

async fn debug() -> Result<(), Box<dyn std::error::Error>> {
    let client = new_client();

    let password = require_env("IPCMANVIEW_PASSWORD")?;
    let ips = require_env("IPCMANVIEW_IPS")?;
    let ips = ips.trim();
    if ips == "" {
        return Err("IP_IPS is empty".into());
    }
    let ips = ips.split(" ");
    let username = std::env::var("IPCMANVIEW_USERNAME").unwrap_or("admin".to_string());

    for i in ips {
        println!("----------- {}", i);
        debug_run(
            client.clone(),
            String::from(i),
            username.clone(),
            password.clone(),
        )
        .await;
    }

    Ok(())
}

async fn debug_run(_client: reqwest::Client, _ip: String, _username: String, _password: String) {
    // let mut man = rpclogin::Manager::new()
    //     .username(username)
    //     .password(password)
    //     .unblock();
    // rpc::Client::new(ip, client);
    //
    // man_print(&mut man).await;
    // println!("Logout: {:?}", man.logout().await);
}
