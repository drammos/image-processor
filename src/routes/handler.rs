use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::AppError;
use std::sync::Arc;
use std::{fs::File, io::Write, path::PathBuf};
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct AppState {
    tx: broadcast::Sender<String>,
}

pub fn routes() -> Router {
    let (tx, _rx) = broadcast::channel(100);
    Arc::new(AppState { tx });

    Router::new()
        .route("/health-check", get(health_check))
        .route("/api/upload", post(upload))
}
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Service is healthy")
}

pub async fn upload(mut multipart: Multipart) -> Result<(), AppError> {
    let upload_dir = PathBuf::from("uploads");
    std::fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("default.bin").to_string();
        println!("File name: {}", file_name);
        let unique_file_name = format!("{}_{}", Uuid::new_v4(), file_name);
        let file_path = upload_dir.join(unique_file_name);

        let mut file = File::create(file_path).expect("Failed to create file");
        let data = field.bytes().await.unwrap();
        file.write_all(&data).expect("Failed to write to file");
    }

    Ok(())
}
