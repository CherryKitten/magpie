use color_eyre::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::SqliteConnection,
    Connection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations");

pub(crate) fn connect(database_url: impl Into<String>) -> Result<SqliteConnection> {
    let database_url = database_url.into();
    let mut db = SqliteConnection::establish(&database_url)?;

    tracing::info!("Connecting to database at {}", database_url);

    run_migrations(&mut db);

    Ok(db)
}

pub fn get_connection_pool(
    database_url: impl Into<String>,
) -> Result<Pool<ConnectionManager<SqliteConnection>>> {
    let database_url = database_url.into();
    let manager = ConnectionManager::new(&database_url);

    tracing::info!("Connecting to database at {}", database_url);
    let pool = Pool::builder()
        .test_on_check_out(true)
        .max_size(15)
        .build(manager)?;

    let mut conn = pool.get()?;

    run_migrations(&mut conn);

    Ok(pool)
}

pub fn run_migrations(conn: &mut SqliteConnection) {
    tracing::info!("Running database migrations");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");
    tracing::info!("Finished running database migrations");
}
