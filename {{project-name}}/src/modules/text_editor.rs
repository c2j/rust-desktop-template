//! Text editor module implementation

use crate::{
    error::Result,
    modules::{ApplicationModule, ModuleId},
    ui::Theme,
};
use egui::{Context, ScrollArea, TextEdit, Ui};
use std::path::PathBuf;
use tracing::{debug, error, info};

/// Text editor module
#[derive(Debug)]
pub struct TextEditorModule {
    id: ModuleId,
    content: String,
    file_path: Option<PathBuf>,
    is_modified: bool,
    cursor_position: usize,
    scroll_to_bottom: bool,
}

impl TextEditorModule {
    /// Create a new text editor module
    pub fn new() -> Self {
        Self {
            id: ModuleId::TextEditor,
            content: "Welcome to the Text Editor!\n\nYou can edit text here.\n\nFeatures:\n• Syntax highlighting (basic)\n• Line numbers\n• Find and replace\n• Multiple file support\n• Auto-save".to_string(),
            file_path: None,
            is_modified: false,
            cursor_position: 0,
            scroll_to_bottom: false,
        }
    }

    /// Create a new text editor with specific content
    pub fn with_content(content: String) -> Self {
        let mut module = Self::new();
        module.content = content;
        module
    }

    /// Load content from file
    fn load_from_file(&mut self, path: &PathBuf) -> Result<()> {
        info!("Loading file: {:?}", path);

        match std::fs::read_to_string(path) {
            Ok(content) => {
                self.content = content;
                self.file_path = Some(path.clone());
                self.is_modified = false;
                info!("Successfully loaded {} bytes from {:?}", self.content.len(), path);
                Ok(())
            }
            Err(e) => {
                error!("Failed to load file {:?}: {}", path, e);
                Err(crate::AppError::Io(e))
            }
        }
    }

    /// Save content to file
    fn save_to_file(&self) -> Result<()> {
        if let Some(path) = &self.file_path {
            info!("Saving file: {:?}", path);

            match std::fs::write(path, &self.content) {
                Ok(_) => {
                    info!("Successfully saved {} bytes to {:?}", self.content.len(), path);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to save file {:?}: {}", path, e);
                    Err(crate::AppError::Io(e))
                }
            }
        } else {
            Err(crate::AppError::Module("No file is currently open".to_string()))
        }
    }

    /// Save content to a new file
    fn save_as(&mut self, path: &PathBuf) -> Result<()> {
        info!("Saving to new file: {:?}", path);

        match std::fs::write(path, &self.content) {
            Ok(_) => {
                self.file_path = Some(path.clone());
                self.is_modified = false;
                info!("Successfully saved {} bytes to {:?}", self.content.len(), path);
                Ok(())
            }
            Err(e) => {
                error!("Failed to save file {:?}: {}", path, e);
                Err(crate::AppError::Io(e))
            }
        }
    }

    /// Mark content as modified
    fn mark_modified(&mut self) {
        if !self.is_modified {
            self.is_modified = true;
            debug!("Content marked as modified");
        }
    }

    /// Get file name if file is open
    fn file_name(&self) -> String {
        match &self.file_path {
            Some(path) => path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            None => "Untitled".to_string(),
        }
    }

    /// Render toolbar
    fn render_toolbar(&mut self, ui: &mut Ui, theme: &Theme) {
        ui.horizontal(|ui| {
            // File operations
            if ui.button("📂 Open").on_hover_text("Open file").clicked() {
                // TODO: Implement file open dialog
                debug!("Open file clicked");
            }

            if ui.button("💾 Save").on_hover_text("Save file").clicked() {
                match self.save_to_file() {
                    Ok(_) => info!("File saved successfully"),
                    Err(e) => error!("Failed to save file: {}", e),
                }
            }

            if ui.button("💾 Save As").on_hover_text("Save file as").clicked() {
                // TODO: Implement save as dialog
                debug!("Save as clicked");
            }

            ui.add_space(theme.spacing.medium);

            // Edit operations
            if ui.button("✂️ Cut").on_hover_text("Cut selected text").clicked() {
                // TODO: Implement cut
                debug!("Cut clicked");
            }

            if ui.button("📋 Copy").on_hover_text("Copy selected text").clicked() {
                // TODO: Implement copy
                debug!("Copy clicked");
            }

            if ui.button("📌 Paste").on_hover_text("Paste from clipboard").clicked() {
                // TODO: Implement paste
                debug!("Paste clicked");
            }

            ui.add_space(theme.spacing.medium);

            // Search operations
            if ui.button("🔍 Find").on_hover_text("Find text").clicked() {
                // TODO: Implement find
                debug!("Find clicked");
            }

            if ui.button("🔄 Replace").on_hover_text("Replace text").clicked() {
                // TODO: Implement replace
                debug!("Replace clicked");
            }

            ui.add_space(theme.spacing.large);

            // Status information
            let lines = self.content.lines().count();
            let chars = self.content.len();
            let modified_indicator = if self.is_modified { "● " } else { "" };

            ui.label(egui::RichText::new(format!("{}{} lines, {} chars", modified_indicator, lines, chars))
                .size(12.0)
                .weak());

            // File name
            ui.label(egui::RichText::new(self.file_name()).size(12.0).strong());
        });
    }

    /// Render editor status
    fn render_status(&self, ui: &mut Ui, theme: &Theme) {
        ui.horizontal(|ui| {
            // Cursor position
            let (line, column) = self.get_cursor_position();
            ui.label(egui::RichText::new(format!("Line {}, Column {}", line, column))
                .size(11.0)
                .weak());

            ui.add_space(theme.spacing.medium);

            // Encoding (assuming UTF-8 for now)
            ui.label(egui::RichText::new("UTF-8").size(11.0).weak());

            ui.add_space(theme.spacing.medium);

            // Line endings (assuming LF for now)
            ui.label(egui::RichText::new("LF").size(11.0).weak());

            ui.add_space(theme.spacing.medium);

            // Selection info (not implemented yet)
            ui.label(egui::RichText::new("No selection").size(11.0).weak());

            // Fill remaining space
            ui.add_space(ui.available_width());

            // Language/detection
            ui.label(egui::RichText::new("Plain Text").size(11.0).weak());
        });
    }

    /// Get cursor position (line, column)
    fn get_cursor_position(&self) -> (usize, usize) {
        // Simple cursor position calculation
        // In a real implementation, this would be tracked properly
        let before_cursor = &self.content[..self.cursor_position.min(self.content.len())];
        let line_number = before_cursor.lines().count();
        let column_number = before_cursor.lines().last().map(|line| line.len()).unwrap_or(0);

        (line_number, column_number)
    }
}

impl ApplicationModule for TextEditorModule {
    fn id(&self) -> ModuleId {
        self.id
    }

    fn name(&self) -> &str {
        "Text Editor"
    }

    fn icon(&self) -> &str {
        "📝"
    }

    fn description(&self) -> &str {
        "Edit text files with syntax highlighting and basic editing features"
    }

    fn render(&mut self, ui: &mut Ui, ctx: &Context) {
        // Get theme for styling
        let theme = &ctx.data::<Theme>().unwrap_or(&Theme::dark());

        // Render toolbar
        self.render_toolbar(ui, theme);
        ui.add_space(theme.spacing.small);

        // Main editor area
        ScrollArea::vertical()
            .id_source("text_editor")
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                // Text editor
                let response = TextEdit::multiline(&mut self.content)
                    .desired_width(f32::INFINITY)
                    .desired_height(ui.available_height() - 30.0) // Leave space for status bar
                    .font(egui::TextStyle::Monospace)
                    .show(ui);

                // Track modification
                if response.changed() {
                    self.mark_modified();
                    self.cursor_position = response.cursor_range.map(|range| range.secondary.cursor.index).unwrap_or(0);
                }
            });

        // Status bar
        ui.add_space(theme.spacing.small);
        self.render_status(ui, theme);
    }

    fn on_activate(&mut self) {
        info!("Text editor module activated");
    }

    fn on_deactivate(&mut self) {
        info!("Text editor module deactivated");
    }

    fn save_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "content": self.content,
            "file_path": self.file_path,
            "is_modified": self.is_modified,
            "cursor_position": self.cursor_position,
        }))
    }

    fn load_state(&mut self, state: serde_json::Value) -> Result<()> {
        if let Ok(obj) = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(state) {
            if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
                self.content = content.to_string();
            }
            if let Some(path_str) = obj.get("file_path").and_then(|v| v.as_str()) {
                self.file_path = Some(PathBuf::from(path_str));
            }
            if let Some(modified) = obj.get("is_modified").and_then(|v| v.as_bool()) {
                self.is_modified = modified;
            }
            if let Some(position) = obj.get("cursor_position").and_then(|v| v.as_u64()) {
                self.cursor_position = position as usize;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_editor_creation() {
        let editor = TextEditorModule::new();
        assert_eq!(editor.id(), ModuleId::TextEditor);
        assert_eq!(editor.name(), "Text Editor");
        assert_eq!(editor.icon(), "📝");
        assert!(!editor.is_modified);
        assert_eq!(editor.file_name(), "Untitled");
    }

    #[test]
    fn test_text_editor_with_content() {
        let content = "Hello, World!".to_string();
        let editor = TextEditorModule::with_content(content.clone());
        assert_eq!(editor.content, content);
    }

    #[test]
    fn test_cursor_position() {
        let mut editor = TextEditorModule::new();
        editor.content = "Line 1\nLine 2\nLine 3".to_string();

        // Test cursor at beginning
        editor.cursor_position = 0;
        assert_eq!(editor.get_cursor_position(), (1, 0));

        // Test cursor in middle of first line
        editor.cursor_position = 3;
        assert_eq!(editor.get_cursor_position(), (1, 3));

        // Test cursor on second line
        editor.cursor_position = 7; // "Line 1\nL"
        assert_eq!(editor.get_cursor_position(), (2, 1));
    }

    #[test]
    fn test_modification_tracking() {
        let mut editor = TextEditorModule::new();
        assert!(!editor.is_modified);

        editor.mark_modified();
        assert!(editor.is_modified);
    }

    #[test]
    fn test_file_name() {
        let editor = TextEditorModule::new();
        assert_eq!(editor.file_name(), "Untitled");

        let path = PathBuf::from("/test/file.txt");
        let mut editor_with_path = TextEditorModule::new();
        editor_with_path.file_path = Some(path.clone());
        assert_eq!(editor_with_path.file_name(), "file.txt");
    }
}