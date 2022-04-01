use crate::game::Game;
use redis::{aio::Connection, AsyncCommands, RedisResult};

pub async fn save_game_async(con: &mut Connection, game: &Game) -> RedisResult<()> {
    let _: () = con
        .set(game.id.to_string(), serde_json::to_string(&game).unwrap())
        .await?;

    Ok(())
}

pub async fn retrieve_game_async(con: &mut Connection, id: usize) -> RedisResult<Option<Game>> {
    let value: Option<String> = con.get(id.to_string()).await?;
    Ok(value.and_then(|s| serde_json::from_str(&s).ok()))
}
