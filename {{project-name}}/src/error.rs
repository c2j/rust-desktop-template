//! Error handling for the desktop application

use thiserror::Error;

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Module error: {0}")]
    Module(String),

    #[error("Theme error: {0}")]
    Theme(String),

    #[error("UI error: {0}")]
    Ui(String),

    #[error("Platform error: {0}")]
    Platform(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Convenience type alias for Results with AppError
pub type Result<T> = std::result::Result<T, AppError>;