use crate::settings::get_config;
use anyhow::Result;
use diesel::prelude::*;

pub mod models;
pub mod schema;
pub use models::*;

pub fn establish_connection() -> Result<SqliteConnection> {
    let config = get_config()?;
    let database_url = config.get_string("db")?;

    Ok(SqliteConnection::establish(&database_url)?)
}
