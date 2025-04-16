use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use crate::{app_error::AppError, db::players::*};

pub fn create_player_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/players", post(create_player_handler).get(list_players_handler))
        .route("/players/{token}", get(get_player_handler).put(update_player_name_handler).delete(delete_player_handler))
        .route("/players/{token}/exists", get(player_exists_handler))
        .with_state(pool)
}

#[derive(Deserialize)]
pub struct CreatePlayerPayload {
    pub name: String,
}

pub async fn create_player_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreatePlayerPayload>,
) -> Result<Json<Player>, AppError> {
    let player = create_player(&pool, &payload.name).await?;
    Ok(Json(player))
}

pub async fn get_player_handler(
    State(pool): State<SqlitePool>,
    Path(token): Path<String>,
) -> Result<Json<Player>, AppError> {
    let player = get_player_by_token(&pool, &token).await?;
    Ok(Json(player))
}

pub async fn player_exists_handler(
    State(pool): State<SqlitePool>,
    Path(token): Path<String>,
) -> Result<Json<bool>, AppError> {
    let exists = player_exists(&pool, &token).await?;
    Ok(Json(exists))
}

pub async fn list_players_handler(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Player>>, AppError> {
    let players = list_players(&pool).await?;
    Ok(Json(players))
}

#[derive(Deserialize)]
pub struct UpdatePlayerPayload {
    pub new_name: String,
}

pub async fn update_player_name_handler(
    State(pool): State<SqlitePool>,
    Path(token): Path<String>,
    Json(payload): Json<UpdatePlayerPayload>,
) -> Result<impl IntoResponse, AppError> {
    update_player_name(&pool, &token, &payload.new_name).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_player_handler(
    State(pool): State<SqlitePool>,
    Path(token): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    delete_player_by_token(&pool, &token).await?;
    Ok(StatusCode::NO_CONTENT)
}
