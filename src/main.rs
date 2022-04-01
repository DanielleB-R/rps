#[macro_use]
extern crate rocket;
use rocket::response::status;
use rocket::serde::json;
use rocket::State;

use rocket_sync_db_pools::{database, diesel};

use rps::db;
use rps::game::Game;
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

#[get("/game/id/<id>")]
async fn get_game(client: &State<redis::Client>, id: usize) -> Option<json::Json<Game>> {
    let mut con = client.get_tokio_connection().await.unwrap();

    rps::save_game::retrieve_game_async(&mut con, id)
        .await
        .unwrap()
        .map(|game| json::Json(game))
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
        .mount("/", routes![get_user_by_id, create_user, get_game])
        .register("/", catchers![not_found])
        .manage(redis::Client::open("redis://localhost/").unwrap())
        .attach(RpsDatabaseConnection::fairing())
}
