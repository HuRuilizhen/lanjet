mod cli;
mod context;
mod server;

use cli::parse;
use context::ServerContext;

#[tokio::main]
async fn main() {
    let server_context: ServerContext = parse();
    server::start(server_context).await.unwrap();
}
