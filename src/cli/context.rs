use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ServerContext {
    pub addr: SocketAddr,
    pub base_dir: PathBuf,
    pub files: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct BannerContext {
    pub addr: SocketAddr,
    pub base_dir: PathBuf,
    pub ignore: PathBuf,
    pub files_count: usize,
    pub total_size: u64,
}
