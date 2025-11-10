use super::AppState;
use axum::Json;
use axum::body::Body;
use axum::extract::Path as AxumPath;
use axum::extract::State as AxumState;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;

pub async fn list_files(AxumState(app_state): AxumState<AppState>) -> impl IntoResponse {
    Json(json!(app_state.path_set))
}

pub async fn download_file(
    AxumState(app_state): AxumState<AppState>,
    AxumPath(path): AxumPath<String>,
) -> impl IntoResponse {
    if !app_state.path_set.contains(&path) {
        return (StatusCode::NOT_FOUND, "File not found".to_string()).into_response();
    }

    let path_buf = PathBuf::from(&path);
    let absolute_path = app_state.base_dir.join(&path_buf);

    let file = match tokio::fs::File::open(&absolute_path).await {
        Ok(f) => f,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Could not open file").into_response();
        }
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Response::builder()
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", path),
        )
        .body(body)
        .unwrap()
}
