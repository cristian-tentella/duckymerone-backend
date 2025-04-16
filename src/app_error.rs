use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Dotenvy error: {0}")]
    DotenvyError(#[from] dotenvy::Error),
    
    #[error("Env error: {0}")]
    EnvError(#[from] std::env::VarError),
    
    #[error("Server error: {0}")]
    ServerError(String)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("App error: {:?}", self);

        let body = Json(json!({
            "error": &self.to_string(),
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}