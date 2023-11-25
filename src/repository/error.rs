use thiserror::Error;

/// I'm on the fence.  Should each of these `Result` types be tied to
/// the module?  Should this be `DbResult`?  I like the consistency of
/// just using `Result`.  But it could be confusing.
pub type Result<T> = core::result::Result<T, DbError>;

/// Any DB error type that needs to be acted on in the service layer should
/// be broken out.  I'm just using NotFound for now.  But you might want to
/// capture indexing errors, such as AlreadyExists.
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Not Found")]
    NotFound,
    #[error("Repository Service error: {0}")]
    DatabaseError(sqlx::Error),
}

/// By implementing From manually (rather than using `DatabaseError(#[from] sqlx:Error),`)
/// We can break out NotFound.  Use this pattern for any other DBError you want to
/// act on in the service layer.
impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::DatabaseError(e),
        }
    }
}
