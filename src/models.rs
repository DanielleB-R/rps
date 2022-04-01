use super::schema::{games, users};
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
    pub id: i32,
    pub player_1: i32,
    pub player_2: i32,
    pub winner: Option<i32>,
    pub rounds: Option<i32>,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[table_name = "games"]
pub struct NewGameRecord {
    pub player_1: i32,
    pub player_2: i32,
}
