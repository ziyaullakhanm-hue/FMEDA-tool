use thiserror::Error;

#[derive(Error, Debug)]
pub enum FmedaError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Calculation error: {0}")]
    Calculation(String),

    #[error("Other error: {0}")]
    Other(String),
}
