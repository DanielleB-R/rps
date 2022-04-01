use crate::models;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CreateUserError {
    #[error("database error")]
    DatabaseError(diesel::result::Error),
    #[error("conflicting username")]
    ConflictingUsernameError,
}

impl From<diesel::result::Error> for CreateUserError {
    fn from(err: diesel::result::Error) -> Self {
        use diesel::result::DatabaseErrorKind::*;
        use diesel::result::Error::*;

        match err {
            DatabaseError(UniqueViolation, _) => Self::ConflictingUsernameError,
            _ => Self::DatabaseError(err),
        }
    }
}

pub fn create_user_struct(
    conn: &PgConnection,
    user: &models::NewUserOwned,
) -> Result<models::User, CreateUserError> {
    use crate::schema::users;

    Ok(diesel::insert_into(users::table)
        .values(user)
        .get_result(conn)?)
}

pub fn get_user_by_id(conn: &PgConnection, id: i32) -> QueryResult<Option<models::User>> {
    use crate::schema::users;

    users::table
        .find(id)
        .filter(users::deleted.eq(false))
        .first(conn)
        .optional()
}

pub fn get_user_by_username(
    conn: &PgConnection,
    username: &str,
) -> QueryResult<Option<models::User>> {
    use crate::schema::users;

    users::table
        .filter(users::username.eq(username))
        .filter(users::deleted.eq(false))
        .first(conn)
        .optional()
}

pub fn create_game(
    conn: &PgConnection,
    record: &models::NewGameRecord,
) -> QueryResult<models::GameRecord> {
    use crate::schema::games::table;

    diesel::insert_into(table).values(record).get_result(conn)
}

pub fn store_game_winner(
    conn: &PgConnection,
    id: i32,
    winner_id: i32,
    rounds: i32,
) -> QueryResult<models::GameRecord> {
    use crate::schema::games;

    diesel::update(games::table.find(id))
        .set((games::winner.eq(winner_id), games::rounds.eq(rounds)))
        .get_result(conn)
}
