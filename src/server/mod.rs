mod handler;
mod state;

use crate::{cli::ServerContext, server::state::AppState};
use axum::{routing::get, Router};
use colored::{self, Colorize};
use std::{io::Error, net::SocketAddr, process::exit};
use tokio::net::TcpListener;

pub async fn start(server_context: ServerContext) -> Result<(), Error> {
    let port = server_context.port;
    let app_state = AppState::from(server_context);

    let app = Router::new()
        .route("/files", get(handler::list_files))
        .route("/file/{path}", get(handler::download_file))
        .with_state(app_state);
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
