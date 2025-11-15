mod banner;
mod cli;
mod matcher;
mod server;

use cli::parse;

#[tokio::main]
async fn main() {
    let (banner_context, server_context) = parse();
    banner::show_banner(banner_context);
    server::start(server_context).await.unwrap();
}
