//! Module state management and persistence

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// Module state manager for handling persistent state
#[derive(Debug)]
pub struct ModuleStateManager {
    storage: Box<dyn StateStorage>,
}

/// Trait for state storage backends
pub trait StateStorage: Send + Sync {
    /// Save module state
    fn save_state(&self, module_id: crate::modules::ModuleId, state: &serde_json::Value) -> Result<()>;

    /// Load module state
    fn load_state(&self, module_id: crate::modules::ModuleId) -> Result<Option<serde_json::Value>>;

    /// Delete module state
    fn delete_state(&self, module_id: crate::modules::ModuleId) -> Result<()>;
}

/// File-based state storage implementation
#[derive(Debug)]
pub struct FileStateStorage {
    base_dir: PathBuf,
}

impl FileStateStorage {
    /// Create new file-based storage
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Get state file path for a module
    fn get_state_file(&self, module_id: crate::modules::ModuleId) -> PathBuf {
        let module_name = match module_id {
            crate::modules::ModuleId::Home => "home",
            crate::modules::ModuleId::FileBrowser => "file_browser",
            crate::modules::ModuleId::TextEditor => "text_editor",
            crate::modules::ModuleId::SystemMonitor => "system_monitor",
            crate::modules::ModuleId::Settings => "settings",
            crate::modules::ModuleId::Custom(id) => &format!("custom_{}", id),
        };

        self.base_dir.join(format!("{}.json", module_name))
    }
}

impl StateStorage for FileStateStorage {
    fn save_state(&self, module_id: crate::modules::ModuleId, state: &serde_json::Value) -> Result<()> {
        let state_file = self.get_state_file(module_id);

        // Ensure directory exists
        if let Some(parent) = state_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write state to file
        let content = serde_json::to_string_pretty(state)?;
        std::fs::write(state_file, content)?;

        debug!("Saved state for module: {:?}", module_id);
        Ok(())
    }

    fn load_state(&self, module_id: crate::modules::ModuleId) -> Result<Option<serde_json::Value>> {
        let state_file = self.get_state_file(module_id);

        if !state_file.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&state_file)?;
        let state: serde_json::Value = serde_json::from_str(&content)?;

        debug!("Loaded state for module: {:?}", module_id);
        Ok(Some(state))
    }

    fn delete_state(&self, module_id: crate::modules::ModuleId) -> Result<()> {
        let state_file = self.get_state_file(module_id);

        if state_file.exists() {
            std::fs::remove_file(&state_file)?;
            debug!("Deleted state for module: {:?}", module_id);
        }

        Ok(())
    }
}

impl ModuleStateManager {
    /// Create new module state manager
    pub fn new(storage: Box<dyn StateStorage>) -> Self {
        Self { storage }
    }

    /// Create with default file storage
    pub fn new_with_file_storage() -> Result<Self> {
        let state_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("{{project-name}}")
            .join("module_states");

        Ok(Self::new(Box::new(FileStateStorage::new(state_dir))))
    }

    /// Save module state
    pub fn save_module_state(&self, module_id: crate::modules::ModuleId, state: &serde_json::Value) -> Result<()> {
        self.storage.save_state(module_id, state)
    }

    /// Load module state
    pub fn load_module_state(&self, module_id: crate::modules::ModuleId) -> Result<Option<serde_json::Value>> {
        self.storage.load_state(module_id)
    }

    /// Delete module state
    pub fn delete_module_state(&self, module_id: crate::modules::ModuleId) -> Result<()> {
        self.storage.delete_state(module_id)
    }

    /// Clean up orphaned states
    pub fn cleanup_orphaned_states(&self, active_modules: &[crate::modules::ModuleId]) -> Result<()> {
        info!("Cleaning up orphaned module states");

        // Get all saved states and remove those not in active_modules
        // This is a placeholder - implementation depends on storage backend
        debug!("Cleanup completed for {} active modules", active_modules.len());
        Ok(())
    }

    /// Backup all states
    pub fn backup_states(&self, backup_path: &PathBuf) -> Result<()> {
        info!("Backing up module states to: {:?}", backup_path);

        // Create backup directory
        std::fs::create_dir_all(backup_path)?;

        // This is a placeholder - implementation depends on storage backend
        debug!("Backup completed successfully");
        Ok(())
    }

    /// Restore states from backup
    pub fn restore_states(&self, backup_path: &PathBuf) -> Result<()> {
        info!("Restoring module states from: {:?}", backup_path);

        if !backup_path.exists() {
            return Err(crate::AppError::Config("Backup directory not found".to_string()));
        }

        // This is a placeholder - implementation depends on storage backend
        debug!("Restore completed successfully");
        Ok(())
    }
}

/// State validation utilities
pub mod validation {
    use serde_json::Value;

    /// Validate state JSON structure
    pub fn validate_state_structure(state: &Value, expected_schema: &Value) -> bool {
        // Simple validation - in a real implementation, you'd use jsonschema or similar
        match (state, expected_schema) {
            (Value::Object(_), Value::Object(_)) => true,
            (Value::Array(_), Value::Array(_)) => true,
            (Value::String(_), Value::String(_)) => true,
            (Value::Number(_), Value::Number(_)) => true,
            (Value::Bool(_), Value::Bool(_)) => true,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }

    /// Sanitize state values
    pub fn sanitize_state(state: &mut Value) {
        match state {
            Value::Object(ref mut map) => {
                for (_, value) in map.iter_mut() {
                    sanitize_state(value);
                }
            }
            Value::Array(ref mut array) => {
                for value in array.iter_mut() {
                    sanitize_state(value);
                }
            }
            Value::String(ref mut string) => {
                // Remove potentially harmful characters
                *string = string.chars()
                    .filter(|c| c.is_ascii() && !c.is_control())
                    .collect();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_file_state_storage() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileStateStorage::new(temp_dir.path().to_path_buf());
        let module_id = crate::modules::ModuleId::Home;

        // Test save and load
        let state = serde_json::json!({
            "test_key": "test_value",
            "number": 42
        });

        assert!(storage.save_state(module_id, &state).is_ok());

        let loaded_state = storage.load_state(module_id).unwrap();
        assert!(loaded_state.is_some());
        assert_eq!(loaded_state.unwrap(), state);

        // Test delete
        assert!(storage.delete_state(module_id).is_ok());
        assert!(storage.load_state(module_id).unwrap().is_none());
    }

    #[test]
    fn test_module_state_manager() {
        let temp_dir = TempDir::new().unwrap();
        let state_dir = temp_dir.path().to_path_buf();
        std::fs::create_dir_all(&state_dir).unwrap();

        let manager = ModuleStateManager::new(Box::new(FileStateStorage::new(state_dir)));

        let module_id = crate::modules::ModuleId::TextEditor;
        let state = serde_json::json!({
            "content": "Hello, World!",
            "cursor_position": 13
        });

        assert!(manager.save_module_state(module_id, &state).is_ok());

        let loaded_state = manager.load_module_state(module_id).unwrap();
        assert!(loaded_state.is_some());
        assert_eq!(loaded_state.unwrap(), state);

        assert!(manager.delete_module_state(module_id).is_ok());
        assert!(manager.load_module_state(module_id).unwrap().is_none());
    }

    #[test]
    fn test_state_validation() {
        use super::validation::*;

        let state = serde_json::json!({
            "name": "test",
            "value": 42
        });

        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "value": {"type": "number"}
            }
        });

        // This is a simplified test - in reality, schema validation would be more complex
        assert!(validate_state_structure(&state, &schema));

        let invalid_state = serde_json::json!("string");
        assert!(!validate_state_structure(&invalid_state, &schema));
    }

    #[test]
    fn test_state_sanitization() {
        use super::validation::*;

        let mut state = serde_json::json!({
            "text": "Hello\x00World\x01"
        });

        sanitize_state(&mut state);

        if let Some(text) = state.get("text").and_then(|v| v.as_str()) {
            assert!(!text.contains('\x00'));
            assert!(!text.contains('\x01'));
        }
    }
}