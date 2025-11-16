use crate::cli::ServerContext;
use pathdiff::diff_paths;
use std::collections::{HashMap, HashSet};
use std::fs::Metadata;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppState {
    pub base_dir: PathBuf,
    pub path_set: HashSet<String>,
    pub meta_data: HashMap<String, Metadata>,
}

impl From<ServerContext> for AppState {
    fn from(server_context: ServerContext) -> Self {
        let mut path_set: HashSet<String> = HashSet::new();
        let mut meta_data: HashMap<String, Metadata> = HashMap::new();

        for file_path in &server_context.files {
            let relative_path: PathBuf = diff_paths(file_path, &server_context.base_dir).unwrap();
            let path = relative_path.display().to_string().replace("\\", "/");
            meta_data.insert(path.clone(), file_path.metadata().unwrap());
            path_set.insert(path);
        }

        Self {
            base_dir: server_context.base_dir,
            path_set,
            meta_data,
        }
    }
}
