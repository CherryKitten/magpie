use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::Sqlite;

use crate::settings::get_config;
use crate::Result;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub mod schema;

pub fn establish_connection() -> Result<SqliteConnection> {
    let config = get_config()?;
    let database_url = config.get_string("db")?;

    Ok(SqliteConnection::establish(&database_url)?)
}

pub fn create_connection_pool() -> Result<DbPool> {
    let config = get_config()?;
    let database_url = config.get_string("db")?;

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let pool = Pool::builder().build(manager)?;

    pool.get()?.batch_execute(
        "
            PRAGMA busy_timeout = 10000;
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA wal_autocheckpoint = 1000;
            PRAGMA wal_checkpoint(TRUNCATE);
            PRAGMA cache_size = 134217728;",
    )?;

    Ok(pool)
}

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");

pub fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<()> {
    log::info!("Running Database Migrations");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");

    Ok(())
}
