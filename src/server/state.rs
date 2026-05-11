use crate::cli::ServerContext;
use ipnet::IpNet;
use pathdiff::diff_paths;
use std::collections::{HashMap, HashSet};
use std::fs::Metadata;
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppState {
    pub base_dir: PathBuf,
    pub path_set: HashSet<String>,
    pub meta_data: HashMap<String, Metadata>,
    pub allow_ips: Vec<IpNet>,
    pub deny_ips: Vec<IpNet>,
}

impl AppState {
    pub fn is_ip_allowed(&self, ip: IpAddr) -> bool {
        if !self.deny_ips.is_empty() && self.deny_ips.iter().any(|rule| rule.contains(&ip)) {
            return false;
        }

        if self.allow_ips.is_empty() {
            return true;
        }

        self.allow_ips.iter().any(|rule| rule.contains(&ip))
    }
}

impl From<ServerContext> for AppState {
    fn from(server_context: ServerContext) -> Self {
        let mut path_set: HashSet<String> = HashSet::new();
        let mut meta_data: HashMap<String, Metadata> = HashMap::new();

        for file_path in &server_context.files {
            let relative_path: PathBuf = diff_paths(file_path, &server_context.base_dir).unwrap();
            let path = relative_path.display().to_string().replace("\\", "/");
            if let Ok(metadata) = file_path.metadata() {
                meta_data.insert(path.clone(), metadata);
                path_set.insert(path);
            }
        }

        Self {
            base_dir: server_context.base_dir,
            path_set,
            meta_data,
            allow_ips: server_context.allow_ips,
            deny_ips: server_context.deny_ips,
        }
    }
}
