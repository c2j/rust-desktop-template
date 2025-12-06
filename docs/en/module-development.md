# Module Development Guide

This comprehensive guide will teach you how to create custom modules for the Rust Desktop Template framework.

## 🎯 Module Overview

Modules are the primary way to extend the functionality of your desktop application. Each module is a self-contained unit of functionality that can be dynamically loaded, activated, and deactivated.

### What Can Modules Do?

- **Render UI Components**: Display custom user interfaces
- **Handle User Input**: Respond to mouse, keyboard, and touch events
- **Manage State**: Maintain persistent and temporary state
- **File Operations**: Read and write files (with permissions)
- **Network Access**: Make HTTP requests (with permissions)
- **System Integration**: Access system resources (with permissions)

## 🏗️ Module Architecture

### Core Trait

All modules implement the `ApplicationModule` trait:

```rust
use crate::modules::{ApplicationModule, ModuleId};
use egui::{Context, Ui};

pub trait ApplicationModule: Send + Sync {
    /// Unique identifier for the module
    fn id(&self) -> ModuleId;

    /// Human-readable name displayed in UI
    fn name(&self) -> &str;

    /// Icon emoji or unicode character
    fn icon(&self) -> &str;

    /// Main rendering function
    fn render(&mut self, ui: &mut Ui, ctx: &Context);

    /// Called every frame, before rendering
    fn update(&mut self, _ctx: &Context) {}

    /// Called when module becomes active
    fn on_activate(&mut self) {}

    /// Called when module becomes inactive
    fn on_deactivate(&mut self) {}

    /// Optional: Save module state
    fn save_state(&self) -> Option<serde_json::Value> { None }

    /// Optional: Restore module state
    fn load_state(&mut self, _state: serde_json::Value) {}

    /// Optional: Module metadata
    fn version(&self) -> &str { "1.0.0" }
    fn description(&self) -> &str { "" }
    fn author(&self) -> &str { "" }
}
```

### Module Registration

Modules are registered with the system in `src/modules/mod.rs`:

```rust
pub fn create_example_modules() -> Vec<Box<dyn ApplicationModule>> {
    vec![
        Box::new(FileBrowser::new()),
        Box::new(TextEditor::new()),
        Box::new(SystemMonitor::new()),
        // Add your custom module here
        Box::new(MyCustomModule::new()),
    ]
}
```

## 🛠️ Creating Your First Module

### Step 1: Create Module File

Create `src/modules/my_custom_module.rs`:

```rust
//! My Custom Module

use crate::modules::{ApplicationModule, ModuleId};
use egui::{Context, Ui};
use std::time::Instant;

pub struct MyCustomModule {
    /// Module state
    counter: u32,
    last_update: Instant,
    user_input: String,
    enabled: bool,
}

impl MyCustomModule {
    pub fn new() -> Self {
        Self {
            counter: 0,
            last_update: Instant::now(),
            user_input: String::new(),
            enabled: true,
        }
    }
}

impl ApplicationModule for MyCustomModule {
    fn id(&self) -> ModuleId {
        // Use a custom ID for your module
        ModuleId::Custom(1)
    }

    fn name(&self) -> &str {
        "My Custom Module"
    }

    fn icon(&self) -> &str {
        "🚀"  // Choose an appropriate emoji or icon
    }

    fn render(&mut self, ui: &mut Ui, _ctx: &Context) {
        ui.heading("🚀 My Custom Module");

        // Module content goes here
        ui.separator();

        ui.checkbox(&mut self.enabled, "Enable Feature");

        if self.enabled {
            ui.label(format!("Counter: {}", self.counter));

            if ui.button("Increment").clicked() {
                self.counter += 1;
            }

            ui.horizontal(|ui| {
                ui.label("Input:");
                ui.text_edit_singleline(&mut self.user_input);
            });

            if !self.user_input.is_empty() {
                ui.label(format!("You typed: {}", self.user_input));
            }
        }
    }

    fn update(&mut self, _ctx: &Context) {
        // Update logic that runs every frame
        if self.last_update.elapsed().as_secs() >= 1 {
            // Update once per second
            self.last_update = Instant::now();
        }
    }

    fn on_activate(&mut self) {
        println!("MyCustomModule activated");
    }

    fn on_deactivate(&mut self) {
        println!("MyCustomModule deactivated");
    }

    fn save_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "counter": self.counter,
            "user_input": self.user_input,
            "enabled": self.enabled
        }))
    }

    fn load_state(&mut self, state: serde_json::Value) {
        if let Ok(obj) = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(state) {
            if let Some(counter) = obj.get("counter").and_then(|v| v.as_u64()) {
                self.counter = counter as u32;
            }
            if let Some(input) = obj.get("user_input").and_then(|v| v.as_str()) {
                self.user_input = input.to_string();
            }
            if let Some(enabled) = obj.get("enabled").and_then(|v| v.as_bool()) {
                self.enabled = enabled;
            }
        }
    }
}
```

### Step 2: Add to Module Registry

Update `src/modules/mod.rs`:

```rust
// Add the module declaration
mod file_browser;
mod text_editor;
mod system_monitor;
mod my_custom_module;  // ← Add this line

// Re-export the module
pub use file_browser::FileBrowser;
pub use text_editor::TextEditor;
pub use system_monitor::SystemMonitor;
pub use my_custom_module::MyCustomModule;  // ← Add this line

// Update the create_example_modules function
pub fn create_example_modules() -> Vec<Box<dyn ApplicationModule>> {
    vec![
        Box::new(FileBrowser::new()),
        Box::new(TextEditor::new()),
        Box::new(SystemMonitor::new()),
        Box::new(MyCustomModule::new()),  // ← Add this line
    ]
}
```

### Step 3: Update ModuleId Enum (Optional)

If you want a named ID for your module, update `src/modules/mod.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModuleId {
    Home,
    FileBrowser,
    TextEditor,
    SystemMonitor,
    Settings,
    MyCustom,  // ← Add this line
    Custom(u32), // For dynamically loaded modules
}

// Update the Display implementation
impl std::fmt::Display for ModuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleId::Home => write!(f, "home"),
            ModuleId::FileBrowser => write!(f, "file_browser"),
            ModuleId::TextEditor => write!(f, "text_editor"),
            ModuleId::SystemMonitor => write!(f, "system_monitor"),
            ModuleId::Settings => write!(f, "settings"),
            ModuleId::MyCustom => write!(f, "my_custom"),  // ← Add this line
            ModuleId::Custom(id) => write!(f, "custom_{}", id),
        }
    }
}
```

Then update your module to use the named ID:

```rust
fn id(&self) -> ModuleId {
    ModuleId::MyCustom  // ← Use named ID instead of Custom(1)
}
```

## 🎨 Advanced UI Development

### Complex Layouts

Use egui's layout system for complex UIs:

```rust
fn render(&mut self, ui: &mut Ui, _ctx: &Context) {
    ui.heading("Advanced Module");

    // Two-column layout
    ui.columns(2, |columns| {
        // Left column
        columns[0].vertical(|ui| {
            ui.label("Left Column");
            ui.button("Button 1");
            ui.button("Button 2");
        });

        // Right column
        columns[1].vertical(|ui| {
            ui.label("Right Column");
            ui.add(egui::Slider::new(&mut self.value, 0.0..=100.0));
        });
    });

    // Horizontal layout
    ui.horizontal(|ui| {
        ui.label("Items:");
        for i in 0..5 {
            ui.button(format!("Item {}", i));
        }
    });

    // Scrollable area
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for i in 0..100 {
                ui.label(format!("Scrollable item {}", i));
            }
        });
}
```

### Interactive Widgets

```rust
use egui::{Color32, RichText};

fn render(&mut self, ui: &mut Ui, ctx: &Context) {
    // Text input
    ui.horizontal(|ui| {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.name);
    });

    // Multi-line text
    ui.label("Description:");
    ui.text_edit_multiline(&mut self.description);

    // Checkboxes
    ui.checkbox(&mut self.enabled, "Enabled");
    ui.checkbox(&mut self.debug_mode, "Debug Mode");

    // Radio buttons
    ui.radio_value(&mut self.mode, Mode::Simple, "Simple");
    ui.radio_value(&mut self.mode, Mode::Advanced, "Advanced");

    // Dropdown
    egui::ComboBox::from_label("Select Option")
        .selected_text(format!("{:?}", self.selected_option))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut self.selected_option, Option::A, "Option A");
            ui.selectable_value(&mut self.selected_option, Option::B, "Option B");
            ui.selectable_value(&mut self.selected_option, Option::C, "Option C");
        });

    // Sliders
    ui.add(egui::Slider::new(&mut self.progress, 0.0..=1.0).text("Progress"));
    ui.add(egui::Slider::new(&mut self.count, 0..=100).text("Count"));

    // Color picker
    ui.color_edit_button_srgba(&mut self.color);

    // Buttons with styling
    if ui.add(
        egui::Button::new(RichText::new("Important Action").color(Color32::RED))
            .fill(Color32::from_rgb(255, 200, 200))
    ).clicked() {
        // Handle button click
    }
}
```

### Menus and Popups

```rust
fn render(&mut self, ui: &mut Ui, ctx: &Context) {
    // Menu bar
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("New").clicked() {
                    self.new_file();
                }
                if ui.button("Open").clicked() {
                    self.open_file();
                }
                if ui.button("Save").clicked() {
                    self.save_file();
                }
            });

            egui::menu::menu(ui, "Edit", |ui| {
                if ui.button("Copy").clicked() {
                    self.copy();
                }
                if ui.button("Paste").clicked() {
                    self.paste();
                }
            });
        });
    });

    // Context menu (right-click menu)
    let response = ui.label("Right-click me");
    response.context_menu(|ui| {
        if ui.button("Context Action 1").clicked() {
            self.context_action_1();
        }
        if ui.button("Context Action 2").clicked() {
            self.context_action_2();
        }
    });

    // Popup window
    if ui.button("Show Popup").clicked() {
        self.show_popup = true;
    }

    if self.show_popup {
        egui::Window::new("Popup Window")
            .collapsible(false)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("This is a popup window!");

                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        self.show_popup = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.show_popup = false;
                    }
                });
            });
    }
}
```

## 📁 File Operations

### File Browser Integration

```rust
use std::path::PathBuf;

pub struct FileManager {
    current_dir: PathBuf,
    selected_file: Option<PathBuf>,
    file_content: String,
}

impl FileManager {
    fn load_file(&mut self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        self.file_content = content;
        self.selected_file = Some(path.clone());
        Ok(())
    }

    fn save_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(path, &self.file_content)?;
        Ok(())
    }

    fn render_file_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Current File:");
            if let Some(path) = &self.selected_file {
                ui.label(path.display().to_string());
            } else {
                ui.label("No file selected");
            }
        });

        if ui.button("Load File").clicked() {
            // Show file dialog (simplified)
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Text Files", &["txt"])
                .pick_file()
            {
                let _ = self.load_file(&path);
            }
        }

        if ui.button("Save File").clicked() {
            if let Some(path) = &self.selected_file {
                let _ = self.save_file(path);
            }
        }

        ui.separator();

        // File content editor
        ui.label("File Content:");
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.text_edit_multiline(&mut self.file_content);
            });
    }
}
```

### Directory Operations

```rust
use std::fs;

impl FileManager {
    fn list_directory(&self, dir: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            entries.push(entry.path());
        }
        Ok(entries)
    }

    fn create_directory(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        fs::create_dir_all(path)
    }

    fn delete_file_or_directory(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        }
    }
}
```

## 🌐 Network Operations

### HTTP Requests

```rust
use ehttp;

pub struct NetworkModule {
    response_text: String,
    loading: bool,
}

impl NetworkModule {
    fn fetch_data(&mut self, url: &str) {
        self.loading = true;
        let url = url.to_string();

        // Spawn a background task for the request
        let (sender, receiver) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            let request = ehttp::Request::get(url);
            ehttp::fetch_blocking(request, move |result| {
                let _ = sender.send(result);
            });
        });

        // In a real implementation, you'd handle the response in update()
        // This is a simplified example
    }

    fn render_network_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("URL:");
            ui.text_edit_singleline(&mut self.url);
        });

        if ui.button("Fetch").clicked() {
            self.fetch_data(&self.url);
        }

        if self.loading {
            ui.spinner();
            ui.label("Loading...");
        }

        if !self.response_text.is_empty() {
            ui.separator();
            ui.label("Response:");
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.label(&self.response_text);
                });
        }
    }
}
```

## ⚙️ Configuration and Settings

### Module Configuration

Create `config/modules/my_custom_module.toml`:

```toml
[module]
enabled = true
auto_load = true
auto_activate = false

[ui]
show_in_navigation = true
navigation_position = 5
shortcut = "Ctrl+Shift+M"

[permissions]
file_system_access = true
network_access = true
system_info_access = false
settings_access = true

[settings]
default_value = 42
api_url = "https://api.example.com"
max_retries = 3
timeout_seconds = 30

[appearance]
theme = "dark"
font_size = 14
show_toolbar = true
```

### Reading Configuration

```rust
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct ModuleConfig {
    pub module: ModuleSettings,
    pub ui: UISettings,
    pub permissions: PermissionSettings,
    pub settings: CustomSettings,
    pub appearance: AppearanceSettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomSettings {
    pub default_value: i32,
    pub api_url: String,
    pub max_retries: u32,
    pub timeout_seconds: u64,
}

impl MyCustomModule {
    fn load_config() -> Result<ModuleConfig, Box<dyn std::error::Error>> {
        let config_path = "config/modules/my_custom_module.toml";
        let content = std::fs::read_to_string(config_path)?;
        let config: ModuleConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
```

## 🎯 State Management

### Local State

```rust
pub struct StatefulModule {
    // Simple state
    counter: u32,

    // Complex state
    data: HashMap<String, serde_json::Value>,

    // Temporary state
    temporary_data: Option<String>,
}

impl StatefulModule {
    fn update_counter(&mut self) {
        self.counter += 1;
    }

    fn set_data(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }
}
```

### Shared Application State

```rust
use std::sync::{Arc, RwLock};

pub struct SharedStateModule {
    shared_state: Arc<RwLock<crate::AppState>>,
}

impl SharedStateModule {
    fn update_shared_state(&self) {
        if let Ok(mut state) = self.shared_state.write() {
            state.custom_data = "Updated from module".to_string();
        }
    }

    fn read_shared_state(&self) -> String {
        if let Ok(state) = self.shared_state.read() {
            state.custom_data.clone()
        } else {
            "Error accessing shared state".to_string()
        }
    }
}
```

### Persistent State

```rust
impl ApplicationModule for MyPersistentModule {
    fn save_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "counter": self.counter,
            "user_settings": {
                "theme": self.current_theme,
                "language": self.language,
            },
            "last_active": chrono::Utc::now().to_rfc3339(),
            "custom_data": self.custom_data
        }))
    }

    fn load_state(&mut self, state: serde_json::Value) {
        if let Ok(obj) = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(state) {
            if let Some(counter) = obj.get("counter").and_then(|v| v.as_u64()) {
                self.counter = counter as u32;
            }

            if let Some(settings) = obj.get("user_settings").and_then(|v| v.as_object()) {
                if let Some(theme) = settings.get("theme").and_then(|v| v.as_str()) {
                    self.current_theme = theme.to_string();
                }
                if let Some(language) = settings.get("language").and_then(|v| v.as_str()) {
                    self.language = language.to_string();
                }
            }

            self.custom_data = obj.get("custom_data").cloned();
        }
    }
}
```

## 🔧 Error Handling

### Result Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("File operation failed: {0}")]
    FileOperation(#[from] std::io::Error),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

impl MyCustomModule {
    fn risky_operation(&mut self) -> Result<(), ModuleError> {
        // Configuration error
        if self.config.api_url.is_empty() {
            return Err(ModuleError::Config("API URL is required".to_string()));
        }

        // File operation error
        std::fs::write("output.txt", "data")?;

        Ok(())
    }
}
```

### Error Display

```rust
fn render(&mut self, ui: &mut Ui, _ctx: &Context) {
    if let Some(error) = &self.last_error {
        ui.colored_label(egui::Color32::RED, format!("Error: {}", error));

        if ui.button("Clear Error").clicked() {
            self.last_error = None;
        }
    }

    // Wrap operations in error handling
    if ui.button("Perform Operation").clicked() {
        match self.risky_operation() {
            Ok(_) => {
                self.status = "Operation completed successfully".to_string();
            }
            Err(e) => {
                self.last_error = Some(e);
                self.status = "Operation failed".to_string();
            }
        }
    }
}
```

## 🧪 Testing Modules

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = MyCustomModule::new();
        assert_eq!(module.name(), "My Custom Module");
        assert_eq!(module.icon(), "🚀");
    }

    #[test]
    fn test_counter_increment() {
        let mut module = MyCustomModule::new();
        let initial = module.counter;
        module.update_counter();
        assert_eq!(module.counter, initial + 1);
    }

    #[test]
    fn test_state_serialization() {
        let module = MyCustomModule::new();
        let state = module.save_state();
        assert!(state.is_some());

        let state_obj = state.unwrap();
        assert!(state_obj.get("counter").is_some());
    }
}
```

### Integration Tests

```rust
// tests/integration/my_custom_module_test.rs
use my_desktop_app::modules::{ApplicationModule, MyCustomModule};
use egui::{Context, Vec2};

#[test]
fn test_module_integration() {
    let mut module = MyCustomModule::new();

    // Test activation/deactivation
    module.on_activate();
    module.on_deactivate();

    // Test state persistence
    let initial_state = module.save_state();
    assert!(initial_state.is_some());

    // Test state restoration
    let mut restored_module = MyCustomModule::new();
    if let Some(state) = initial_state {
        restored_module.load_state(state);
    }
}
```

## 📚 Best Practices

### Performance

- **Lazy Loading**: Load expensive resources only when needed
- **Caching**: Cache expensive computations
- **Minimal UI Updates**: Only update UI elements that have changed
- **Background Tasks**: Move long-running operations to background threads

```rust
pub struct PerformantModule {
    cached_data: Option<ExpensiveData>,
    last_cache_update: std::time::Instant,
}

impl PerformantModule {
    fn get_data(&mut self) -> &ExpensiveData {
        if self.cached_data.is_none() ||
           self.last_cache_update.elapsed().as_secs() > 60 {
            self.cached_data = Some(self.compute_expensive_data());
            self.last_cache_update = std::time::Instant::now();
        }
        self.cached_data.as_ref().unwrap()
    }
}
```

### User Experience

- **Responsive UI**: Keep the UI responsive during long operations
- **Visual Feedback**: Provide visual feedback for user actions
- **Keyboard Shortcuts**: Support keyboard shortcuts for common actions
- **Accessibility**: Use proper contrast and support screen readers

```rust
fn render_responsive_ui(&mut self, ui: &mut Ui, ctx: &Context) {
    // Show loading indicator during long operations
    if self.is_loading {
        ui.spinner();
        ui.label("Processing...");
        return;
    }

    // Use async operations for long tasks
    if ui.button("Start Long Operation").clicked() {
        self.is_loading = true;
        self.start_background_task(ctx);
    }
}
```

### Error Handling

- **Graceful Degradation**: Handle errors gracefully without crashing
- **User-Friendly Messages**: Show understandable error messages
- **Logging**: Log detailed error information for debugging
- **Recovery**: Provide ways to recover from error states

### Security

- **Input Validation**: Validate all user input
- **Permission Checks**: Respect the module's permission settings
- **Secure Storage**: Encrypt sensitive data
- **Resource Limits**: Respect resource consumption limits

---

This guide provides everything you need to create rich, powerful modules for your desktop applications. Experiment with different approaches and find what works best for your specific use case!