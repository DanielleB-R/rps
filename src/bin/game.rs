use rps::{game, save_game};

fn main() {
    let mut game = game::Game::new(8, 2, 6);

    game.play1(game::Move::Rock);
    game.play2(game::Move::Scissors);

    let redis_client = redis::Client::open("redis://localhost/").unwrap();
    let mut con = redis_client.get_connection().unwrap();

    save_game::save_game(&mut con, &game).unwrap();

    let new_game = save_game::retrieve_game(&mut con, game.id);

    println!("{:?}", game);
    println!("{:?}", new_game);
}
