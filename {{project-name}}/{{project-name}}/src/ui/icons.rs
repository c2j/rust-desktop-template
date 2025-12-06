//! Icon management for the desktop application

use egui::{Color32, FontId, RichText};

/// Icon management system for the desktop application
#[derive(Debug, Clone)]
pub struct IconManager {
    /// Default icon size
    default_size: f32,
}

impl IconManager {
    /// Create a new icon manager
    pub fn new() -> Self {
        Self {
            default_size: 16.0,
        }
    }

    /// Get an icon with default size
    pub fn get(&self, icon: &str) -> RichText {
        self.get_with_size(icon, self.default_size)
    }

    /// Get an icon with custom size
    pub fn get_with_size(&self, icon: &str, size: f32) -> RichText {
        RichText::new(icon).size(size)
    }

    /// Get a colored icon
    pub fn get_colored(&self, icon: &str, color: Color32) -> RichText {
        self.get_with_size_and_color(icon, self.default_size, color)
    }

    /// Get a colored icon with custom size
    pub fn get_with_size_and_color(&self, icon: &str, size: f32, color: Color32) -> RichText {
        RichText::new(icon).size(size).color(color)
    }
}

impl Default for IconManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Common icons used throughout the application
pub mod icons {
    /// Navigation icons
    pub mod navigation {
        pub const HOME: &str = "🏠";
        pub const FILE_BROWSER: &str = "📁";
        pub const TEXT_EDITOR: &str = "📝";
        pub const SYSTEM_MONITOR: &str = "📊";
        pub const SETTINGS: &str = "⚙️";
        pub const BACK: &str = "⬅️";
        pub const FORWARD: &str = "➡️";
        pub const REFRESH: &str = "🔄";
    }

    /// Action icons
    pub mod actions {
        pub const ADD: &str = "➕";
        pub const REMOVE: &str = "➖";
        pub const EDIT: &str = "✏️";
        pub const SAVE: &str = "💾";
        pub const OPEN: &str = "📂";
        pub const CLOSE: &str = "❌";
        pub const CHECK: &str = "✅";
        pub const ERROR: &str = "❌";
        pub const WARNING: &str = "⚠️";
        pub const INFO: &str = "ℹ️";
        pub const HELP: &str = "❓";
    }

    /// UI element icons
    pub mod ui {
        pub const MENU: &str = "☰";
        pub const ARROW_DOWN: &str = "⬇️";
        pub const ARROW_UP: &str = "⬆️";
        pub const ARROW_LEFT: &str = "⬅️";
        pub const ARROW_RIGHT: &str = "➡️";
        pub const MINIMIZE: &str = "➖";
        pub const MAXIMIZE: &str = "⬜";
        pub const RESTORE: &str = "🗗";
        pub const CLOSE: &str = "❌";
    }

    /// File type icons
    pub mod files {
        pub const FILE: &str = "📄";
        pub const FOLDER: &str = "📁";
        pub const FOLDER_OPEN: &str = "📂";
        pub const CODE: &str = "📟";
        pub const IMAGE: &str = "🖼️";
        pub const VIDEO: &str = "🎬";
        pub const AUDIO: &str = "🎵";
        pub const ARCHIVE: &str = "📦";
        pub const CONFIG: &str = "⚙️";
        pub const DATABASE: &str = "🗄️";
    }

    /// System icons
    pub mod system {
        pub const MEMORY: &str = "🧠";
        pub const CPU: &str = "🖥️";
        pub const DISK: &str = "💾";
        pub const NETWORK: &str = "🌐";
        pub const BATTERY: &str = "🔋";
        pub const SHUTDOWN: &str = "🔌";
        pub const RESTART: &str = "🔄";
        pub const LOCK: &str = "🔒";
        pub const UNLOCK: &str = "🔓";
    }

    /// Theme and appearance icons
    pub mod theme {
        pub const LIGHT: &str = "☀️";
        pub const DARK: &str = "🌙";
        pub const AUTO: &str = "🌓";
        pub const PALETTE: &str = "🎨";
        pub const CONTRAST: &str = "🔳";
    }

    /// Status icons
    pub mod status {
        pub const SUCCESS: &str = "✅";
        pub const ERROR: &str = "❌";
        pub const WARNING: &str = "⚠️";
        pub const INFO: &str = "ℹ️";
        pub const LOADING: &str = "⏳";
        pub const COMPLETED: &str = "✨";
        pub const IN_PROGRESS: &str = "⚡";
        pub const PENDING: &str = "⏰";
    }
}

/// Helper functions for commonly used icon combinations
pub mod helpers {
    use super::IconManager;
    use crate::ui::Theme;

    /// Create a navigation icon with appropriate styling
    pub fn navigation_icon(manager: &IconManager, icon: &str, theme: &Theme, active: bool) -> egui::RichText {
        if active {
            manager.get_colored(icon, theme.colors.text)
        } else {
            manager.get_colored(icon, theme.colors.text_secondary)
        }
    }

    /// Create a status icon with appropriate color
    pub fn status_icon(manager: &IconManager, status: &str, theme: &Theme) -> egui::RichText {
        let color = match status {
            "success" | "ok" => theme.colors.success,
            "error" | "failed" => theme.colors.error,
            "warning" | "warn" => theme.colors.warning,
            _ => theme.colors.text,
        };

        let icon = match status {
            "success" | "ok" => super::icons::status::SUCCESS,
            "error" | "failed" => super::icons::status::ERROR,
            "warning" | "warn" => super::icons::status::WARNING,
            "info" => super::icons::status::INFO,
            "loading" => super::icons::status::LOADING,
            _ => super::icons::status::INFO,
        };

        manager.get_colored(icon, color)
    }

    /// Create a themed action icon
    pub fn action_icon(manager: &IconManager, icon: &str, theme: &Theme, primary: bool) -> egui::RichText {
        let color = if primary {
            theme.colors.primary
        } else {
            theme.colors.text_secondary
        };

        manager.get_colored(icon, color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::Theme;

    #[test]
    fn test_icon_manager_creation() {
        let manager = IconManager::new();
        assert_eq!(manager.default_size, 16.0);
    }

    #[test]
    fn test_icon_creation() {
        let manager = IconManager::new();
        let icon = manager.get("🚀");
        // Should not panic
        assert_eq!(icon.text(), "🚀");
    }

    #[test]
    fn test_colored_icon() {
        let manager = IconManager::new();
        let icon = manager.get_colored("🚀", egui::Color32::RED);
        // Should not panic
        assert_eq!(icon.text(), "🚀");
    }

    #[test]
    fn test_helper_functions() {
        let manager = IconManager::new();
        let theme = Theme::dark();

        let nav_icon = helpers::navigation_icon(&manager, "🏠", &theme, true);
        assert_eq!(nav_icon.text(), "🏠");

        let status_icon = helpers::status_icon(&manager, "success", &theme);
        assert_eq!(status_icon.text(), "✅");
    }
}