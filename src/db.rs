use crate::models;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(
    conn: &PgConnection,
    username: &'a str,
    name: &'a str,
    pronouns: &'a str,
    age: i32,
) -> QueryResult<models::User> {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(&models::NewUser {
            username,
            name,
            pronouns,
            age,
        })
        .get_result(conn)
}

pub fn create_user_struct<'a>(
    conn: &PgConnection,
    user: &models::NewUserOwned,
) -> QueryResult<models::User> {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(user)
        .get_result(conn)
}

pub fn get_user_by_id(conn: &PgConnection, id: i32) -> QueryResult<models::User> {
    use crate::schema::users;

    users::table
        .find(id)
        .filter(users::deleted.eq(false))
        .first(conn)
}
