use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use dotenvy::dotenv;
use ipcmanview::{db, ipc::IpcStore};
use ipcmanview_server::http;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    // Config
    let config_database_url =
        std::env::var("DATABASE_URL").unwrap_or("sqlite://ipcmanview.db".to_string());
    let config_port: u16 = std::env::var("HTTP_PORT")
        .map_or_else(|_| 8000, |port| port.parse().expect("Invalid HTTP_PORT"));
    let config_ip: IpAddr = std::env::var("HTTP_ADDRESS").map_or_else(
        |_| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        |ip| ip.parse().expect("Invalid HTTP_ADDRESS"),
    );

    // Setup
    let pool = db::new(&config_database_url)
        .await
        .expect("Failed to open database");
    let store = IpcStore::new(pool.clone())
        .await
        .expect("Failed to create store");
    let client = reqwest::ClientBuilder::new()
        .no_deflate()
        // HACK: prevent connection reset when requesting too fast
        .pool_max_idle_per_host(0)
        .build()
        .expect("Failed to create reqwest client");

    // App
    let app = http::app(http::AppState {
        pool,
        store: store.clone(),
        client,
    });

    // Listen
    let addr = SocketAddr::from((config_ip, config_port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(store))
        .await
        .unwrap();
}

pub async fn shutdown_signal(store: IpcStore) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");

    store.shutdown().await;

    println!("done");
}
