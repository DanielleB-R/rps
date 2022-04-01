use crate::models;
use diesel::pg::PgConnection;
use diesel::prelude::*;

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
