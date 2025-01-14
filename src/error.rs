use {
    axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    },
    serde_json::json,
};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Value not found")]
    NotFound,
    #[error("Configuration Error {0}")]
    Config(#[from] config::ConfigError),
    #[error("Could not start service: {0}")]
    Startup(String),
    #[error("Could not stop service")]
    TcpBind,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Resource not Found"),
            Self::Config(_) | Self::TcpBind | Self::Startup(_) => todo!(),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
