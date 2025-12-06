//! Status bar component for displaying system information and application status

use crate::error::Result;
use egui::{Context, Response, RichText, Ui};
use std::sync::{Arc, RwLock};
use tracing::{debug, warn};

/// Status bar component
#[derive(Debug)]
pub struct StatusBar {
    /// Last update time
    last_update: chrono::DateTime<chrono::Local>,
}

impl StatusBar {
    /// Create a new status bar
    pub fn new() -> Self {
        Self {
            last_update: chrono::Local::now(),
        }
    }

    /// Render the status bar UI
    pub fn render(&mut self, ui: &mut Ui, ctx: &Context, state: &Arc<RwLock<crate::AppState>>) {
        // Update last update time
        self.last_update = chrono::Local::now();

        // Create status bar layout
        ui.horizontal(|ui| {
            // Left side - Application status
            self.render_left_status(ui, state);

            // Spacer
            ui.allocate_ui_with_layout(egui::Vec2::new(ui.available_width() - 300.0, ui.available_height()), egui::Layout::default(), |ui| {
                // Middle section - Flexible spacer
                ui.add_space(ui.available_width());
            });

            // Right side - System information
            self.render_right_status(ui, state);
        });
    }

    /// Render left side status information
    fn render_left_status(&self, ui: &mut Ui, state: &Arc<RwLock<crate::AppState>>) {
        let state = match state.read() {
            Ok(state) => state,
            Err(e) => {
                warn!("Failed to read application state: {}", e);
                ui.label("Application State Error");
                return;
            }
        };

        // Application name and version
        ui.label(RichText::new("{{project-name}}").size(12.0));
        ui.label(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))).size(10.0).weak());

        ui.separator();

        // Current module
        if let Some(active_module) = state.active_module {
            if let Some(module) = state.modules.get(active_module) {
                ui.label(RichText::new(format!("📦 {}", module.name())).size(10.0));
            }
        } else {
            ui.label(RichText::new("📦 No Module").size(10.0).weak());
        }
    }

    /// Render right side system information
    fn render_right_status(&self, ui: &mut Ui, _state: &Arc<RwLock<crate::AppState>>) {
        // Memory usage
        self.render_memory_usage(ui);

        ui.separator();

        // Current time
        self.render_current_time(ui);

        ui.separator();

        // System status
        self.render_system_status(ui);
    }

    /// Render memory usage information
    fn render_memory_usage(&self, ui: &mut Ui) {
        #[cfg(not(target_os = "unknown"))]
        {
            match sysinfo::System::new_with_specifics(sysinfo::RefreshKind::new().with_memory()) {
                Ok(mut system) => {
                    system.refresh_memory();

                    let total_memory = system.total_memory();
                    let used_memory = system.used_memory();

                    // Format memory usage
                    let used_mb = used_memory / (1024 * 1024);
                    let total_mb = total_memory / (1024 * 1024);
                    let usage_percent = (used_memory as f64 / total_memory as f64 * 100.0) as u8;

                    // Choose color based on usage
                    let usage_color = if usage_percent > 80 {
                        egui::Color32::RED
                    } else if usage_percent > 60 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::GREEN
                    };

                    let memory_text = format!("🧠 {}MB / {}MB", used_mb, total_mb);
                    ui.label(RichText::new(memory_text).size(10.0).color(usage_color));
                }
                Err(_) => {
                    ui.label(RichText::new("🧠 Memory: N/A").size(10.0).weak());
                }
            }
        }

        #[cfg(target_os = "unknown")]
        {
            ui.label(RichText::new("🧠 Memory Info").size(10.0).weak());
        }
    }

    /// Render current time
    fn render_current_time(&self, ui: &mut Ui) {
        let time_str = self.last_update.format("%H:%M:%S").to_string();
        let date_str = self.last_update.format("%Y-%m-%d").to_string();

        ui.label(RichText::new(format!("📅 {}", date_str)).size(10.0).weak());
        ui.label(RichText::new(format!("🕒 {}", time_str)).size(10.0));
    }

    /// Render system status
    fn render_system_status(&self, ui: &mut Ui) {
        // System status indicator
        let status_color = egui::Color32::GREEN;
        let status_text = "● Running";

        ui.label(RichText::new(status_text).size(10.0).color(status_color));
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_creation() {
        let status_bar = StatusBar::new();

        // Should not panic
        assert!(status_bar.last_update <= chrono::Local::now());
    }

    #[test]
    fn test_status_bar_update() {
        let mut status_bar = StatusBar::new();
        let old_time = status_bar.last_update;

        // Simulate a render call
        let ctx = egui::Context::default();
        let ui = ctx.new_row_ui();
        let state = std::sync::Arc::new(std::sync::RwLock::new(
            crate::AppState::new().unwrap()
        ));

        status_bar.render(&mut ui, &ctx, &state);

        // Should update last_update time
        assert!(status_bar.last_update >= old_time);
    }
}