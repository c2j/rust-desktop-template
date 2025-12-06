//! Module system for the desktop application

use egui::{Context, Ui};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// Unique identifier for modules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModuleId {
    Home,
    FileBrowser,
    TextEditor,
    SystemMonitor,
    Settings,
    Custom(u32),
}

impl Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            ModuleId::Home => write!(f, "Home"),
            ModuleId::FileBrowser => write!(f, "File Browser"),
            ModuleId::TextEditor => write!(f, "Text Editor"),
            ModuleId::SystemMonitor => write!(f, "System Monitor"),
            ModuleId::Settings => write!(f, "Settings"),
            ModuleId::Custom(id) => write!(f, "Custom {}", id),
        }
    }
}

/// Trait that all application modules must implement
pub trait ApplicationModule: Send + Sync {
    /// Get the module's unique identifier
    fn id(&self) -> ModuleId;

    /// Get the module's display name
    fn name(&self) -> &str;

    /// Get the module's icon (emoji or icon identifier)
    fn icon(&self) -> &str;

    /// Get the module's description
    fn description(&self) -> &str {
        ""
    }

    /// Render the module's UI
    fn render(&mut self, ui: &mut Ui, ctx: &Context);

    /// Update module state (called every frame)
    fn update(&mut self, _ctx: &Context) {
        // Default implementation does nothing
    }

    /// Called when the module becomes active
    fn on_activate(&mut self) {
        // Default implementation does nothing
    }

    /// Called when the module becomes inactive
    fn on_deactivate(&mut self) {
        // Default implementation does nothing
    }

    /// Save module state (if persistent)
    fn save_state(&self) -> Option<serde_json::Value> {
        None
    }

    /// Load module state (if persistent)
    fn load_state(&mut self, _state: serde_json::Value) -> crate::Result<()> {
        Ok(())
    }
}

pub mod file_browser;
pub mod registry;
pub mod state;
pub mod system_monitor;
pub mod text_editor;

pub use registry::ModuleRegistry;

/// Example module implementations
pub fn create_example_modules() -> Vec<Box<dyn ApplicationModule>> {
    vec![
        Box::new(file_browser::FileBrowserModule::new()),
        Box::new(text_editor::TextEditorModule::new()),
        Box::new(system_monitor::SystemMonitorModule::new()),
    ]
}