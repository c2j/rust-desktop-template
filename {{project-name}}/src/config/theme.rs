//! Theme configuration management

use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info, warn};

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Theme name
    pub name: String,
    /// Theme file path (for custom themes)
    pub theme_file: Option<String>,
    /// Custom theme settings
    pub custom_settings: Option<CustomThemeSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomThemeSettings {
    /// Color overrides
    pub colors: ColorOverrides,
    /// Font overrides
    pub fonts: FontOverrides,
    /// Layout overrides
    pub layout: LayoutOverrides,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorOverrides {
    pub primary: Option<String>,
    pub background: Option<String>,
    pub text: Option<String>,
    pub border: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontOverrides {
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
    pub line_height: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutOverrides {
    pub sidebar_width: Option<f32>,
    pub button_radius: Option<f32>,
    pub spacing: Option<f32>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "{{theme}}".to_string(),
            theme_file: None,
            custom_settings: None,
        }
    }
}

impl Default for CustomThemeSettings {
    fn default() -> Self {
        Self {
            colors: ColorOverrides::default(),
            fonts: FontOverrides::default(),
            layout: LayoutOverrides::default(),
        }
    }
}

impl Default for ColorOverrides {
    fn default() -> Self {
        Self {
            primary: None,
            background: None,
            text: None,
            border: None,
        }
    }
}

impl Default for FontOverrides {
    fn default() -> Self {
        Self {
            font_size: None,
            font_family: None,
            line_height: None,
        }
    }
}

impl Default for LayoutOverrides {
    fn default() -> Self {
        Self {
            sidebar_width: None,
            button_radius: None,
            spacing: None,
        }
    }
}

impl ThemeConfig {
    /// Load theme configuration from project settings
    pub fn load_from_settings(theme_name: &str) -> Result<Self> {
        info!("Loading theme configuration for: {}", theme_name);

        // Check if it's a custom theme file
        if theme_name.ends_with(".toml") {
            debug!("Loading custom theme file: {}", theme_name);
            let custom_settings = Self::load_custom_theme_file(theme_name)?;
            return Ok(Self {
                name: theme_name.to_string(),
                theme_file: Some(theme_name.to_string()),
                custom_settings: Some(custom_settings),
            });
        }

        // Built-in theme
        debug!("Using built-in theme: {}", theme_name);
        Ok(Self {
            name: theme_name.to_string(),
            theme_file: None,
            custom_settings: None,
        })
    }

    /// Load custom theme file
    fn load_custom_theme_file(theme_file: &str) -> Result<CustomThemeSettings> {
        let content = std::fs::read_to_string(theme_file)
            .map_err(|e| AppError::Theme(format!("Failed to read theme file: {}", e)))?;

        let settings: CustomThemeSettings = toml::from_str(&content)
            .map_err(|e| AppError::Theme(format!("Failed to parse theme file: {}", e)))?;

        debug!("Loaded custom theme settings from: {}", theme_file);
        Ok(settings)
    }

    /// Save theme configuration to settings file
    pub fn save_to_settings(&self) -> Result<()> {
        if let Some(custom_settings) = &self.custom_settings {
            if let Some(theme_file) = &self.theme_file {
                info!("Saving custom theme settings to: {}", theme_file);
                let content = toml::to_string_pretty(custom_settings)
                    .map_err(|e| AppError::Theme(format!("Failed to serialize theme settings: {}", e)))?;

                std::fs::write(theme_file, content)
                    .map_err(|e| AppError::Theme(format!("Failed to write theme settings: {}", e)))?;

                debug!("Saved custom theme settings successfully");
            }
        }
        Ok(())
    }

    /// Validate theme configuration
    pub fn validate(&self) -> Result<()> {
        debug!("Validating theme configuration");

        if self.name.is_empty() {
            return Err(AppError::Theme("Theme name cannot be empty".to_string()));
        }

        // Validate custom theme file if specified
        if let Some(theme_file) = &self.theme_file {
            if !Path::new(theme_file).exists() {
                warn!("Theme file does not exist: {}", theme_file);
            }
        }

        debug!("Theme configuration validation passed");
        Ok(())
    }

    /// Check if this is a custom theme
    pub fn is_custom(&self) -> bool {
        self.theme_file.is_some()
    }

    /// Get theme name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get custom settings
    pub fn custom_settings(&self) -> Option<&CustomThemeSettings> {
        self.custom_settings.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_config_default() {
        let config = ThemeConfig::default();
        assert_eq!(config.name, "{{theme}}");
        assert!(!config.is_custom());
    }

    #[test]
    fn test_theme_config_validation() {
        let mut config = ThemeConfig::default();
        assert!(config.validate().is_ok());

        config.name = "".to_string();
        assert!(config.validate().is_err());

        config.name = "Valid Theme".to_string();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_custom_theme_detection() {
        let config = ThemeConfig::default();
        assert!(!config.is_custom());

        let config = ThemeConfig {
            name: "custom".to_string(),
            theme_file: Some("custom.toml".to_string()),
            custom_settings: None,
        };
        assert!(config.is_custom());
    }
}