mod cli;
mod server;

use cli::{ServerContext, parse};

#[tokio::main]
async fn main() {
    let server_context: ServerContext = parse();
    server::start(server_context).await.unwrap();
}
