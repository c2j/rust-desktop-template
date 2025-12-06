//! Configuration management for the desktop application

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info};

pub mod project;
pub mod theme;

pub use project::ProjectConfig;
pub use theme::ThemeConfig;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub theme: ThemeConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: ProjectConfig::default(),
            theme: ThemeConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from files
    pub fn load() -> Result<Self> {
        info!("Loading application configuration");

        // Try to load from project.toml first
        let project_config = if Path::new("project.toml").exists() {
            debug!("Loading project configuration from project.toml");
            ProjectConfig::load_from_file("project.toml")?
        } else {
            debug!("Using default project configuration");
            ProjectConfig::default()
        };

        // Load theme configuration
        let theme_config = ThemeConfig::load_from_settings(&project_config.theme)?;

        Ok(Self {
            project: project_config,
            theme: theme_config,
        })
    }

    /// Save configuration to files
    pub fn save(&self, project_path: &Path) -> Result<()> {
        info!("Saving application configuration");

        // Save project configuration
        self.project.save_to_file(project_path)?;

        // Save theme configuration if custom
        if self.theme.is_custom() {
            self.theme.save_to_settings()?;
        }

        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        self.project.validate()?;
        self.theme.validate()?;
        Ok(())
    }

    /// Get a mutable reference to project configuration
    pub fn project_mut(&mut self) -> &mut ProjectConfig {
        &mut self.project
    }

    /// Get a reference to project configuration
    pub fn project(&self) -> &ProjectConfig {
        &self.project
    }

    /// Get a mutable reference to theme configuration
    pub fn theme_mut(&mut self) -> &mut ThemeConfig {
        &mut self.theme
    }

    /// Get a reference to theme configuration
    pub fn theme(&self) -> &ThemeConfig {
        &self.theme
    }
}

/// Configuration validation trait
pub trait ConfigValidation {
    /// Validate the configuration
    fn validate(&self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::default();
        assert_eq!(config.project.name, "My Desktop App");
        assert_eq!(config.theme.name, "Dark");
    }

    #[test]
    fn test_config_validation() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }
}