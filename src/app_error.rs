use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Dotenvy error: {0}")]
    DotenvyError(#[from] dotenvy::Error),
    
    #[error("Env error: {0}")]
    EnvError(#[from] std::env::VarError),
}