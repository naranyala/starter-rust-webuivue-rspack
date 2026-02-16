use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Lock error: {0}")]
    Lock(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

pub type AppResult<T> = Result<T, AppError>;

pub fn lock_error<T>(e: std::sync::PoisonError<T>) -> AppError {
    AppError::Lock(e.to_string())
}

pub fn not_found<T>(item: &str) -> AppError {
    AppError::NotFound(item.to_string())
}
