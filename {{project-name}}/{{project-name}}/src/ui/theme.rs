//! Theme system for the desktop application

use crate::error::Result;
use egui::{Color32, Rounding, Style, Visuals};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info};

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub theme_type: ThemeType,
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub widgets: WidgetStyles,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ThemeType {
    Light,
    Dark,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: Color32,
    pub secondary: Color32,
    pub background: Color32,
    pub surface: Color32,
    pub text: Color32,
    pub text_secondary: Color32,
    pub border: Color32,
    pub error: Color32,
    pub warning: Color32,
    pub success: Color32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub font_family: String,
    pub font_size_base: f32,
    pub font_size_small: f32,
    pub font_size_large: f32,
    pub line_height: f32,
    pub font_weight_normal: u32,
    pub font_weight_bold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub unit: f32,
    pub small: f32,
    pub medium: f32,
    pub large: f32,
    pub xlarge: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetStyles {
    pub button_radius: Rounding,
    pub input_radius: Rounding,
    pub card_radius: Rounding,
    pub button_padding: egui::Vec2,
    pub input_padding: egui::Vec2,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Create a dark theme
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            theme_type: ThemeType::Dark,
            colors: ColorPalette {
                primary: Color32::from_rgb(0x00, 0x7A, 0xCC),
                secondary: Color32::from_rgb(0x26, 0xA0, 0xDA),
                background: Color32::from_rgb(0x1E, 0x1E, 0x1E),
                surface: Color32::from_rgb(0x25, 0x25, 0x25),
                text: Color32::from_rgb(0xD4, 0xD4, 0xD4),
                text_secondary: Color32::from_rgb(0x85, 0x85, 0x85),
                border: Color32::from_rgb(0x45, 0x45, 0x45),
                error: Color32::from_rgb(0xF4, 0x47, 0x47),
                warning: Color32::from_rgb(0xFF, 0xCC, 0x02),
                success: Color32::from_rgb(0x89, 0xD1, 0x85),
            },
            typography: Typography {
                font_family: "Proportional".to_string(),
                font_size_base: 14.0,
                font_size_small: 12.0,
                font_size_large: 16.0,
                line_height: 1.4,
                font_weight_normal: 400,
                font_weight_bold: 700,
            },
            spacing: Spacing {
                unit: 8.0,
                small: 4.0,
                medium: 8.0,
                large: 16.0,
                xlarge: 24.0,
            },
            widgets: WidgetStyles {
                button_radius: Rounding::same(4.0),
                input_radius: Rounding::same(4.0),
                card_radius: Rounding::same(8.0),
                button_padding: egui::vec2(12.0, 6.0),
                input_padding: egui::vec2(8.0, 6.0),
            },
        }
    }

    /// Create a light theme
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            theme_type: ThemeType::Light,
            colors: ColorPalette {
                primary: Color32::from_rgb(0x00, 0x7A, 0xCC),
                secondary: Color32::from_rgb(0x26, 0xA0, 0xDA),
                background: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                surface: Color32::from_rgb(0xF8, 0xF8, 0xF8),
                text: Color32::from_rgb(0x00, 0x00, 0x00),
                text_secondary: Color32::from_rgb(0x60, 0x60, 0x60),
                border: Color32::from_rgb(0xE0, 0xE0, 0xE0),
                error: Color32::from_rgb(0xE5, 0x14, 0x00),
                warning: Color32::from_rgb(0xFF, 0x8F, 0x00),
                success: Color32::from_rgb(0x16, 0xC6, 0x0C),
            },
            typography: Typography {
                font_family: "Proportional".to_string(),
                font_size_base: 14.0,
                font_size_small: 12.0,
                font_size_large: 16.0,
                line_height: 1.4,
                font_weight_normal: 400,
                font_weight_bold: 700,
            },
            spacing: Spacing {
                unit: 8.0,
                small: 4.0,
                medium: 8.0,
                large: 16.0,
                xlarge: 24.0,
            },
            widgets: WidgetStyles {
                button_radius: Rounding::same(4.0),
                input_radius: Rounding::same(4.0),
                card_radius: Rounding::same(8.0),
                button_padding: egui::vec2(12.0, 6.0),
                input_padding: egui::vec2(8.0, 6.0),
            },
        }
    }

    /// Load theme from file
    pub fn load(theme_name: &str) -> Result<Self> {
        info!("Loading theme: {}", theme_name);

        // Try to load from themes directory first
        let theme_path = Path::new("themes").join(format!("{}.toml", theme_name));
        if theme_path.exists() {
            debug!("Loading theme from file: {:?}", theme_path);
            let content = std::fs::read_to_string(&theme_path)?;
            let theme: Theme = toml::from_str(&content)?;
            return Ok(theme);
        }

        // Fall back to built-in themes
        match theme_name.to_lowercase().as_str() {
            "light" => Ok(Theme::light()),
            "dark" => Ok(Theme::dark()),
            _ => {
                debug!("Unknown theme '{}', using default dark theme", theme_name);
                Ok(Theme::dark())
            }
        }
    }

    /// Save theme to file
    pub fn save(&self, path: &Path) -> Result<()> {
        debug!("Saving theme to: {:?}", path);
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Apply theme to egui context
    pub fn apply(&self, ctx: &Context) {
        debug!("Applying theme: {}", self.name);

        let mut style = Style::default();

        // Set colors based on theme type
        match self.theme_type {
            ThemeType::Dark => {
                style.visuals = Visuals::dark();
            }
            ThemeType::Light => {
                style.visuals = Visuals::light();
            }
            ThemeType::Custom => {
                // Custom visuals will be set below
            }
        }

        // Apply custom colors
        style.visuals.widgets.noninteractive.bg_fill = self.colors.background;
        style.visuals.widgets.noninteractive.fg_stroke.color = self.colors.text;
        style.visuals.widgets.inactive.bg_fill = self.colors.surface;
        style.visuals.widgets.inactive.fg_stroke.color = self.colors.text;
        style.visuals.widgets.active.bg_fill = self.colors.primary;
        style.visuals.widgets.active.fg_stroke.color = self.colors.background;
        style.visuals.widgets.open.bg_fill = self.colors.surface;
        style.visuals.widgets.open.fg_stroke.color = self.colors.text;

        // Apply spacing
        style.spacing.item_spacing = egui::vec2(self.spacing.medium, self.spacing.medium);
        style.spacing.button_padding = self.widgets.button_padding;

        // Apply widget styles
        style.visuals.widgets.noninteractive.rounding = self.widgets.button_radius;
        style.visuals.widgets.inactive.rounding = self.widgets.button_radius;
        style.visuals.widgets.active.rounding = self.widgets.button_radius;
        style.visuals.widgets.open.rounding = self.widgets.button_radius;

        ctx.set_style(style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let dark_theme = Theme::dark();
        assert_eq!(dark_theme.name, "Dark");
        assert!(matches!(dark_theme.theme_type, ThemeType::Dark));

        let light_theme = Theme::light();
        assert_eq!(light_theme.name, "Light");
        assert!(matches!(light_theme.theme_type, ThemeType::Light));
    }

    #[test]
    fn test_theme_save_load() {
        use tempfile::tempdir;

        let theme = Theme::dark();
        let dir = tempdir().unwrap();
        let theme_path = dir.path().join("test_theme.toml");

        // Save theme
        theme.save(&theme_path).unwrap();
        assert!(theme_path.exists());

        // Load theme
        let loaded_theme = Theme::load("test_theme").unwrap();
        assert_eq!(loaded_theme.name, theme.name);
    }

    #[test]
    fn test_unknown_theme_fallback() {
        let theme = Theme::load("unknown_theme").unwrap();
        assert_eq!(theme.name, "Dark"); // Should fall back to dark theme
    }
}

/// Theme configuration structure for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub theme_type: ThemeType,
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub widgets: WidgetStyles,
}

impl From<Theme> for ThemeConfig {
    fn from(theme: Theme) -> Self {
        Self {
            name: theme.name,
            theme_type: theme.theme_type,
            colors: theme.colors,
            typography: theme.typography,
            spacing: theme.spacing,
            widgets: theme.widgets,
        }
    }
}

impl From<ThemeConfig> for Theme {
    fn from(config: ThemeConfig) -> Self {
        Self {
            name: config.name,
            theme_type: config.theme_type,
            colors: config.colors,
            typography: config.typography,
            spacing: config.spacing,
            widgets: config.widgets,
        }
    }
}