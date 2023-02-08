pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use models::*;
use schema::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


