//! Logging configuration for the desktop application

use tracing::{info, Level};
use tracing_subscriber::{fmt, EnvFilter};

/// Initialize the logging system
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    match tracing::subscriber::set_global_default(subscriber) {
        Ok(()) => info!("Logging system initialized successfully"),
        Err(e) => eprintln!("Failed to initialize logging: {}", e),
    }
}

/// Get the current log level
pub fn current_level() -> Level {
    tracing::level_filters::LevelFilter::current().into_level()
        .unwrap_or(Level::INFO)
}