use sqlx::migrate::MigrateError;
use sqlx::Error;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, MagpieError>;

#[derive(Error, Debug)]
pub enum MagpieError {
    #[error("Unknown error occured, sorry.")]
    GenericError,
    #[error("Database Error occured: {msg}")]
    DatabaseError { msg: String },
}

impl From<sqlx::Error> for MagpieError {
    fn from(value: Error) -> Self {
        let msg = value.to_string();

        MagpieError::DatabaseError { msg }
    }
}

impl From<sqlx::migrate::MigrateError> for MagpieError {
    fn from(value: MigrateError) -> Self {
        let msg = value.to_string();

        MagpieError::DatabaseError { msg }
    }
}
