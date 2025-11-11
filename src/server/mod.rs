mod handler;
mod state;

use crate::{cli::ServerContext, server::state::AppState};
use axum::{routing::get, Router};
use colored::{self, Colorize};
use local_ip_address::local_ip;
use std::{io::Error, net::SocketAddr, process::exit};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::fmt;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for <CTRL+C>");

    tracing::info!("🛑 Received shutdown signal, shutting down...");
}

pub async fn start(server_context: ServerContext) -> Result<(), Error> {
    let port = server_context.port;
    let base_dir = server_context.base_dir.canonicalize().unwrap();
    let total_size = server_context.total_size;
    let count = server_context.files.len();
    let app_state = AppState::from(server_context);

    fmt::init();
    let app = Router::new()
        .route("/files", get(handler::list_files))
        .route("/file/{*path}", get(handler::download_file))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("{}: {}", "error".red().bold(), err);
            exit(0);
        }
    };

    info!("🚀 Lanjet service started");
    info!("📂 Base directory: {}", base_dir.display());
    info!(
        "🧮 Serving {} files ({:.2} KB total)",
        count,
        total_size as f64 / 1024.0
    );
    info!("🌐 Serving at http://{}:{}", "127.0.0.1", port);
    info!("🌐 Serving at http://{}:{}", local_ip().unwrap(), port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
}
