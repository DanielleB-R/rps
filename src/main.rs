#[macro_use]
extern crate rocket;
use rocket::response::status;
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
        // TODO: Return a 400 when we get a constraint violation
        .unwrap();

    json::Json(new_user)
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
        .mount("/", routes![get_user_by_id, create_user])
        .register("/", catchers![not_found])
        .attach(RpsDatabaseConnection::fairing())
}
