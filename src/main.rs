mod banner;
mod cli;
mod matcher;
mod server;
mod util;

use cli::parse;

#[tokio::main]
async fn main() {
    let (banner_context, server_context) = parse();
    server::start(banner_context, server_context).await.unwrap();
}
