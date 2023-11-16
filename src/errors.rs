pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    SQLXError(#[from] sqlx::Error),
    #[error("Env var not found: {0}")]
    EnvNotFound(#[from] std::env::VarError),
    #[error("Parse error: {0}")]
    ParseIntErrror(#[from] core::num::ParseIntError),
}
