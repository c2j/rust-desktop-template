//! Configuration management for the desktop application

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info};

pub mod theme;

pub use theme::ThemeConfig;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Project name
    pub name: String,
    /// Theme configuration
    pub theme: ThemeConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "{{project-name}}".to_string(),
            theme: ThemeConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from files
    pub fn load() -> Result<Self> {
        info!("Loading application configuration");

        // Load theme configuration
        let theme_config = ThemeConfig::load()?;

        Ok(Self {
            name: "{{project-name}}".to_string(),
            theme: theme_config,
        })
    }

    /// Save configuration to files
    pub fn save(&self) -> Result<()> {
        info!("Saving application configuration");
        self.theme.save()
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        debug!("Validating configuration");

        if self.name.is_empty() {
            return Err(crate::error::AppError::Config("Project name cannot be empty".to_string()));
        }

        self.theme.validate()?;

        debug!("Configuration validation passed");
        Ok(())
    }

    /// Get project name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get theme configuration
    pub fn theme(&self) -> &ThemeConfig {
        &self.theme
    }

    /// Get theme configuration (mutable)
    pub fn theme_mut(&mut self) -> &mut ThemeConfig {
        &mut self.theme
    }
}