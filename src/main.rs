use std::net::SocketAddr;

use dotenvy::dotenv;
use ipcmanview::{db, http, ipc::IpcManagerStore};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    // Setup
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or("sqlite://ipcmanview.db".to_string());
    let pool = db::new(&database_url).await.unwrap();
    let store = IpcManagerStore::new(&pool).await.unwrap();
    let client = reqwest::ClientBuilder::new()
        .no_deflate()
        // HACK: prevent connection reset when requesting too fast
        .pool_max_idle_per_host(0)
        .build()
        .unwrap();

    // App
    let app = http::app(http::AppState {
        pool,
        store: store.clone(),
        client,
    });

    // Listen
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(store))
        .await
        .unwrap();
}

pub async fn shutdown_signal(store: IpcManagerStore) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
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

    store.reset().await;

    println!("done");
}
