use dotenvy::dotenv;
use ipcmanview::db;
use ipcmanview::rpc::utils::new_client;
use ipcmanview::rpc::{self, mediafilefind, rpclogin};
use ipcmanview::{man_print, require_env};

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

use ipcmanview::core;
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

    let client = new_client();

    // let man = rpclogin::Manager::new(rpc::Client::new(require_env("IPCMANVIEW_IP")?, agent))
    //     .username(require_env("IPCMANVIEW_USERNAME")?)
    //     .password(require_env("IPCMANVIEW_PASSWORD")?)
    //     .unlock();
    // let cam = db::camera_add(&mut pool, man).await?;

    let cam = db::camera_manager_get(&mut pool, 1, client).await?;
    let res = db_run(&cam, pool).await;

    _ = cam.man.lock().unwrap().logout();

    res
}

async fn db_run(
    cam: &core::Camera,
    mut pool: SqliteConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    db::camera_detail_update(&mut pool, cam.id, cam.detail().await?).await?;
    db::camera_software_version_update(&mut pool, cam.id, cam.version().await?).await?;
    let camera_scan = db::camera_scan(
        &mut pool,
        cam,
        chrono::Utc::now() - chrono::Duration::hours(24),
        chrono::Utc::now(),
    )
    .await?;
    println!("CameraScan: {:?}", camera_scan);

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
    let client = new_client();
    let mut man: Option<rpclogin::Manager> = if let Some(ip) = std::env::var("IPCMANVIEW_IP").ok() {
        let username = require_env("IPCMANVIEW_USERNAME")?;
        let password = require_env("IPCMANVIEW_PASSWORD")?;
        Some(
            rpclogin::Manager::new(rpc::Client::new(ip, client.clone()))
                .username(username)
                .password(password)
                .unlock(),
        )
    } else {
        println!("Manager: None");
        None
    };
    let mut input = String::new();

    loop {
        cli_get_input(&mut input, "> ")?;

        match input.as_str() {
            "add" | "a" => {
                if let Some(ref man) = man {
                    println!("Error: already added: {}", man.client.ip);
                    continue;
                }

                cli_get_input(&mut input, "IP: ")?;
                let ip = input.clone();
                cli_get_input(&mut input, "Username: ")?;
                let username = input.clone();
                cli_get_input(&mut input, "Password: ")?;
                let password = input.clone();

                man = Some(
                    rpclogin::Manager::new(rpc::Client::new(ip, client.clone()))
                        .username(username)
                        .password(password)
                        .unlock(),
                )
            }
            "print" | "p" => {
                let man: &mut rpclogin::Manager = if let Some(ref mut man) = man {
                    man
                } else {
                    println!("Error: No Manager");
                    continue;
                };

                man_print(man).await;
            }
            "file" | "f" => {
                let man: &mut rpclogin::Manager = if let Some(ref mut man) = man {
                    man
                } else {
                    println!("Error: No Manager");
                    continue;
                };

                println!("Pictures - Last 24 hours");
                match mediafilefind::find_next_file_info_iterator(
                    man,
                    mediafilefind::Condition::new(
                        chrono::Utc::now() - chrono::Duration::hours(24),
                        chrono::Utc::now(),
                    )
                    .picture(),
                )
                .await
                {
                    Ok(mut iter) => {
                        while let Some(files) = iter.next().await {
                            for file in files {
                                println!("file_path: {}", file.file_path);
                            }
                        }
                    }
                    Err(err) => println!("Error: {:?}", err),
                }
            }
            "keepalive" | "k" => {
                let man: &mut rpclogin::Manager = if let Some(ref mut man) = man {
                    man
                } else {
                    println!("Error: No Manager");
                    continue;
                };

                match man.keep_alive_or_login().await {
                    Ok(sec) => println!("Keep Alive: {sec}"),
                    Err(err) => println!("Error: {:?}", err),
                }
            }
            "login" | "l" => {
                let man: &mut rpclogin::Manager = if let Some(ref mut man) = man {
                    man
                } else {
                    println!("Error: No Manager");
                    continue;
                };

                match man.login().await {
                    Ok(res) => println!("Login: {res}"),
                    Err(err) => println!("Error: {:?}", err),
                }
            }
            "logout" | "L" => {
                let man: &mut rpclogin::Manager = if let Some(ref mut man) = man {
                    man
                } else {
                    println!("Error: No Manager");
                    continue;
                };

                match man.logout().await {
                    Ok(res) => println!("Logout: {res}"),
                    Err(err) => println!("Error: {:?}", err),
                }
            }
            "config" | "c" => {
                let man: &mut rpclogin::Manager = if let Some(ref mut man) = man {
                    man
                } else {
                    println!("Error: No Manager");
                    continue;
                };

                println!("{:?}", man.client.config)
            }
            "quit" | "q" => {
                break;
            }
            _ => println!("Error: Unknown Command: {input}",),
        }
    }

    if let Some(mut man) = man {
        println!("...Logging out {}", man.client.ip);
        if let Err(err) = man.logout().await {
            println!("Error: {ip}: {err:?}", ip = man.client.ip)
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

async fn debug_run(client: reqwest::Client, ip: String, username: String, password: String) {
    let mut man = rpclogin::Manager::new(rpc::Client::new(ip, client))
        .username(username)
        .password(password)
        .unlock();

    man_print(&mut man).await;

    println!("Logout: {:?}", man.logout().await);
}
