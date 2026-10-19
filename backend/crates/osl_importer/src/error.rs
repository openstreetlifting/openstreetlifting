use thiserror::Error;

pub type Result<T> = std::result::Result<T, ImporterError>;

#[derive(Error, Debug)]
pub enum ImporterError {
    #[cfg(feature = "database")]
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[cfg(feature = "database")]
    #[error("Storage error: {0}")]
    StorageError(#[from] osl_db::error::StorageError),

    #[error("Data transformation error: {0}")]
    TransformationError(String),

    #[error("Import error: {0}")]
    ImportError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
