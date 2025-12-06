//! Text editor module with basic editing capabilities

use crate::modules::ApplicationModule;
use crate::modules::ModuleId;
use egui::{Context, Ui};

/// Text editor module
pub struct TextEditor {
    content: String,
    modified: bool,
    cursor_pos: usize,
}

impl TextEditor {
    /// Create a new text editor module
    pub fn new() -> Self {
        Self {
            content: "Welcome to the Text Editor!\n\nYou can type your text here.".to_string(),
            modified: false,
            cursor_pos: 0,
        }
    }
}

impl ApplicationModule for TextEditor {
    fn id(&self) -> ModuleId {
        ModuleId::TextEditor
    }

    fn name(&self) -> &str {
        "Text Editor"
    }

    fn icon(&self) -> &str {
        "📝"
    }

    fn render(&mut self, ui: &mut Ui, _ctx: &Context) {
        ui.heading("📝 Text Editor");

        // Status bar
        ui.horizontal(|ui| {
            ui.label(format!("Length: {}", self.content.len()));
            ui.label(format!("Lines: {}", self.content.lines().count()));
            if self.modified {
                ui.label("🔴 Modified");
            }
        });

        ui.separator();

        // Text editor area
        let response = ui.text_edit_multiline(&mut self.content);
        if response.changed() {
            self.modified = true;
        }

        ui.separator();

        // Toolbar
        ui.horizontal(|ui| {
            if ui.button("🗑️ Clear").clicked() {
                self.content.clear();
                self.modified = true;
            }
            if ui.button("📋 Copy").clicked() {
                ui.output_mut(|o| o.copied_text = self.content.clone());
            }
        });
    }

    fn update(&mut self, _ctx: &Context) {
        // Update cursor position, auto-save, etc.
    }

    fn on_activate(&mut self) {
        tracing::info!("Text editor activated");
    }

    fn on_deactivate(&mut self) {
        if self.modified {
            tracing::info!("Text editor deactivated with unsaved changes");
        }
    }

    fn save_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "content": self.content,
            "modified": self.modified,
            "cursor_pos": self.cursor_pos
        }))
    }

    fn load_state(&mut self, state: serde_json::Value) -> crate::Result<()> {
        if let Ok(obj) = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(state) {
            if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
                self.content = content.to_string();
            }
            if let Some(modified) = obj.get("modified").and_then(|v| v.as_bool()) {
                self.modified = modified;
            }
            if let Some(cursor_pos) = obj.get("cursor_pos").and_then(|v| v.as_u64()) {
                self.cursor_pos = cursor_pos as usize;
            }
        }
        Ok(())
    }
}