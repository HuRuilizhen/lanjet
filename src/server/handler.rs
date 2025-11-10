use super::AppState;
use axum::extract::Path as AxumPath;
use axum::extract::State as AxumState;

pub async fn list_files(AxumState(app_state): AxumState<AppState>) -> String {
    "receive list files request".to_string()
}

pub async fn download_file(
    AxumState(app_state): AxumState<AppState>,
    AxumPath(path): AxumPath<String>,
) -> String {
    format!("receive download file {} request", path).to_string()
}
