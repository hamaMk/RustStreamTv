//! HTTP handlers for the media-streaming API.
//!
//! The module exposes three endpoints:
//! 1. **GET /device-info** – returns basic metadata about the server device
//! 2. **GET /media** – returns a JSON list of all discovered media files
//! 3. **GET /media/:filename** – streams an individual media file back to the client
//!
//! All handlers rely on [`AppState`](crate::state::AppState) to access runtime
//! configuration such as the media root folder and the device name.

use crate::media::scan_media_files;
use crate::state::AppState;

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
};
use mime_guess::MimeGuess;
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

/// GET `/device-info`
/// Returns the configured device name plus the server’s OS platform.
///
/// ### Success response (HTTP 200)
/// ```json
/// { "name": "RustStream-PC", "platform": "linux" }
/// ```
pub async fn get_device_info(State(state): State<AppState>) -> impl IntoResponse {
    let device_name = &state.config.device_name;
    Json(serde_json::json!({
        "name": device_name,
        "platform": std::env::consts::OS
    }))
}

/// GET `/media`
/// Recursively scans the configured media folder and returns a JSON array of
/// [`MediaFile`](crate::media::MediaFile) objects.
///
/// ### Success response (HTTP 200)
/// ```json
/// [
///   { "name": "video.mp4", "size": 1057932, "extension": "mp4", "path": "/media/video.mp4" }
/// ]
/// ```
///
/// ### Error response (HTTP 500)
/// Returned when the server cannot read the folder or file metadata.
pub async fn get_media_list(State(state): State<AppState>) -> impl IntoResponse {
    let folder = &state.config.folder;
    match scan_media_files(folder) {
        Ok(files) => Json(files).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to scan media directory",
        )
            .into_response(),
    }
}

/// GET `/media/:filename`
/// Streams the requested file back to the client in chunks.
///
/// * Returns **HTTP 200** with a streaming body on success.
/// * Returns **HTTP 404** if the file is missing.
/// * Returns **HTTP 500** if the file cannot be opened.
///
/// The `Content-Type` header is inferred via [`mime_guess`].
pub async fn stream_media(
    Path(filename): Path<String>,
    State(state): State<AppState>,
) -> Response {
    // Build absolute path to the requested file
    let folder = &state.config.folder;
    let filepath = PathBuf::from(folder).join(&filename);

    // 404 early if it doesn’t exist
    if !filepath.exists() {
        return (StatusCode::NOT_FOUND, "File not found").into_response();
    }

    // Open file asynchronously
    match File::open(&filepath).await {
        Ok(file) => {
            // Convert to a stream of bytes
            let stream = ReaderStream::new(file);
            let body_stream = Body::from_stream(stream);

            // Build headers
            let mime = MimeGuess::from_path(&filepath).first_or_octet_stream();

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.to_string())
                .body(body_stream)
                .unwrap()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not read media file",
        )
            .into_response(),
    }
}
