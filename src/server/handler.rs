use super::{state::AppState, style::inline_css, util::file_icon};
use crate::util::human_size;
use axum::Json;
use axum::body::Body;
use axum::extract::Path as AxumPath;
use axum::extract::State as AxumState;
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Response};
use maud::{Markup, html};
use mime_guess::from_path;
use serde_json::json;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;
use urlencoding::encode;

pub async fn index_page(AxumState(state): AxumState<AppState>) -> impl IntoResponse {
    let files = &state.path_set;
    let meta_data = &state.meta_data;

    let markup: Markup = html! {
        html {
            head {
                meta charset="utf-8";
                title { "LanJet" }
                style { (inline_css()) }
            }
            body {
                div class="container" {
                    h1 { "✈️ LanJet" }
                    ul {
                        @for file in files {
                            li {
                                div class="file-name" {
                                    @let encoded = encode(file);

                                    (file_icon(file))
                                    a href=(format!("/file/{}", encoded)) { (file) }
                                }
                                span class="file-size" {
                                    (human_size(meta_data[file].len()))
                                }
                            }
                        }
                    }
                    footer {
                        "Powered by LanJet"
                    }
                }
            }
        }
    };

    Html(markup.into_string())
}

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
    let mime_type = from_path(&absolute_path).first_or_octet_stream();
    let inline = matches!(
        (mime_type.type_().as_str(), mime_type.subtype().as_str()),
        ("image", _)
            | ("text", _)
            | ("application", "pdf")
            | ("application", "json")
            | ("application", "xml")
    );
    let disposition = if inline {
        format!("inline; filename=\"{}\"", path)
    } else {
        format!("attachment; filename=\"{}\"", path)
    };

    Response::builder()
        .header(header::CONTENT_TYPE, mime_type.as_ref())
        .header(header::CONTENT_DISPOSITION, disposition)
        .body(body)
        .unwrap()
}
