use dotenv::dotenv;
use ipcmanview::man_print;
use ipcmanview::rpc::{self, mediafilefind, rpclogin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    match std::env::args().skip(1).next() {
        Some(command) => match command.as_str() {
            "cli" => cli(),
            "debug" => debug(),
            _ => Err("Invalid Command".into()),
        },
        None => Ok(println!("No Command")),
    }
}

fn get_agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build()
}

use std::io;
use std::io::Write;
use std::time::Duration;

fn cli_get_input(input: &mut String, message: &str) -> Result<(), io::Error> {
    print!("{}", message);
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(input)?;
    *input = input.trim().to_string();
    Ok(())
}

fn cli_add(
    agent: ureq::Agent,
    ip: String,
    username: String,
    password: String,
) -> Option<rpclogin::Manager> {
    let client = rpc::Client::new(ip.clone(), agent.clone());
    let mut man = rpclogin::Manager::new(client, username, password);
    println!("...Logging in to {ip}");
    match man.login_or_relogin() {
        Ok(_) => Some(man),
        Err(err) => {
            println!("Error: {:?}", err);
            None
        }
    }
}

fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let agent = get_agent();
    let mut man: Option<rpclogin::Manager> = if let Some(ip) = std::env::var("IPCMANVIEW_IP").ok() {
        let username =
            std::env::var("IPCMANVIEW_USERNAME").map_err(|_| "IPCMANVIEW_USERNAME not set")?;
        let password =
            std::env::var("IPCMANVIEW_PASSWORD").map_err(|_| "IPCMANVIEW_PASSWORD not set")?;
        cli_add(agent.clone(), ip, username, password)
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

                man = cli_add(agent.clone(), ip, username, password);
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

                match man.login_or_relogin() {
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
    let agent: ureq::Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

    let password = std::env::var("IPCMANVIEW_PASSWORD").map_err(|_| "IPCVIEW_PASSWORD not set")?;
    let ips = std::env::var("IPCMANVIEW_IPS").map_err(|_| "IPCMANVIEW_IPS not set")?;
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
    let mut man = rpclogin::Manager::new(rpc::Client::new(ip, agent), username, password);

    man_print(&mut man);

    man.logout().unwrap();
}
