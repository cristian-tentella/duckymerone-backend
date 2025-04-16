use sqlx::SqlitePool;
use crate::app_error::AppError;

pub struct Player {
    pub id: Option<i64>,
    pub name: String,
    pub token: String,
}

pub async fn create_player(pool: &SqlitePool, name: &str) -> Result<Player, AppError> {
    let token = uuid::Uuid::new_v4().to_string();

    sqlx::query!(
        "INSERT INTO players (name, token) VALUES (?, ?)",
        name,
        token
    )
    .execute(pool)
    .await?;

    let player = sqlx::query_as!(
        Player,
        "SELECT id, name, token FROM players WHERE token = ?",
        token
    )
    .fetch_one(pool)
    .await?;

    Ok(player)
}

pub async fn get_player_by_token(pool: &SqlitePool, token: &str) -> Result<Player, AppError> {
    sqlx::query_as!(
        Player,
        "SELECT id, name, token FROM players WHERE token = ?",
        token
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::SqlxError)
}

pub async fn player_exists(pool: &SqlitePool, token: &str) -> Result<bool, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM players WHERE token = ?)",
        token
    )
    .fetch_one(pool)
    .await?;

    Ok(exists == 1)
}

pub async fn list_players(pool: &SqlitePool) -> Result<Vec<Player>, AppError> {
    sqlx::query_as!(
        Player,
        "SELECT id, name, token FROM players"
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::SqlxError)
}

pub async fn update_player_name(pool: &SqlitePool, token: &str, new_name: &str) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE players SET name = ? WHERE token = ?",
        new_name,
        token
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_player_by_token(pool: &SqlitePool, token: &str) -> Result<(), AppError> {
    sqlx::query!(
        "DELETE FROM players WHERE token = ?",
        token
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    #[sqlx::test]
    async fn test_create_and_get_player(pool: SqlitePool) -> Result<(), AppError> {
        let name = "Alice";
        let player = create_player(&pool, name).await?;
        assert_eq!(player.name, name);

        let fetched = get_player_by_token(&pool, &player.token).await?;
        assert_eq!(fetched.id, player.id);
        Ok(())
    }

    #[sqlx::test]
    async fn test_player_exists(pool: SqlitePool) -> Result<(), AppError> {
        let player = create_player(&pool, "Bob").await?;
        assert!(player_exists(&pool, &player.token).await?);
        assert!(!player_exists(&pool, "nonexistent").await?);
        Ok(())
    }

    #[sqlx::test]
    async fn test_update_player_name(pool: SqlitePool) -> Result<(), AppError> {
        let player = create_player(&pool, "Charlie").await?;
        update_player_name(&pool, &player.token, "Chuck").await?;
        let updated = get_player_by_token(&pool, &player.token).await?;
        assert_eq!(updated.name, "Chuck");
        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_player(pool: SqlitePool) -> Result<(), AppError> {
        let player = create_player(&pool, "Dave").await?;
        delete_player_by_token(&pool, &player.token).await?;
        let result = get_player_by_token(&pool, &player.token).await;
        assert!(result.is_err());
        Ok(())
    }
}
