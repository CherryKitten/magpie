use crate::Result;

pub async fn setup() -> Result<sqlx::Pool<sqlx::Postgres>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:nya@localhost:5432/postgres")
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
