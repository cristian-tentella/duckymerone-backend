mod app_error;
mod db;
mod routes;

use app_error::AppError;
use axum::Router;
use dotenvy::dotenv;
use sqlx::SqlitePool;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn init_tracing() {
    let tracing_env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(tracing_env_filter)
        .with_line_number(true)
        .with_thread_names(true)
        .init();
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    init_tracing();
    info!("Duckymerone started!");

    dotenv()?;
    info!("Loaded environment variables from .env file");

    let db_pool = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;
    info!("Connected to database");

    let router = Router::new().merge(routes::players::create_player_router(db_pool.clone()));

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:4444")
        .await
        .map_err(|err| AppError::ServerError(err.to_string()))?;
    info!(
        "Listening on {}",
        tcp_listener
            .local_addr()
            .map_err(|err| AppError::ServerError(err.to_string()))?
    );

    axum::serve(tcp_listener, router)
        .await
        .map_err(|err| AppError::ServerError(err.to_string()))?;

    Ok(())
}
