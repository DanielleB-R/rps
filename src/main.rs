#[macro_use]
extern crate rocket;
use rocket::serde::json;
use serde::Deserialize;

use rps::models::{NewUser, User};

#[get("/user/id/<id>")]
fn get_user_by_id(id: i32) -> json::Json<User> {
    json::Json(User {
        id,
        name: "Danielle".to_owned(),
        pronouns: "she/her".to_owned(),
        age: 38,
        deleted: false,
        username: "danielle".to_owned(),
    })
}

#[post("/user", data = "<input>")]
fn create_user(input: json::Json<NewUser>) -> json::Json<User> {
    json::Json(User {
        id: 20,
        name: input.name.to_owned(),
        pronouns: input.pronouns.to_owned(),
        age: input.age,
        deleted: false,
        username: input.username.to_owned(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_user_by_id, create_user])
}
