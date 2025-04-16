mod app_error;
mod db;

use app_error::AppError;
use dotenvy::dotenv;
use sqlx::SqlitePool;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn init_tracing() {
     let tracing_env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

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
    
    let db_pool = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await;

    Ok(())
}
