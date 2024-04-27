use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

struct AppState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let socket = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Serving {:?} on {}", path, socket);
    let app_state = Arc::new(AppState { path: path.clone() });

    let service = ServeDir::new(path);

    let app = Router::new()
        .route("/", get(dir_handler))
        .route("/*path", get(file_handler))
        .nest_service("/file", service)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(socket).await?;
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn dir_handler(State(app_state): State<Arc<AppState>>) -> (StatusCode, String) {
    let mut content = String::new();
    content.push_str(
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
          <title>Index of the document</title>
        </head>
        <body>

        <ul>

        "#,
    );
    let entries = tokio::fs::read_dir(std::path::Path::new(&app_state.path)).await;
    match entries {
        Err(e) => {
            warn!("Error reading directory: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error reading directory".to_string(),
            );
        }
        Ok(mut entries) => {
            while let Ok(entry) = entries.next_entry().await {
                if entry.is_none() {
                    break;
                }
                match entry {
                    Some(entry) => {
                        let path = entry.path();
                        let name = path.file_name().unwrap().to_string_lossy().to_string();
                        content.push_str(&format!("<li><a href=\"{}/\">{}/</a></li>", name, name));
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
    }

    content.push_str(
        r##"
        </ul>
        </body>
        </html>
        "##,
    );
    (StatusCode::OK, content)
}

async fn file_handler(
    State(app_state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let file_path = std::path::Path::new(&app_state.path).join(path);
    info!("File requested: {:?}", file_path);

    if file_path.is_dir() {
        let app_state = Arc::new(AppState { path: file_path });
        return dir_handler(State(app_state)).await;
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let path = "Cargo.toml".to_string();
        let app_state = Arc::new(AppState {
            path: PathBuf::from("."),
        });
        let (state_code, content) = file_handler(State(app_state), Path(path)).await;
        assert_eq!(state_code, StatusCode::OK);
        assert!(content.trim().contains("[package]"));
    }
}
