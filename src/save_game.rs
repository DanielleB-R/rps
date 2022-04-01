use crate::game::Game;
use redis::{AsyncCommands, Commands, Connection, RedisResult};

pub fn save_game(con: &mut Connection, game: &Game) -> RedisResult<()> {
    let _: () = con.set(game.id.to_string(), serde_json::to_string(&game).unwrap())?;

    Ok(())
}

pub async fn save_game_async(con: &mut redis::aio::Connection, game: &Game) -> RedisResult<()> {
    let _: () = con
        .set(game.id.to_string(), serde_json::to_string(&game).unwrap())
        .await?;

    Ok(())
}

pub fn retrieve_game(con: &mut Connection, id: usize) -> RedisResult<Option<Game>> {
    let value: Option<String> = con.get(id.to_string())?;
    Ok(value.and_then(|s| serde_json::from_str(&s).ok()))
}

pub async fn retrieve_game_async(
    con: &mut redis::aio::Connection,
    id: usize,
) -> RedisResult<Option<Game>> {
    let value: Option<String> = con.get(id.to_string()).await?;
    Ok(value.and_then(|s| serde_json::from_str(&s).ok()))
}
