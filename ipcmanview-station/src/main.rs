use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use dotenvy::dotenv;
use ipcmanview::{db, ipc::IpcStore};
use ipcmanview_station::{api, app::AppState, mpa};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "ipcmanview_station=debug,ipcmanview=debug,sqlx=debug,tower_http=debug,axum::rejection=trace"
                    .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    // Config
    let config_database_url =
        std::env::var("DATABASE_URL").unwrap_or("sqlite://ipcmanview.db".to_string());
    let config_socket_address: SocketAddr = std::env::var("HTTP_ADDRESS").map_or_else(
        |_| "127.0.0.1:8000".parse().unwrap(),
        |address| address.parse().expect("Invalid HTTP_ADDRESS"),
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
    let app_state = AppState {
        pool,
        store: store.clone(),
        client,
    };
    let app = mpa::router()
        .nest("/api", api::router())
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    // Listen
    tracing::info!("listening on {}", config_socket_address);
    axum::Server::bind(&config_socket_address)
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
