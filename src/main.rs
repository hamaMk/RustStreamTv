use axum::{Router, routing::get, serve};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::api::{get_media_list, get_device_info, stream_media};
use crate::state::AppState;

mod config;
mod api;
mod media;
mod state;

/// Entry point for the RustStreamTv media server.
///
/// This server exposes the following HTTP GET endpoints:
/// - `GET /media` – Returns a list of available media files.
/// - `GET /media/:filename` – Streams a specified media file.
/// - `GET /device-info` – Returns metadata about the current device.
///
/// The configuration is loaded from a TOML file (default: `config/default.toml`)
/// which specifies the port to bind to and a friendly device name.
///
/// # Example `config/default.toml`:
/// ```toml
/// folder = "media"
/// port = 8080
/// device_name = "MyRustPC"
/// ```
///
/// # Panics
/// - If the config file fails to load.
/// - If the Axum server fails to bind or serve.
#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let config = config::Config::from_file("config/default.toml").expect("Failed to load config");

    let state = AppState {
        config: Arc::new(config),
    };

    let app = Router::new()
        .route("/media", get(get_media_list))
        .route("/media/:filename", get(stream_media))
        .route("/device-info", get(get_device_info))
        .with_state(state.clone());;

    let addr = SocketAddr::from(([0, 0, 0, 0], state.config.port));
    println!("RustStream [{}] listening on {}", state.config.device_name, addr);


    let listener = TcpListener::bind(addr).await?;
    serve(listener, app).await?;

    Ok(())
}