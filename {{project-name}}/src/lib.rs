//! {{project-description}}
//!
//! A modern desktop application built with Rust, egui, and eframe
//!
//! This template provides a VS Code-like interface with:
//! - Navigation sidebar with module switching
//! - Central workspace for main content
//! - Optional right sidebar for secondary content
//! - Status bar for system information
//! - Comprehensive theming system
//! - Modular architecture for extensibility

pub mod app;
pub mod config;
pub mod error;
pub mod logger;
pub mod modules;
pub mod platform;
pub mod ui;

pub use app::App;
pub use error::{AppError, Result};