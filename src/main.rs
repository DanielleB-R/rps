#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json;
use rocket::State;

use rocket_sync_db_pools::{database, diesel};

use serde::Deserialize;

use rps::db;
use rps::game::{Game, Move};
use rps::models::{NewUserOwned, User};

#[get("/user/id/<id>")]
async fn get_user_by_id(conn: RpsDatabaseConnection, id: i32) -> Option<json::Json<User>> {
    Some(json::Json(
        conn.run(move |c| db::get_user_by_id(c, id)).await.ok()?,
    ))
}

#[post("/user", data = "<input>")]
async fn create_user(
    conn: RpsDatabaseConnection,
    input: json::Json<NewUserOwned>,
) -> json::Json<User> {
    let new_user = conn
        .run(move |c| db::create_user_struct(c, &input))
        .await
        // TODO: Return a 400 when we get a constraint violation
        .unwrap();

    json::Json(new_user)
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct NewGameInput {
    player1: i32,
    player2: i32,
}

#[post("/game", data = "<input>")]
async fn new_game(
    redis: &State<redis::Client>,
    input: json::Json<NewGameInput>,
) -> (Status, Option<json::Json<Game>>) {
    //TODO: Check in Postgres for the users

    let game = Game::new(rand::random(), input.player1, input.player2);
    (Status::Created, Some(json::Json(game)))
}

#[get("/game/<id>")]
async fn get_game(client: &State<redis::Client>, id: usize) -> Option<json::Json<Game>> {
    let mut con = client.get_tokio_connection().await.unwrap();

    rps::save_game::retrieve_game_async(&mut con, id)
        .await
        .unwrap()
        .map(|game| json::Json(game))
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct MoveInput {
    player_id: i32,
    action: Move,
}

#[post("/game/<id>/move", data = "<input>")]
async fn make_move(
    redis: &State<redis::Client>,
    id: usize,
    input: json::Json<MoveInput>,
) -> Option<(Status, json::Json<Game>)> {
    let mut con = redis.get_tokio_connection().await.unwrap();

    let mut game = rps::save_game::retrieve_game_async(&mut con, id)
        .await
        .unwrap()?;

    match game.play_by_id(input.player_id, input.action) {
        None => Some((Status::BadRequest, json::Json(game))),
        Some(_) => {
            rps::save_game::save_game_async(&mut con, &game)
                .await
                .unwrap();
            Some((Status::Ok, json::Json(game)))
        }
    }
}

#[database("rps")]
struct RpsDatabaseConnection(diesel::PgConnection);

#[catch(404)]
fn not_found() -> status::NotFound<()> {
    status::NotFound(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![get_user_by_id, create_user, get_game, make_move],
        )
        .register("/", catchers![not_found])
        .manage(redis::Client::open("redis://localhost/").unwrap())
        .attach(RpsDatabaseConnection::fairing())
}
