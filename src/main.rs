use dotenvy::dotenv;
use sqlx::SqlitePool;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tracing_env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(tracing_env_filter)
        .with_line_number(true)
        .with_thread_names(true)
        .init();
    
    info!("Duckymerone started!");

    dotenv()?;
    
    let db_pool = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;

    Ok(())
}
