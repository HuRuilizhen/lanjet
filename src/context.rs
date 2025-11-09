use std::path::PathBuf;

#[derive(Debug)]
pub struct ServerContext {
    pub files: Vec<PathBuf>,
    pub port: u16,
}
