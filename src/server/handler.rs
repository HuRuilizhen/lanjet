use axum::extract::Path as AxumPath;

pub async fn list_files() -> String {
    "receive list files request".to_string()
}

pub async fn download_file(AxumPath(path): AxumPath<String>) -> String {
    format!("receive download file {} request", path).to_string()
}
