//! Project configuration management

use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info, warn};

/// Project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,
    /// Project version
    pub version: String,
    /// Default theme
    pub theme: String,
    /// Project description
    pub description: String,
    /// Author information
    pub author: AuthorConfig,
    /// Layout configuration
    pub layout: LayoutConfig,
    /// Feature flags
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorConfig {
    pub name: String,
    pub email: String,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    /// Left sidebar width in pixels
    pub left_sidebar_width: f32,
    /// Right sidebar width in pixels
    pub right_sidebar_width: f32,
    /// Status bar visibility
    pub show_status_bar: bool,
    /// Window minimum size
    pub window_min_size: [f32; 2],
    /// Window default size
    pub window_default_size: [f32; 2],
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "My Desktop App".to_string(),
            version: "0.1.0".to_string(),
            theme: "{{theme}}".to_string(),
            description: "{{project-description}}".to_string(),
            author: AuthorConfig::default(),
            layout: LayoutConfig::default(),
            features: vec!["{{#if include_examples}}examples{{/if}}".to_string()],
        }
    }
}

impl Default for AuthorConfig {
    fn default() -> Self {
        Self {
            name: "{{author-name}}".to_string(),
            email: "{{author-email}}".to_string(),
            website: None,
        }
    }
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            left_sidebar_width: 48.0,
            right_sidebar_width: 300.0,
            show_status_bar: true,
            window_min_size: [800.0, 600.0],
            window_default_size: [1200.0, 800.0],
        }
    }
}

impl ProjectConfig {
    /// Load project configuration from file
    pub fn load_from_file(path: &str) -> Result<Self> {
        info!("Loading project configuration from: {}", path);

        let content = std::fs::read_to_string(path)
            .map_err(|e| AppError::Config(format!("Failed to read project config: {}", e)))?;

        let config: ProjectConfig = toml::from_str(&content)
            .map_err(|e| AppError::Config(format!("Failed to parse project config: {}", e)))?;

        debug!("Loaded project configuration: {:?}", config);
        Ok(config)
    }

    /// Save project configuration to file
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        info!("Saving project configuration to: {:?}", path);

        let content = toml::to_string_pretty(self)
            .map_err(|e| AppError::Config(format!("Failed to serialize project config: {}", e)))?;

        std::fs::write(path, content)
            .map_err(|e| AppError::Config(format!("Failed to write project config: {}", e)))?;

        debug!("Saved project configuration successfully");
        Ok(())
    }

    /// Create default project configuration file
    pub fn create_default_file(path: &Path) -> Result<()> {
        info!("Creating default project configuration at: {:?}", path);

        let config = ProjectConfig::default();
        config.save_to_file(path)?;

        info!("Created default project configuration");
        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        debug!("Validating project configuration");

        // Validate project name
        if self.name.is_empty() {
            return Err(AppError::Config("Project name cannot be empty".to_string()));
        }

        if self.name.len() > 50 {
            return Err(AppError::Config("Project name too long (max 50 characters)".to_string()));
        }

        // Validate version
        if self.version.is_empty() {
            return Err(AppError::Config("Project version cannot be empty".to_string()));
        }

        // Simple semantic version validation
        if !self.version.chars().all(|c| c.is_ascii_digit() || c == '.') {
            warn!("Project version '{}' may not follow semantic versioning", self.version);
        }

        // Validate theme
        if self.theme.is_empty() {
            return Err(AppError::Config("Theme cannot be empty".to_string()));
        }

        let valid_themes = ["light", "dark", "{{theme}}"];
        if !valid_themes.contains(&self.theme.as_str()) && !self.theme.ends_with(".toml") {
            warn!("Unknown theme: '{}', using default", self.theme);
        }

        // Validate author
        self.author.validate()?;

        // Validate layout
        self.layout.validate()?;

        debug!("Project configuration validation passed");
        Ok(())
    }

    /// Get project name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get project version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get default theme
    pub fn default_theme(&self) -> &str {
        &self.theme
    }

    /// Get layout configuration
    pub fn layout(&self) -> &LayoutConfig {
        &self.layout
    }

    /// Get layout configuration (mutable)
    pub fn layout_mut(&mut self) -> &mut LayoutConfig {
        &mut self.layout
    }
}

impl AuthorConfig {
    /// Validate author configuration
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(AppError::Config("Author name cannot be empty".to_string()));
        }

        if self.email.is_empty() {
            return Err(AppError::Config("Author email cannot be empty".to_string()));
        }

        // Basic email validation
        if !self.email.contains('@') || !self.email.contains('.') {
            warn!("Author email '{}' may be invalid", self.email);
        }

        Ok(())
    }
}

impl LayoutConfig {
    /// Validate layout configuration
    pub fn validate(&self) -> Result<()> {
        if self.left_sidebar_width < 20.0 || self.left_sidebar_width > 500.0 {
            return Err(AppError::Config("Left sidebar width must be between 20 and 500 pixels".to_string()));
        }

        if self.right_sidebar_width < 100.0 || self.right_sidebar_width > 800.0 {
            return Err(AppError::Config("Right sidebar width must be between 100 and 800 pixels".to_string()));
        }

        if self.window_min_size[0] < 400.0 || self.window_min_size[1] < 300.0 {
            return Err(AppError::Config("Window minimum size too small (minimum 400x300)".to_string()));
        }

        if self.window_default_size[0] < self.window_min_size[0] || self.window_default_size[1] < self.window_min_size[1] {
            return Err(AppError::Config("Default window size smaller than minimum size".to_string()));
        }

        Ok(())
    }

    /// Get left sidebar width
    pub fn left_sidebar_width(&self) -> f32 {
        self.left_sidebar_width
    }

    /// Get right sidebar width
    pub fn right_sidebar_width(&self) -> f32 {
        self.right_sidebar_width
    }

    /// Check if status bar should be shown
    pub fn show_status_bar(&self) -> bool {
        self.show_status_bar
    }

    /// Get window minimum size
    pub fn window_min_size(&self) -> [f32; 2] {
        self.window_min_size
    }

    /// Get window default size
    pub fn window_default_size(&self) -> [f32; 2] {
        self.window_default_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_config_default() {
        let config = ProjectConfig::default();
        assert_eq!(config.name, "My Desktop App");
        assert_eq!(config.version, "0.1.0");
        assert_eq!(config.theme, "{{theme}}");
    }

    #[test]
    fn test_project_config_validation() {
        let mut config = ProjectConfig::default();
        assert!(config.validate().is_ok());

        // Test invalid name
        config.name = "".to_string();
        assert!(config.validate().is_err());

        config.name = "Valid Name".to_string();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_layout_config_validation() {
        let mut layout = LayoutConfig::default();
        assert!(layout.validate().is_ok());

        // Test invalid sidebar width
        layout.left_sidebar_width = 10.0;
        assert!(layout.validate().is_err());

        layout.left_sidebar_width = 48.0;
        assert!(layout.validate().is_ok());
    }

    #[test]
    fn test_author_config_validation() {
        let mut author = AuthorConfig::default();
        assert!(author.validate().is_ok());

        // Test invalid email
        author.email = "invalid-email".to_string();
        assert!(author.validate().is_ok()); // Should warn but not fail

        author.email = "valid@email.com".to_string();
        assert!(author.validate().is_ok());
    }

    #[test]
    fn test_project_config_serialization() {
        let config = ProjectConfig::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: ProjectConfig = toml::from_str(&serialized).unwrap();

        assert_eq!(deserialized.name, config.name);
        assert_eq!(deserialized.version, config.version);
        assert_eq!(deserialized.theme, config.theme);
    }
}