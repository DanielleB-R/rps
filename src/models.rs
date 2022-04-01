use super::schema::users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub pronouns: String,
    pub age: i32,
    #[serde(skip)]
    pub deleted: bool,
    pub username: String,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub pronouns: &'a str,
    pub age: i32,
    pub username: &'a str,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUserOwned {
    pub name: String,
    pub pronouns: String,
    pub age: i32,
    pub username: String,
}

#[derive(Debug, Clone, Queryable)]
pub struct GameRecord {
    id: i32,
    player_1: i32,
    player_2: i32,
    winner: Option<i32>,
    rounds: Option<i32>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[table_name = "games"]
pub struct NewGameRecord {
    player_1: i32,
    player_2: i32,
}
