mod handler;
mod state;

use crate::banner::show_banner;
use crate::cli::{BannerContext, ServerContext};
use axum::{Router, routing::get};
use colored::{self, Colorize};
use state::AppState;
use std::{io::Error, process::exit};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::fmt;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for <CTRL+C>");

    tracing::info!("🛑 Received shutdown signal, shutting down...");
}

pub async fn start(
    banner_context: BannerContext,
    server_context: ServerContext,
) -> Result<(), Error> {
    let addr = server_context.addr;
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
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("{}: {}", "error".red().bold(), err);
            exit(0);
        }
    };

    show_banner(banner_context);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
}
