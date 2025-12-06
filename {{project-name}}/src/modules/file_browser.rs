//! File browser module implementation

use crate::{
    error::Result,
    modules::{ApplicationModule, ModuleId},
    ui::Theme,
};
use egui::{Context, ScrollArea, Ui};
use std::path::PathBuf;
use tracing::{debug, error, info};

/// File browser module
#[derive(Debug)]
pub struct FileBrowserModule {
    id: ModuleId,
    current_path: PathBuf,
    items: Vec<FileItem>,
    selected_index: Option<usize>,
}

#[derive(Debug, Clone)]
struct FileItem {
    name: String,
    path: PathBuf,
    is_directory: bool,
    size: u64,
    modified: std::time::SystemTime,
}

impl FileBrowserModule {
    /// Create a new file browser module
    pub fn new() -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));

        Self {
            id: ModuleId::FileBrowser,
            current_path,
            items: Vec::new(),
            selected_index: None,
        }
    }

    /// Load directory contents
    fn load_directory(&mut self) -> Result<()> {
        info!("Loading directory: {:?}", self.current_path);

        self.items.clear();

        if !self.current_path.exists() {
            error!("Directory does not exist: {:?}", self.current_path);
            return Ok(());
        }

        // Read directory entries
        let entries = match std::fs::read_dir(&self.current_path) {
            Ok(entries) => entries,
            Err(e) => {
                error!("Failed to read directory: {}", e);
                return Ok(());
            }
        };

        // Process each entry
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name()
                .to_string_lossy()
                .to_string();

            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(_) => continue,
            };

            let file_item = FileItem {
                name,
                path,
                is_directory: metadata.is_dir(),
                size: metadata.len(),
                modified: metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now()),
            };

            self.items.push(file_item);
        }

        // Sort items: directories first, then files, both alphabetically
        self.items.sort_by(|a, b| {
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        debug!("Loaded {} items from {:?}", self.items.len(), self.current_path);
        Ok(())
    }

    /// Navigate to parent directory
    fn navigate_to_parent(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            info!("Navigating to parent: {:?}", parent);
            self.current_path = parent.to_path_buf();
            self.selected_index = None;
            if let Err(e) = self.load_directory() {
                error!("Failed to load parent directory: {}", e);
            }
        }
    }

    /// Navigate to child directory
    fn navigate_to_child(&mut self, path: &PathBuf) {
        info!("Navigating to child: {:?}", path);
        self.current_path = path.clone();
        self.selected_index = None;
        if let Err(e) = self.load_directory() {
            error!("Failed to load child directory: {}", e);
        }
    }

    /// Format file size
    fn format_size(&self, bytes: u64) -> String {
        const KIB: u64 = 1024;
        const MIB: u64 = KIB * KIB;
        const GIB: u64 = MIB * KIB;

        if bytes >= GIB {
            format!("{:.1} GB", bytes as f64 / GIB as f64)
        } else if bytes >= MIB {
            format!("{:.1} MB", bytes as f64 / MIB as f64)
        } else if bytes >= KIB {
            format!("{:.1} KB", bytes as f64 / KIB as f64)
        } else {
            format!("{} B", bytes)
        }
    }

    /// Format timestamp
    fn format_timestamp(&self, time: std::time::SystemTime) -> String {
        match time.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => {
                let datetime = chrono::DateTime::from_timestamp(
                    duration.as_secs() as i64,
                    duration.subsec_nanos(),
                );
                match datetime {
                    Some(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
                    None => "Unknown".to_string(),
                }
            }
            Err(_) => "Unknown".to_string(),
        }
    }

    /// Render navigation toolbar
    fn render_toolbar(&self, ui: &mut Ui, theme: &Theme) {
        ui.horizontal(|ui| {
            // Back button
            if ui.button("⬅️ Back").on_hover_text("Go to parent directory").clicked() {
                // TODO: Navigate to parent
                debug!("Back button clicked");
            }

            ui.add_space(theme.spacing.small);

            // Up button
            if ui.button("⬆️ Up").on_hover_text("Go to parent directory").clicked() {
                // TODO: Navigate to parent
                debug!("Up button clicked");
            }

            ui.add_space(theme.spacing.medium);

            // Current path
            let path_str = self.current_path.display().to_string();
            ui.label(egui::RichText::new(format!("📁 {}", path_str)).size(14.0));

            ui.add_space(theme.spacing.large);

            // Refresh button
            if ui.button("🔄 Refresh").on_hover_text("Refresh directory").clicked() {
                // TODO: Refresh directory
                debug!("Refresh button clicked");
            }
        });
    }
}

impl ApplicationModule for FileBrowserModule {
    fn id(&self) -> ModuleId {
        self.id
    }

    fn name(&self) -> &str {
        "File Browser"
    }

    fn icon(&self) -> &str {
        "📁"
    }

    fn description(&self) -> &str {
        "Browse and manage files and directories"
    }

    fn render(&mut self, ui: &mut Ui, ctx: &Context) {
        // Load directory if needed
        if self.items.is_empty() {
            if let Err(e) = self.load_directory() {
                ui.colored_label(egui::Color32::RED, format!("Error loading directory: {}", e));
                return;
            }
        }

        // Get theme for styling
        let theme = &ctx.data::<Theme>().unwrap_or(&Theme::dark());

        // Render toolbar
        self.render_toolbar(ui, theme);
        ui.add_space(theme.spacing.small);

        // Render file list
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                // Table header
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Name").strong().size(12.0));
                    ui.add_space(200.0);
                    ui.label(egui::RichText::new("Size").strong().size(12.0));
                    ui.add_space(80.0);
                    ui.label(egui::RichText::new("Modified").strong().size(12.0));
                });
                ui.add_space(theme.spacing.small);

                // File items
                for (index, item) in self.items.iter().enumerate() {
                    let is_selected = self.selected_index == Some(index);

                    ui.horizontal(|ui| {
                        if is_selected {
                            ui.painter().rect_filled(
                                ui.available_rect_before_wrap(),
                                0.0,
                                theme.colors.primary.with_alpha(0.2),
                            );
                        }

                        // Icon and name
                        let icon = if item.is_directory { "📁" } else { "📄" };
                        ui.label(egui::RichText::new(format!("{} {}", icon, item.name)).size(12.0));

                        if ui.available_width_before_wrap() > 100.0 {
                            ui.add_space(200.0);

                            // Size
                            let size_str = if item.is_directory {
                                "—".to_string()
                            } else {
                                self.format_size(item.size)
                            };
                            ui.label(egui::RichText::new(size_str).size(11.0).monospace());

                            ui.add_space(80.0);

                            // Modified time
                            let time_str = self.format_timestamp(item.modified);
                            ui.label(egui::RichText::new(time_str).size(11.0).monospace());
                        }
                    });

                    // Handle item click
                    if ui.response_contains_pointer(ui.available_rect_before_wrap()) {
                        if ui.input(|i| i.pointer.primary_clicked()) {
                            self.selected_index = Some(index);

                            if item.is_directory {
                                // Navigate to directory
                                self.navigate_to_child(&item.path);
                            } else {
                                // Open file
                                debug!("Open file: {:?}", item.path);
                                // TODO: Implement file opening
                            }
                        }
                    }

                    ui.add_space(theme.spacing.small);
                }
            });

        // Status bar
        ui.add_space(theme.spacing.medium);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("{} items", self.items.len())).size(11.0).weak());

            if let Some(selected) = self.selected_index {
                if let Some(item) = self.items.get(selected) {
                    ui.add_space(theme.spacing.medium);
                    ui.label(egui::RichText::new(format!("Selected: {}", item.name)).size(11.0).weak());
                }
            }
        });
    }

    fn on_activate(&mut self) {
        info!("File browser module activated");
        // Load directory when activated
        if let Err(e) = self.load_directory() {
            error!("Failed to load directory on activation: {}", e);
        }
    }

    fn on_deactivate(&mut self) {
        info!("File browser module deactivated");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_browser_creation() {
        let module = FileBrowserModule::new();
        assert_eq!(module.id(), ModuleId::FileBrowser);
        assert_eq!(module.name(), "File Browser");
        assert_eq!(module.icon(), "📁");
    }

    #[test]
    fn test_file_size_formatting() {
        let module = FileBrowserModule::new();

        assert_eq!(module.format_size(512), "512 B");
        assert_eq!(module.format_size(1536), "1.5 KB");
        assert_eq!(module.format_size(2097152), "2.0 MB");
        assert_eq!(module.format_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_directory_navigation() {
        let mut module = FileBrowserModule::new();
        let initial_path = module.current_path.clone();

        // Should be able to navigate to parent if not at root
        if let Some(parent) = initial_path.parent() {
            module.navigate_to_parent();
            assert_ne!(module.current_path, initial_path);
        }
    }
}