use std::path::PathBuf;

#[derive(Debug)]
pub struct ServerContext {
    pub base_dir: PathBuf,
    pub files: Vec<PathBuf>,
    pub total_size: u64,
    pub port: u16,
}
