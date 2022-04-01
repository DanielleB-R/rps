use rocket_sync_db_pools::{database, diesel};

#[database("rps")]
pub struct RpsDatabaseConnection(diesel::PgConnection);
