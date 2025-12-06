//! Module lifecycle management

use crate::{
    error::Result,
    modules::{ApplicationModule, ModuleId, ModuleRegistry},
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, warn};

/// Module lifecycle events
#[derive(Debug, Clone)]
pub enum ModuleLifecycleEvent {
    Loading { module_id: ModuleId },
    Loaded { module_id: ModuleId, duration: std::time::Duration },
    Activating { module_id: ModuleId },
    Activated { module_id: ModuleId, duration: std::time::Duration },
    Deactivating { module_id: ModuleId },
    Deactivated { module_id: ModuleId, duration: std::time::Duration },
    Error { module_id: ModuleId, error: String },
    Unloading { module_id: ModuleId },
    Unloaded { module_id: ModuleId, duration: std::time::Duration },
}

/// Module lifecycle manager
#[derive(Debug)]
pub struct ModuleLifecycleManager {
    /// Module lifecycle state
    states: HashMap<ModuleId, ModuleState>,
    /// Lifecycle event listeners
    listeners: Vec<Box<dyn ModuleLifecycleListener>>,
    /// Performance metrics
    metrics: LifecycleMetrics,
}

/// Module lifecycle state
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleState {
    Unloaded,
    Loading,
    Loaded,
    Activating,
    Active,
    Deactivating,
    Inactive,
    Error(String),
}

/// Lifecycle performance metrics
#[derive(Debug, Default)]
pub struct LifecycleMetrics {
    /// Loading times by module
    load_times: HashMap<ModuleId, Vec<std::time::Duration>>,
    /// Activation times by module
    activation_times: HashMap<ModuleId, Vec<std::time::Duration>>,
    /// Error counts by module
    error_counts: HashMap<ModuleId, u32>,
}

/// Trait for lifecycle event listeners
pub trait ModuleLifecycleListener: Send + Sync {
    /// Handle lifecycle event
    fn on_event(&self, event: &ModuleLifecycleEvent);
}

impl ModuleLifecycleManager {
    /// Create new lifecycle manager
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            listeners: Vec::new(),
            metrics: LifecycleMetrics::default(),
        }
    }

    /// Load a module
    pub fn load_module(
        &mut self,
        module_id: ModuleId,
        registry: &ModuleRegistry,
    ) -> Result<()> {
        info!("Loading module: {:?}", module_id);

        let start_time = std::time::Instant::now();

        // Set loading state
        self.set_state(module_id, ModuleState::Loading);
        self.emit_event(ModuleLifecycleEvent::Loading { module_id });

        // Get the module from registry
        let module = registry.get(module_id)
            .ok_or_else(|| crate::AppError::Module(format!("Module not found: {:?}", module_id)))?;

        // Simulate loading process (in a real implementation, this might involve
        // loading resources, initializing services, etc.)
        std::thread::sleep(std::time::Duration::from_millis(100));

        let duration = start_time.elapsed();
        self.metrics.load_times
            .entry(module_id)
            .or_insert_with(Vec::new)
            .push(duration);

        // Set loaded state
        self.set_state(module_id, ModuleState::Loaded);
        self.emit_event(ModuleLifecycleEvent::Loaded { module_id, duration });

        debug!("Module loaded successfully: {:?} (took {}ms)", module_id, duration.as_millis());
        Ok(())
    }

    /// Activate a module
    pub fn activate_module(
        &mut self,
        module_id: ModuleId,
        registry: &mut ModuleRegistry,
    ) -> Result<()> {
        info!("Activating module: {:?}", module_id);

        let start_time = std::time::Instant::now();

        // Set activating state
        self.set_state(module_id, ModuleState::Activating);
        self.emit_event(ModuleLifecycleEvent::Activating { module_id });

        // Deactivate current active module if any
        let current_active = self.get_active_module();
        if let Some(active_id) = current_active {
            if active_id != module_id {
                self.deactivate_module(active_id, registry)?;
            }
        }

        // Get mutable reference to module
        let module = registry.get_mut(module_id)
            .ok_or_else(|| crate::AppError::Module(format!("Module not found: {:?}", module_id)))?;

        // Call on_activate
        module.on_activate();

        let duration = start_time.elapsed();
        self.metrics.activation_times
            .entry(module_id)
            .or_insert_with(Vec::new)
            .push(duration);

        // Set active state
        self.set_state(module_id, ModuleState::Active);
        self.emit_event(ModuleLifecycleEvent::Activated { module_id, duration });

        debug!("Module activated successfully: {:?} (took {}ms)", module_id, duration.as_millis());
        Ok(())
    }

    /// Deactivate a module
    pub fn deactivate_module(
        &mut self,
        module_id: ModuleId,
        registry: &ModuleRegistry,
    ) -> Result<()> {
        info!("Deactivating module: {:?}", module_id);

        let start_time = std::time::Instant::now();

        // Set deactivating state
        self.set_state(module_id, ModuleState::Deactivating);
        self.emit_event(ModuleLifecycleEvent::Deactivating { module_id });

        // Get mutable reference to module
        let module = registry.get_mut(module_id)
            .ok_or_else(|| crate::AppError::Module(format!("Module not found: {:?}", module_id)))?;

        // Call on_deactivate
        module.on_deactivate();

        let duration = start_time.elapsed();

        // Set inactive state
        self.set_state(module_id, ModuleState::Inactive);
        self.emit_event(ModuleLifecycleEvent::Deactivated { module_id, duration });

        debug!("Module deactivated successfully: {:?} (took {}ms)", module_id, duration.as_millis());
        Ok(())
    }

    /// Unload a module
    pub fn unload_module(
        &mut self,
        module_id: ModuleId,
        registry: &mut ModuleRegistry,
    ) -> Result<()> {
        info!("Unloading module: {:?}", module_id);

        let start_time = std::time::Instant::now();

        // Set unloading state
        self.set_state(module_id, ModuleState::Unloading);
        self.emit_event(ModuleLifecycleEvent::Unloading { module_id });

        // Deactivate if active
        if self.get_state(module_id) == ModuleState::Active {
            self.deactivate_module(module_id, registry)?;
        }

        // Remove module from registry
        registry.remove(module_id);

        let duration = start_time.elapsed();
        self.set_state(module_id, ModuleState::Unloaded);
        self.emit_event(ModuleLifecycleEvent::Unloaded { module_id, duration });

        debug!("Module unloaded successfully: {:?}", module_id);
        Ok(())
    }

    /// Get module state
    pub fn get_state(&self, module_id: ModuleId) -> ModuleState {
        self.states.get(&module_id).cloned().unwrap_or(ModuleState::Unloaded)
    }

    /// Set module state
    fn set_state(&mut self, module_id: ModuleId, state: ModuleState) {
        self.states.insert(module_id, state);
    }

    /// Get currently active module
    pub fn get_active_module(&self) -> Option<ModuleId> {
        self.states
            .iter()
            .find(|(_, state)| *state == ModuleState::Active)
            .map(|(id, _)| *id)
    }

    /// Add lifecycle listener
    pub fn add_listener(&mut self, listener: Box<dyn ModuleLifecycleListener>) {
        self.listeners.push(listener);
    }

    /// Emit lifecycle event to all listeners
    fn emit_event(&self, event: ModuleLifecycleEvent) {
        for listener in &self.listeners {
            listener.on_event(&event);
        }
    }

    /// Get lifecycle metrics
    pub fn get_metrics(&self) -> &LifecycleMetrics {
        &self.metrics
    }

    /// Get module metrics
    pub fn get_module_metrics(&self, module_id: ModuleId) -> ModuleMetrics {
        ModuleMetrics {
            load_time: self.metrics.load_times
                .get(&module_id)
                .map(|times| if times.is_empty() { 0.0 } else { times.iter().sum::<std::time::Duration>().as_millis() as f64 / times.len() as f64 })
                .unwrap_or(0.0),
            activation_time: self.metrics.activation_times
                .get(&module_id)
                .map(|times| if times.is_empty() { 0.0 } else { times.iter().sum::<std::time::Duration>().as_millis() as f64 / times.len() as f64 })
                .unwrap_or(0.0),
            error_count: *self.metrics.error_counts.get(&module_id).unwrap_or(&0),
        }
    }

    /// Handle module error
    pub fn handle_module_error(&mut self, module_id: ModuleId, error: String) {
        error!("Module error: {:?} - {}", module_id, error);

        // Increment error count
        *self.metrics.error_counts.entry(module_id).or_insert(0) += 1;

        // Set error state
        self.set_state(module_id, ModuleState::Error(error.clone()));

        // Emit error event
        self.emit_event(ModuleLifecycleEvent::Error { module_id, error });
    }

    /// Validate module state
    pub fn validate_state(&self, module_id: ModuleId) -> Result<()> {
        let state = self.get_state(module_id);

        match state {
            ModuleState::Error(ref error) => {
                Err(crate::AppError::Module(format!("Module in error state: {}", error)))
            }
            ModuleState::Unloaded => {
                Err(crate::AppError::Module("Module not loaded".to_string()))
            }
            _ => Ok(()),
        }
    }

    /// Get all module states
    pub fn get_all_states(&self) -> HashMap<ModuleId, ModuleState> {
        self.states.clone()
    }

    /// Cleanup unloaded modules
    pub fn cleanup_unloaded(&mut self) {
        self.states.retain(|_, state| *state != ModuleState::Unloaded);
    }

    /// Get performance summary
    pub fn get_performance_summary(&self) -> String {
        let mut summary = String::new();

        summary.push_str("=== Module Lifecycle Performance ===\n");

        for (module_id, state) in &self.states {
            summary.push_str(&format!("Module: {:?}\n", module_id));
            summary.push_str(&format!("  State: {:?}\n", state));

            let metrics = self.get_module_metrics(*module_id);
            summary.push_str(&format!("  Avg Load Time: {:.1}ms\n", metrics.load_time));
            summary.push_str(&format!("  Avg Activation Time: {:.1}ms\n", metrics.activation_time));
            summary.push_str(&format!("  Error Count: {}\n", metrics.error_count));
            summary.push('\n');
        }

        summary
    }
}

/// Module-specific metrics
#[derive(Debug)]
pub struct ModuleMetrics {
    pub load_time: f64,
    pub activation_time: f64,
    pub error_count: u32,
}

impl Default for ModuleLifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple console listener for debugging
pub struct ConsoleListener;

impl ModuleLifecycleListener for ConsoleListener {
    fn on_event(&self, event: &ModuleLifecycleEvent) {
        match event {
            ModuleLifecycleEvent::Loading { module_id } => {
                debug!("[Lifecycle] Loading module: {:?}", module_id);
            }
            ModuleLifecycleEvent::Loaded { module_id, duration } => {
                info!("[Lifecycle] Module loaded: {:?} ({}ms)", module_id, duration.as_millis());
            }
            ModuleLifecycleEvent::Activating { module_id } => {
                debug!("[Lifecycle] Activating module: {:?}", module_id);
            }
            ModuleLifecycleEvent::Activated { module_id, duration } => {
                info!("[Lifecycle] Module activated: {:?} ({}ms)", module_id, duration.as_millis());
            }
            ModuleLifecycleEvent::Deactivating { module_id } => {
                debug!("[Lifecycle] Deactivating module: {:?}", module_id);
            }
            ModuleLifecycleEvent::Deactivated { module_id, duration } => {
                info!("[Lifecycle] Module deactivated: {:?} ({}ms)", module_id, duration.as_millis());
            }
            ModuleLifecycleEvent::Error { module_id, error } => {
                error!("[Lifecycle] Module error: {:?} - {}", module_id, error);
            }
            ModuleLifecycleEvent::Unloading { module_id } => {
                debug!("[Lifecycle] Unloading module: {:?}", module_id);
            }
            ModuleLifecycleEvent::Unloaded { module_id, duration } => {
                info!("[Lifecycle] Module unloaded: {:?} ({}ms)", module_id, duration.as_millis());
            }
        }
    }
}

/// Metrics collector listener
pub struct MetricsCollector {
    metrics: Arc<RwLock<HashMap<ModuleId, Vec<std::time::Duration>>>>,
    event_type: String,
}

impl MetricsCollector {
    pub fn new(event_type: &str) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            event_type: event_type.to_string(),
        }
    }

    pub fn get_metrics(&self) -> HashMap<ModuleId, Vec<std::time::Duration>> {
        self.metrics.read().unwrap().clone()
    }
}

impl ModuleLifecycleListener for MetricsCollector {
    fn on_event(&self, event: &ModuleLifecycleEvent) {
        let (module_id, duration) = match event {
            ModuleLifecycleEvent::Loaded { module_id, duration } => (*module_id, *duration),
            ModuleLifecycleEvent::Activated { module_id, duration } => (*module_id, *duration),
            ModuleLifecycleEvent::Deactivated { module_id, duration } => (*module_id, *duration),
            ModuleLifecycleEvent::Unloaded { module_id, duration } => (*module_id, *duration),
            _ => return,
        };

        if let Ok(mut metrics) = self.metrics.write() {
            metrics.entry(module_id).or_insert_with(Vec::new).push(duration);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::{create_example_modules, ModuleRegistry};

    #[test]
    fn test_lifecycle_manager_creation() {
        let manager = ModuleLifecycleManager::new();
        assert!(manager.get_all_states().is_empty());
        assert!(manager.get_active_module().is_none());
    }

    #[test]
    fn test_module_state_transitions() {
        let mut manager = ModuleLifecycleManager::new();
        let module_id = crate::modules::ModuleId::Home;

        // Initial state should be Unloaded
        assert_eq!(manager.get_state(module_id), ModuleState::Unloaded);

        // Simulate state change
        manager.set_state(module_id, ModuleState::Loading);
        assert_eq!(manager.get_state(module_id), ModuleState::Loading);

        manager.set_state(module_id, ModuleState::Loaded);
        assert_eq!(manager.get_state(module_id), ModuleState::Loaded);

        manager.set_state(module_id, ModuleState::Active);
        assert_eq!(manager.get_state(module_id), ModuleState::Active);
        assert_eq!(manager.get_active_module(), Some(module_id));
    }

    #[test]
    fn test_lifecycle_listener() {
        let manager = ModuleLifecycleManager::new();
        let listener = ConsoleListener;

        manager.add_listener(Box::new(listener));

        // Emit test event
        let event = ModuleLifecycleEvent::Loading {
            module_id: crate::modules::ModuleId::Home,
        };
        manager.emit_event(event);
    }

    #[test]
    fn test_metrics_collection() {
        let collector = MetricsCollector::new("test");

        let event = ModuleLifecycleEvent::Loaded {
            module_id: crate::modules::ModuleId::Home,
            duration: std::time::Duration::from_millis(100),
        };

        collector.on_event(&event);

        let metrics = collector.get_metrics();
        assert!(metrics.contains_key(&crate::modules::ModuleId::Home));
    }

    #[test]
    fn test_module_metrics() {
        let manager = ModuleLifecycleManager::new();
        let module_id = crate::modules::ModuleId::TextEditor;

        // Simulate some activity
        manager.metrics.load_times.insert(module_id, vec![
            std::time::Duration::from_millis(100),
            std::time::Duration::from_millis(150),
        ]);
        manager.metrics.activation_times.insert(module_id, vec![
            std::time::Duration::from_millis(50),
            std::time::Duration::from_millis(75),
        ]);
        manager.metrics.error_counts.insert(module_id, 1);

        let metrics = manager.get_module_metrics(module_id);
        assert_eq!(metrics.load_time, 125.0); // (100 + 150) / 2
        assert_eq!(metrics.activation_time, 62.5); // (50 + 75) / 2
        assert_eq!(metrics.error_count, 1);
    }
}