use dotenvy::dotenv;
use ipcmanview_cli::run;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let ip = std::env::var("IPC_IP").expect("IPC_IP not set");
    let username = std::env::var("IPC_USERNAME").expect("IPC_USERNAME not set");
    let password = std::env::var("IPC_PASSWORD").expect("IPC_PASSWORD not set");

    let ips = ip.split(",");

    for ip in ips {
        let mut client = dahua_rpc::Client::new(
            dahua_rpc::recommended_reqwest_client_builder()
                .build()
                .expect("failed to create reqwest client"),
            ip.to_string(),
            username.to_string(),
            password.to_string(),
        );

        eprintln!("++++++++++ {ip}");

        let res = run(&mut client).await;
        client.logout().await;

        if let Err(err) = res {
            eprintln!("---------- {ip}: {err}");
        }

        eprintln!("")
    }
}
