#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json;
use rocket::State;

use serde::Deserialize;

use rps::db;
use rps::db_connection::RpsDatabaseConnection;
use rps::game::{Game, Move};
use rps::management::NewGameError;
use rps::models::{NewUserOwned, User};

#[get("/user/id/<id>")]
async fn get_user_by_id(conn: RpsDatabaseConnection, id: i32) -> Option<json::Json<User>> {
    Some(json::Json(
        conn.run(move |c| db::get_user_by_id(c, id))
            .await
            .unwrap()?,
    ))
}

#[get("/user/username/<username>")]
async fn get_user_by_username(
    conn: RpsDatabaseConnection,
    username: String,
) -> Option<json::Json<User>> {
    Some(json::Json(
        conn.run(move |c| db::get_user_by_username(c, &username))
            .await
            .unwrap()?,
    ))
}

#[post("/user", data = "<input>")]
async fn create_user(
    conn: RpsDatabaseConnection,
    input: json::Json<NewUserOwned>,
) -> Result<json::Json<User>, Status> {
    conn.run(move |c| db::create_user_struct(c, &input))
        .await
        .map(|new_user| json::Json(new_user))
        .map_err(|err| match err {
            db::CreateUserError::ConflictingUsernameError => Status::BadRequest,
            _ => Status::InternalServerError,
        })
}

#[post("/game", data = "<input>")]
async fn new_game(
    postgres: RpsDatabaseConnection,
    redis: &State<redis::Client>,
    input: json::Json<rps::management::NewGameInput>,
) -> Result<(Status, json::Json<Game>), (Status, String)> {
    let mut con = redis.get_tokio_connection().await.unwrap();
    rps::management::new_game(postgres, &mut con, input.into_inner())
        .await
        .map(|new_game| (Status::Created, json::Json(new_game)))
        .map_err(|err| match err {
            NewGameError::Player1NotFound
            | NewGameError::Player2NotFound
            | NewGameError::PlayersAreSame => (Status::BadRequest, err.to_string()),
            err => (Status::InternalServerError, err.to_string()),
        })
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
    postgres: RpsDatabaseConnection,
    redis: &State<redis::Client>,
    id: usize,
    input: json::Json<MoveInput>,
) -> Option<(Status, json::Json<Game>)> {
    let mut con = redis.get_tokio_connection().await.unwrap();

    let mut game = rps::save_game::retrieve_game_async(&mut con, id)
        .await
        .unwrap()?;

    match game.play_by_id(input.player_id, input.action) {
        Err(_) => Some((Status::BadRequest, json::Json(game))),
        Ok(done) => {
            if done {
                let winner_id = game.winner.unwrap();
                let rounds = game.rounds.len();
                postgres
                    .run(move |c| db::store_game_winner(c, id as i32, winner_id, rounds as i32))
                    .await
                    .unwrap();
            }
            rps::save_game::save_game_async(&mut con, &game)
                .await
                .unwrap();

            Some((Status::Ok, json::Json(game)))
        }
    }
}

#[catch(404)]
fn not_found() -> status::NotFound<()> {
    status::NotFound(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                get_user_by_id,
                get_user_by_username,
                create_user,
                new_game,
                get_game,
                make_move
            ],
        )
        .register("/", catchers![not_found])
        .manage(redis::Client::open("redis://localhost/").unwrap())
        .attach(RpsDatabaseConnection::fairing())
}
