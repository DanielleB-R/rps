#[macro_use]
extern crate rocket;
use rocket::serde::json;

use rocket_sync_db_pools::{database, diesel};

use rps::db;
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
        .unwrap();

    json::Json(new_user)
}

#[database("rps")]
struct RpsDatabaseConnection(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_user_by_id, create_user])
        .attach(RpsDatabaseConnection::fairing())
}
