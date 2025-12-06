//! File browser module for navigating and managing files

use crate::modules::ApplicationModule;
use crate::modules::ModuleId;
use egui::{Context, Ui};

/// File browser module
pub struct FileBrowser {
    current_path: String,
    selected_file: Option<String>,
    show_hidden: bool,
}

impl FileBrowser {
    /// Create a new file browser module
    pub fn new() -> Self {
        Self {
            current_path: std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("/"))
                .to_string_lossy()
                .to_string(),
            selected_file: None,
            show_hidden: false,
        }
    }
}

impl ApplicationModule for FileBrowser {
    fn id(&self) -> ModuleId {
        ModuleId::FileBrowser
    }

    fn name(&self) -> &str {
        "File Browser"
    }

    fn icon(&self) -> &str {
        "📁"
    }

    fn render(&mut self, ui: &mut Ui, _ctx: &Context) {
        ui.heading("📁 File Browser");

        ui.horizontal(|ui| {
            ui.label("Current Path:");
            ui.label(&self.current_path);
        });

        ui.separator();

        // Simple file list display
        ui.label("Files:");
        if let Ok(entries) = std::fs::read_dir(&self.current_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("??");

                if path.is_dir() {
                    if ui.button(format!("📁 {}", name)).clicked() {
                        self.current_path = path.to_string_lossy().to_string();
                    }
                } else {
                    if ui.button(format!("📄 {}", name)).clicked() {
                        self.selected_file = Some(path.to_string_lossy().to_string());
                    }
                }
            }
        } else {
            ui.label("Cannot read directory");
        }

        if let Some(ref selected) = self.selected_file {
            ui.separator();
            ui.label(format!("Selected: {}", selected));
        }
    }

    fn update(&mut self, _ctx: &Context) {
        // Update file watcher or refresh logic here
    }

    fn on_activate(&mut self) {
        // Refresh current directory when activated
        tracing::info!("File browser activated");
    }

    fn on_deactivate(&mut self) {
        tracing::info!("File browser deactivated");
    }
}