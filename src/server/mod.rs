mod handler;

use crate::cli::ServerContext;
use axum::{Router, routing::get};
use std::{io::Error, net::SocketAddr};

pub async fn start(server_context: ServerContext) -> Result<(), Error> {
    let app = Router::new()
        .route("/files", get(handler::list_files))
        .route("/file/{path}", get(handler::download_file));
    let addr = SocketAddr::from(([127, 0, 0, 1], server_context.port));
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await
}
