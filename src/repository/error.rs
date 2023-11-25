use thiserror::Error;

pub type Result<T> = core::result::Result<T, DbError>;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Not Found")]
    NotFound,
    #[error("Repository Service error: {0}")]
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::DatabaseError(e),
        }
    }
}
