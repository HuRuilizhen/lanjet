use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ServerContext {
    pub addr: SocketAddr,
    pub base_dir: PathBuf,
    pub files: Vec<PathBuf>,
    pub ignore: PathBuf,
    pub total_size: u64,
}
