//! Module configuration management

use crate::{
    error::Result,
    modules::{ApplicationModule, ModuleId},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Module configuration manager
#[derive(Debug)]
pub struct ModuleConfigManager {
    configs: HashMap<ModuleId, ModuleConfig>,
    defaults: HashMap<ModuleId, ModuleConfig>,
}

/// Individual module configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    /// Module identifier
    pub module_id: ModuleId,
    /// Module name
    pub name: String,
    /// Module icon
    pub icon: String,
    /// Module description
    pub description: String,
    /// Module settings
    pub settings: ModuleSettings,
    /// UI configuration
    pub ui: ModuleUIConfig,
    /// Permissions
    pub permissions: ModulePermissions,
}

/// Module-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSettings {
    /// Whether the module is enabled
    pub enabled: bool,
    /// Auto-load module on startup
    pub auto_load: bool,
    /// Auto-activate module on startup
    pub auto_activate: bool,
    /// Module-specific configuration values
    pub custom: HashMap<String, serde_json::Value>,
}

/// UI configuration for module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleUIConfig {
    /// Whether to show module in navigation
    pub show_in_navigation: bool,
    /// Module position in navigation
    pub navigation_position: Option<u32>,
    /// Module shortcut key
    pub shortcut: Option<String>,
    /// Whether to show module in status bar
    pub show_in_status_bar: bool,
    /// Custom CSS classes or styling
    pub custom_styles: HashMap<String, String>,
}

/// Module permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulePermissions {
    /// Can access file system
    pub file_system_access: bool,
    /// Can access network
    pub network_access: bool,
    /// Can access system information
    pub system_info_access: bool,
    /// Can modify settings
    pub settings_access: bool,
    /// Required permissions list
    pub required_permissions: Vec<String>,
}

impl Default for ModuleConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleConfigManager {
    /// Create new module configuration manager
    pub fn new() -> Self {
        let manager = Self {
            configs: HashMap::new(),
            defaults: HashMap::new(),
        };

        // Set up default configurations
        manager.setup_default_configs();
        manager
    }

    /// Set up default module configurations
    fn setup_default_configs(&mut self) {
        // Home module config
        self.defaults.insert(
            ModuleId::Home,
            ModuleConfig {
                module_id: ModuleId::Home,
                name: "Home".to_string(),
                icon: "🏠".to_string(),
                description: "Home screen and dashboard".to_string(),
                settings: ModuleSettings {
                    enabled: true,
                    auto_load: true,
                    auto_activate: true,
                    custom: HashMap::new(),
                },
                ui: ModuleUIConfig {
                    show_in_navigation: true,
                    navigation_position: Some(1),
                    shortcut: None,
                    show_in_status_bar: true,
                    custom_styles: HashMap::new(),
                },
                permissions: ModulePermissions {
                    file_system_access: false,
                    network_access: false,
                    system_info_access: true,
                    settings_access: true,
                    required_permissions: Vec::new(),
                },
            },
        );

        // File browser module config
        self.defaults.insert(
            ModuleId::FileBrowser,
            ModuleConfig {
                module_id: ModuleId::FileBrowser,
                name: "File Browser".to_string(),
                icon: "📁".to_string(),
                description: "Browse and manage files and directories".to_string(),
                settings: ModuleSettings {
                    enabled: true,
                    auto_load: true,
                    auto_activate: false,
                    custom: {
                        let mut custom = HashMap::new();
                        custom.insert("show_hidden_files".to_string(), serde_json::Value::Bool(false));
                        custom.insert("default_path".to_string(), serde_json::Value::String(
                            std::env::current_dir()
                                .unwrap_or_else(|_| PathBuf::from("/"))
                                .display()
                                .to_string(),
                        ));
                        custom.insert("show_file_sizes".to_string(), serde_json::Value::Bool(true));
                        custom.insert("sort_by".to_string(), serde_json::Value::String("name".to_string()));
                        custom
                    },
                },
                ui: ModuleUIConfig {
                    show_in_navigation: true,
                    navigation_position: Some(2),
                    shortcut: Some("Ctrl+Shift+F".to_string()),
                    show_in_status_bar: true,
                    custom_styles: HashMap::new(),
                },
                permissions: ModulePermissions {
                    file_system_access: true,
                    network_access: false,
                    system_info_access: false,
                    settings_access: false,
                    required_permissions: vec!["file_system".to_string()],
                },
            },
        );

        // Text editor module config
        self.defaults.insert(
            ModuleId::TextEditor,
            ModuleConfig {
                module_id: ModuleId::TextEditor,
                name: "Text Editor".to_string(),
                icon: "📝".to_string(),
                description: "Edit text files with syntax highlighting".to_string(),
                settings: ModuleSettings {
                    enabled: true,
                    auto_load: true,
                    auto_activate: false,
                    custom: {
                        let mut custom = HashMap::new();
                        custom.insert("font_size".to_string(), serde_json::Value::Number(serde_json::Number::from(14)));
                        custom.insert("tab_size".to_string(), serde_json::Value::Number(serde_json::Number::from(4)));
                        custom.insert("word_wrap".to_string(), serde_json::Value::Bool(true));
                        custom.insert("auto_save".to_string(), serde_json::Value::Bool(true));
                        custom.insert("syntax_highlighting".to_string(), serde_json::Value::Bool(true));
                        custom.insert("theme".to_string(), serde_json::Value::String("dark".to_string()));
                        custom
                    },
                },
                ui: ModuleUIConfig {
                    show_in_navigation: true,
                    navigation_position: Some(3),
                    shortcut: Some("Ctrl+Shift+E".to_string()),
                    show_in_status_bar: true,
                    custom_styles: HashMap::new(),
                },
                permissions: ModulePermissions {
                    file_system_access: true,
                    network_access: false,
                    system_info_access: false,
                    settings_access: true,
                    required_permissions: vec!["file_system".to_string()],
                },
            },
        );

        // System monitor module config
        self.defaults.insert(
            ModuleId::SystemMonitor,
            ModuleConfig {
                module_id: ModuleId::SystemMonitor,
                name: "System Monitor".to_string(),
                icon: "📊".to_string(),
                description: "Monitor system resources and performance".to_string(),
                settings: ModuleSettings {
                    enabled: true,
                    auto_load: true,
                    auto_activate: false,
                    custom: {
                        let mut custom = HashMap::new();
                        custom.insert("update_interval".to_string(), serde_json::Value::Number(serde_json::Number::from(1000)));
                        custom.insert("show_graphs".to_string(), serde_json::Value::Bool(true));
                        custom.insert("graph_history".to_string(), serde_json::Value::Number(serde_json::Number::from(60)));
                        custom.insert("cpu_threshold".to_string(), serde_json::Value::Number(serde_json::Number::from(80)));
                        custom.insert("memory_threshold".to_string(), serde_json::Value::Number(serde_json::Number::from(80)));
                        custom.insert("top_processes_count".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
                        custom
                    },
                },
                ui: ModuleUIConfig {
                    show_in_navigation: true,
                    navigation_position: Some(4),
                    shortcut: Some("Ctrl+Shift+M".to_string()),
                    show_in_status_bar: true,
                    custom_styles: HashMap::new(),
                },
                permissions: ModulePermissions {
                    file_system_access: false,
                    network_access: false,
                    system_info_access: true,
                    settings_access: false,
                    required_permissions: vec!["system_info".to_string()],
                },
            },
        );

        // Settings module config
        self.defaults.insert(
            ModuleId::Settings,
            ModuleConfig {
                module_id: ModuleId::Settings,
                name: "Settings".to_string(),
                icon: "⚙️".to_string(),
                description: "Application settings and configuration".to_string(),
                settings: ModuleSettings {
                    enabled: true,
                    auto_load: true,
                    auto_activate: false,
                    custom: HashMap::new(),
                },
                ui: ModuleUIConfig {
                    show_in_navigation: true,
                    navigation_position: Some(999), // Always last
                    shortcut: Some("Ctrl+,".to_string()),
                    show_in_status_bar: false,
                    custom_styles: HashMap::new(),
                },
                permissions: ModulePermissions {
                    file_system_access: false,
                    network_access: false,
                    system_info_access: false,
                    settings_access: true,
                    required_permissions: Vec::new(),
                },
            },
        );
    }

    /// Get configuration for a module
    pub fn get_config(&self, module_id: ModuleId) -> Option<&ModuleConfig> {
        self.configs.get(&module_id).or_else(|| self.defaults.get(&module_id))
    }

    /// Get mutable configuration for a module
    pub fn get_config_mut(&mut self, module_id: ModuleId) -> &mut ModuleConfig {
        self.configs.entry(module_id).or_insert_with(|| {
            self.defaults
                .get(&module_id)
                .cloned()
                .unwrap_or_else(|| ModuleConfig::default(module_id))
        })
    }

    /// Set configuration for a module
    pub fn set_config(&mut self, module_id: ModuleId, config: ModuleConfig) {
        self.configs.insert(module_id, config);
    }

    /// Load configurations from file
    pub fn load_from_file(&mut self, path: &std::path::Path) -> Result<()> {
        info!("Loading module configurations from: {:?}", path);

        if !path.exists() {
            warn!("Module config file does not exist: {:?}", path);
            return Ok(());
        }

        let content = std::fs::read_to_string(path)?;
        let configs: HashMap<String, ModuleConfig> = serde_json::from_str(&content)?;

        for (key, config) in configs {
            if let Ok(module_id) = serde_json::from_str::<ModuleId>(&format!("\"{}\"", key)) {
                self.configs.insert(module_id, config);
            }
        }

        info!("Loaded {} module configurations", self.configs.len());
        Ok(())
    }

    /// Save configurations to file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        info!("Saving module configurations to: {:?}", path);

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Convert configs to JSON-serializable format
        let mut json_configs: HashMap<String, ModuleConfig> = HashMap::new();
        for (module_id, config) in &self.configs {
            let key = serde_json::to_string(module_id)?;
            json_configs.insert(key, config.clone());
        }

        let content = serde_json::to_string_pretty(&json_configs)?;
        std::fs::write(path, content)?;

        info!("Saved {} module configurations", self.configs.len());
        Ok(())
    }

    /// Get all enabled modules
    pub fn get_enabled_modules(&self) -> Vec<ModuleId> {
        let mut enabled_modules = Vec::new();

        for (module_id, config) in &self.configs {
            if config.settings.enabled {
                enabled_modules.push(*module_id);
            }
        }

        // Also check defaults for modules not in configs
        for (module_id, default_config) in &self.defaults {
            if !self.configs.contains_key(module_id) && default_config.settings.enabled {
                enabled_modules.push(*module_id);
            }
        }

        enabled_modules.sort_by_key(|id| {
            self.get_config(*id)
                .and_then(|c| c.ui.navigation_position)
                .unwrap_or(999)
        });

        enabled_modules
    }

    /// Get modules sorted by navigation position
    pub fn get_navigation_modules(&self) -> Vec<(&ModuleId, &ModuleConfig)> {
        let mut modules: Vec<_> = self
            .defaults
            .iter()
            .chain(self.configs.iter())
            .map(|(id, config)| (id, config))
            .collect();

        // Remove duplicates (use config over default)
        modules.sort_by_key(|(id, _)| **id);
        modules.dedup_by(|(id, _)| **id);

        // Sort by navigation position
        modules.sort_by(|(_, config_a), (_, config_b)| {
            let pos_a = config_a.ui.navigation_position.unwrap_or(999);
            let pos_b = config_b.ui.navigation_position.unwrap_or(999);
            pos_a.cmp(&pos_b)
        });

        // Filter enabled modules
        modules
            .into_iter()
            .filter(|(_, config)| config.settings.enabled && config.ui.show_in_navigation)
            .collect()
    }

    /// Get all configurations
    pub fn get_all_configs(&self) -> HashMap<ModuleId, ModuleConfig> {
        let mut all_configs = self.defaults.clone();

        // Override with custom configs
        for (module_id, config) in &self.configs {
            all_configs.insert(*module_id, config.clone());
        }

        all_configs
    }

    /// Validate module permissions
    pub fn validate_permissions(&self, module_id: ModuleId) -> Result<()> {
        if let Some(config) = self.get_config(module_id) {
            // Check if required permissions are available
            for required_permission in &config.permissions.required_permissions {
                match required_permission.as_str() {
                    "file_system" if config.permissions.file_system_access => {}
                    "network" if config.permissions.network_access => {}
                    "system_info" if config.permissions.system_info_access => {}
                    "settings" if config.permissions.settings_access => {}
                    perm => {
                        return Err(crate::AppError::Module(format!(
                            "Module {:?} requires '{}' permission but it's not granted",
                            module_id, perm
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    /// Reset module to default configuration
    pub fn reset_to_default(&mut self, module_id: ModuleId) {
        if let Some(default_config) = self.defaults.get(&module_id) {
            self.configs.insert(module_id, default_config.clone());
        }
    }

    /// Reset all modules to default configurations
    pub fn reset_all_to_default(&mut self) {
        self.configs.clear();
    }
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            module_id: ModuleId::Home,
            name: "Unknown Module".to_string(),
            icon: "📦".to_string(),
            description: "A module without description".to_string(),
            settings: ModuleSettings::default(),
            ui: ModuleUIConfig::default(),
            permissions: ModulePermissions::default(),
        }
    }
}

impl Default for ModuleSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_load: true,
            auto_activate: false,
            custom: HashMap::new(),
        }
    }
}

impl Default for ModuleUIConfig {
    fn default() -> Self {
        Self {
            show_in_navigation: true,
            navigation_position: None,
            shortcut: None,
            show_in_status_bar: false,
            custom_styles: HashMap::new(),
        }
    }
}

impl Default for ModulePermissions {
    fn default() -> Self {
        Self {
            file_system_access: false,
            network_access: false,
            system_info_access: false,
            settings_access: false,
            required_permissions: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_config_manager_creation() {
        let manager = ModuleConfigManager::new();

        // Should have default configurations for known modules
        assert!(manager.get_config(ModuleId::Home).is_some());
        assert!(manager.get_config(ModuleId::FileBrowser).is_some());
        assert!(manager.get_config(ModuleId::TextEditor).is_some());
        assert!(manager.get_config(ModuleId::SystemMonitor).is_some());
    }

    #[test]
    fn test_module_config_modification() {
        let mut manager = ModuleConfigManager::new();
        let module_id = ModuleId::TextEditor;

        // Get config and modify it
        let config = manager.get_config_mut(module_id);
        config.name = "Custom Text Editor".to_string();

        // Verify changes
        let updated_config = manager.get_config(module_id).unwrap();
        assert_eq!(updated_config.name, "Custom Text Editor");
    }

    #[test]
    fn test_enabled_modules() {
        let manager = ModuleConfigManager::new();
        let enabled_modules = manager.get_enabled_modules();

        // All default modules should be enabled
        assert!(enabled_modules.contains(&ModuleId::Home));
        assert!(enabled_modules.contains(&ModuleId::FileBrowser));
        assert!(enabled_modules.contains(&ModuleId::TextEditor));
        assert!(enabled_modules.contains(&ModuleId::SystemMonitor));
    }

    #[test]
    fn test_navigation_modules_ordering() {
        let manager = ModuleConfigManager::new();
        let nav_modules = manager.get_navigation_modules();

        // Should be sorted by navigation position
        let positions: Vec<Option<u32>> = nav_modules
            .iter()
            .map(|(_, config)| config.ui.navigation_position)
            .collect();

        let mut sorted_positions = positions.clone();
        sorted_positions.sort();

        assert_eq!(positions, sorted_positions);
    }

    #[test]
    fn test_module_permissions_validation() {
        let manager = ModuleConfigManager::new();

        // File browser should have file system permission
        assert!(manager.validate_permissions(ModuleId::FileBrowser).is_ok());

        // Invalid module should error
        let mut custom_config = manager.get_config(ModuleId::Home).unwrap().clone();
        custom_config.permissions.file_system_access = false;
        custom_config.permissions.required_permissions.push("file_system".to_string());

        let mut manager = ModuleConfigManager::new();
        manager.set_config(ModuleId::Home, custom_config);

        assert!(manager.validate_permissions(ModuleId::Home).is_err());
    }
}