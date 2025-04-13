use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    
    let db_pool = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;

    Ok(())
}
