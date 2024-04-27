use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tracing::{info, warn};

struct AppState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let socket = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Serving {:?} on {}", path, socket);
    let app_state = Arc::new(AppState { path });

    let app = Router::new()
        .route("/", get(root))
        .route("/*path", get(file_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(socket).await?;
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn file_handler(
    State(app_state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let file_path = std::path::Path::new(&app_state.path).join(path);
    if !file_path.exists() {
        warn!("File not found: {:?}", file_path);
        return (StatusCode::NOT_FOUND, "File not found".to_string());
    }
    match tokio::fs::read_to_string(&file_path).await {
        Err(e) => {
            warn!("Error reading file: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error reading file".to_string(),
            )
        }
        Ok(content) => {
            info!("File served: {:?}", &file_path);
            (StatusCode::OK, content)
        }
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}
