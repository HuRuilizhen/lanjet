mod handler;
mod state;

use crate::{cli::ServerContext, server::state::AppState};
use axum::{routing::get, Router};
use colored::{self, Colorize};
use std::{io::Error, net::SocketAddr, process::exit};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::fmt;

pub async fn start(server_context: ServerContext) -> Result<(), Error> {
    let port = server_context.port;
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

    axum::serve(listener, app).await
}
