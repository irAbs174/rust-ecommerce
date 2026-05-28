use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error")]
    InternalServerError,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Insufficient stock")]
    InsufficientStock,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(ref e) => {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error occurred".to_string(),
                )
            }
            AppError::AuthenticationFailed => (
                StatusCode::UNAUTHORIZED,
                "Authentication failed".to_string(),
            ),
            AppError::Unauthorized => (StatusCode::FORBIDDEN, "Unauthorized".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::InvalidInput(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Conflict(ref msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),
            AppError::InsufficientStock => (StatusCode::BAD_REQUEST, "Insufficient stock".to_string()),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
