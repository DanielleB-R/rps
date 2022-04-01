use crate::db;
use crate::db_connection::RpsDatabaseConnection;
use crate::game::Game;
use crate::models;
use crate::save_game;

use redis::aio::Connection;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewGameError {
    #[error("player 1 not found in database")]
    Player1NotFound,
    #[error("player 2 not found in database")]
    Player2NotFound,
    #[error("players must be distinct")]
    PlayersAreSame,
    #[error("database error")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("redis error")]
    RedisError(#[from] redis::RedisError),
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewGameInput {
    player1: String,
    player2: String,
}

pub async fn new_game(
    postgres: RpsDatabaseConnection,
    redis: &mut Connection,
    input: NewGameInput,
) -> Result<Game, NewGameError> {
    if input.player1 == input.player2 {
        return Err(NewGameError::PlayersAreSame);
    }

    let player1 = postgres
        .run(move |c| db::get_user_by_username(c, &input.player1))
        .await?
        .ok_or(NewGameError::Player1NotFound)?;
    let player2 = postgres
        .run(move |c| db::get_user_by_username(c, &input.player2))
        .await?
        .ok_or(NewGameError::Player2NotFound)?;

    let new_game_record = models::NewGameRecord {
        player_1: player1.id,
        player_2: player2.id,
    };
    let game_record = postgres
        .run(move |c| db::create_game(c, &new_game_record))
        .await?;

    let game = Game::new(game_record.id as usize, player1.id, player2.id);

    save_game::save_game_async(redis, &game).await?;

    Ok(game)
}
