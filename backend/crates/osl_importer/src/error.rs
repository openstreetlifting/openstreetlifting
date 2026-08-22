use thiserror::Error;

pub type Result<T> = std::result::Result<T, ImporterError>;

#[derive(Error, Debug)]
pub enum ImporterError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Storage error: {0}")]
    StorageError(#[from] osl_domain::error::StorageError),

    #[error("Data transformation error: {0}")]
    TransformationError(String),

    #[error("Import error: {0}")]
    ImportError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
