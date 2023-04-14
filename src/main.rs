use dotenvy::dotenv;
use ipcmanview::rpc::{self, mediafilefind, rpclogin};
use ipcmanview::{db, new_agent};
use ipcmanview::{man_print, require_env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    match std::env::args().skip(1).next() {
        Some(command) => match command.as_str() {
            "cli" => cli(),
            "debug" => debug(),
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

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::ConnectOptions;
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

    let agent = new_agent();

    let man = rpclogin::Manager::new(rpc::Client::new(require_env("IPCMANVIEW_IP")?, agent))
        .username(require_env("IPCMANVIEW_USERNAME")?)
        .password(require_env("IPCMANVIEW_PASSWORD")?)
        .unlock();

    let mut cam = db::camera_add(&mut pool, man).await?;

    // let mut cam = db::camera_manager_get(&mut pool, 1, agent).await?;

    println!("{}", cam.id);

    db::camera_detail_update(&mut pool, &mut cam).await?;
    db::camera_software_version_update(&mut pool, &mut cam).await?;

    cam.man.logout()?;

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

fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let agent = new_agent();
    let mut man: Option<rpclogin::Manager> = if let Some(ip) = std::env::var("IPCMANVIEW_IP").ok() {
        let username = require_env("IPCMANVIEW_USERNAME")?;
        let password = require_env("IPCMANVIEW_PASSWORD")?;
        Some(
            rpclogin::Manager::new(rpc::Client::new(ip, agent.clone()))
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
                    rpclogin::Manager::new(rpc::Client::new(ip, agent.clone()))
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

                man_print(man);
            }
            "file" | "f" => {
                let man: &mut rpclogin::Manager = if let Some(ref mut man) = man {
                    man
                } else {
                    println!("Error: No Manager");
                    continue;
                };

                if let Err(err) = man.keep_alive_or_login() {
                    println!("Error: keep_alive: {:?}", err);
                    continue;
                }

                println!("Pictures - Last 24 hours");
                match mediafilefind::find_next_file_info_iterator(
                    &mut man.client,
                    mediafilefind::Condition::new(
                        chrono::Local::now().naive_local() - chrono::Duration::hours(24),
                        chrono::Local::now().naive_local(),
                    )
                    .picture(),
                ) {
                    Ok(iter) => {
                        for file in iter {
                            println!("file_path: {}", file.file_path);
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

                match man.keep_alive_or_login() {
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

                match man.login() {
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

                match man.logout() {
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
        if let Err(err) = man.logout() {
            println!("Error: {ip}: {err:?}", ip = man.client.ip)
        }
    }

    Ok(())
}

fn debug() -> Result<(), Box<dyn std::error::Error>> {
    let agent = new_agent();

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
            agent.clone(),
            String::from(i),
            username.clone(),
            password.clone(),
        );
    }

    Ok(())
}

fn debug_run(agent: ureq::Agent, ip: String, username: String, password: String) {
    let mut man = rpclogin::Manager::new(rpc::Client::new(ip, agent))
        .username(username)
        .password(password)
        .unlock();

    man_print(&mut man);

    println!("Logout: {:?}", man.logout());
}
