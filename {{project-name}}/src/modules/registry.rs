//! Module registry for managing application modules

use super::{ApplicationModule, ModuleId};
use std::collections::HashMap;
use tracing::{debug, info};

/// Registry for managing application modules
#[derive(Default)]
pub struct ModuleRegistry {
    modules: HashMap<ModuleId, Box<dyn ApplicationModule>>,
}

impl std::fmt::Debug for ModuleRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleRegistry")
            .field("module_count", &self.modules.len())
            .finish()
    }
}

impl ModuleRegistry {
    /// Create a new empty module registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a module
    pub fn register(&mut self, module: Box<dyn ApplicationModule>) {
        let id = module.id();
        info!("Registering module: {:?}", id);
        self.modules.insert(id, module);
    }

    /// Get a module by ID
    pub fn get(&self, id: ModuleId) -> Option<&dyn ApplicationModule> {
        self.modules.get(&id).map(|m| m.as_ref())
    }

    /// Get a mutable module by ID
    pub fn get_mut(&mut self, _id: ModuleId) -> Option<&mut dyn ApplicationModule> {
        // TODO: Implement proper mutable reference handling
        None
    }

    /// Remove a module by ID
    pub fn remove(&mut self, id: ModuleId) -> Option<Box<dyn ApplicationModule>> {
        debug!("Removing module: {:?}", id);
        self.modules.remove(&id)
    }

    /// Get all module IDs
    pub fn module_ids(&self) -> Vec<ModuleId> {
        self.modules.keys().copied().collect()
    }

    /// Get all modules
    pub fn modules(&self) -> Vec<&dyn ApplicationModule> {
        self.modules.values().map(|m| m.as_ref()).collect()
    }

    /// Get module count
    pub fn len(&self) -> usize {
        self.modules.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    /// Activate a module
    pub fn activate_module(&mut self, id: ModuleId) -> crate::Result<()> {
        if let Some(module) = self.modules.get_mut(&id) {
            info!("Activating module: {:?}", id);
            module.on_activate();
            Ok(())
        } else {
            Err(crate::AppError::Module(format!("Module not found: {:?}", id)))
        }
    }

    /// Deactivate a module
    pub fn deactivate_module(&mut self, id: ModuleId) -> crate::Result<()> {
        if let Some(module) = self.modules.get_mut(&id) {
            info!("Deactivating module: {:?}", id);
            module.on_deactivate();
            Ok(())
        } else {
            Err(crate::AppError::Module(format!("Module not found: {:?}", id)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestModule {
        id: ModuleId,
        name: String,
    }

    impl TestModule {
        fn new(id: ModuleId, name: &str) -> Self {
            Self {
                id,
                name: name.to_string(),
            }
        }
    }

    impl ApplicationModule for TestModule {
        fn id(&self) -> ModuleId {
            self.id
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn icon(&self) -> &str {
            "🧪"
        }

        fn render(&mut self, _ui: &mut egui::Ui, _ctx: &egui::Context) {
            // Test implementation
        }
    }

    #[test]
    fn test_module_registration() {
        let mut registry = ModuleRegistry::new();
        assert!(registry.is_empty());

        let module = Box::new(TestModule::new(ModuleId::Home, "Test Module"));
        registry.register(module);

        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());
        assert!(registry.get(ModuleId::Home).is_some());
    }

    #[test]
    fn test_module_activation() {
        let mut registry = ModuleRegistry::new();
        let module = Box::new(TestModule::new(ModuleId::Home, "Test Module"));
        registry.register(module);

        assert!(registry.activate_module(ModuleId::Home).is_ok());
        assert!(registry.deactivate_module(ModuleId::Home).is_ok());
    }
}