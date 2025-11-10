use crate::cli::ServerContext;
use pathdiff::diff_paths;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppState {
    pub base_dir: PathBuf,
    pub path_set: HashSet<PathBuf>,
    pub files: Vec<PathBuf>,
}

impl From<ServerContext> for AppState {
    fn from(server_context: ServerContext) -> Self {
        let mut path_set: HashSet<PathBuf> = HashSet::new();

        for file_path in &server_context.files {
            let relative_path: PathBuf = diff_paths(file_path, &server_context.base_dir).unwrap();
            path_set.insert(relative_path);
        }

        Self {
            base_dir: server_context.base_dir,
            files: server_context.files,
            path_set: path_set,
        }
    }
}
