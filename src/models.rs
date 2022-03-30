use super::schema::users;
use diesel::{Insertable, Queryable};

#[derive(Debug, Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub pronouns: String,
    pub age: i32,
    pub deleted: bool,
    pub username: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub pronouns: &'a str,
    pub age: i32,
    pub username: &'a str,
}
