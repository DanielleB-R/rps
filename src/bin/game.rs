use redis::Commands;
use rps::game;

fn main() {
    let mut game = game::Game::new(4, 2, 6);

    game.play1(game::Move::Rock);
    game.play2(game::Move::Scissors);

    let redis_client = redis::Client::open("redis://localhost/").unwrap();
    let mut con = redis_client.get_connection().unwrap();

    let _: () = con
        .set(game.id.to_string(), serde_json::to_string(&game).unwrap())
        .unwrap();

    let retrieved_game: Option<String> = con.get(game.id.to_string()).unwrap();

    let new_game: game::Game = serde_json::from_str(&retrieved_game.unwrap()).unwrap();

    println!("{:?}", game);
    println!("{:?}", new_game);
}
