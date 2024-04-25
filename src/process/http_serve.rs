use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::path::PathBuf;
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    println_http_cli_logo();
    tracing_subscriber::registry().with(fmt::layer()).init();
    info!("Serveing {:?}, on {}", path, port);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let state = HttpServeState { path: path.clone() };
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", ServeDir::new(path))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(&path);
    info!("Reading file: {:?}", p);
    if !p.exists() {
        return (
            StatusCode::NOT_FOUND,
            format!("File {:?} not found", p.display()),
        );
    }
    // if p.is_dir() {
    //     p = std::path::Path::new(&state.path)
    //         .join(&path)
    //         .join("index.html");
    // }
    match tokio::fs::read_to_string(&p).await {
        Ok(content) => {
            info!("Read {} bytes", content.len());
            (StatusCode::OK, content)
        }
        Err(e) => {
            warn!("Error reading file: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

fn println_http_cli_logo() {
    println!();
    println!(r" _     _____  _____  ____      ____  _     _ ");
    println!(r"/ \ /|/__ __\/__ __\/  __\    /   _\/ \   / \");
    println!(r"| |_||  / \    / \  |  \/|    |  /  | |   | |");
    println!(r"| | ||  | |    | |  |  __/    |  \__| |_/\| |");
    println!(r"\_/ \|  \_/    \_/  \_/       \____/\____/\_/");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_file() {
        let path = PathBuf::from(".");
        let state = HttpServeState { path };
        let (status, body) =
            file_handler(State(Arc::new(state)), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.trim().starts_with("[package]"));
    }
}
