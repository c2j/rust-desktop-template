//! Workspace component for displaying the active module content

use crate::{
    error::Result,
    modules::{ApplicationModule, ModuleId},
    ui::Theme,
};
use egui::{Context, Frame, Ui};
use std::sync::{Arc, RwLock};
use tracing::{debug, warn};

/// Workspace component that displays the active module's content
#[derive(Debug)]
pub struct Workspace {
    /// Shared application state
    state: Arc<RwLock<crate::AppState>>,
    /// Previous active module for change detection
    previous_module: Option<ModuleId>,
}

impl Workspace {
    /// Create a new workspace component
    pub fn new(state: Arc<RwLock<crate::AppState>>) -> Self {
        Self {
            state,
            previous_module: None,
        }
    }

    /// Render the workspace UI
    pub fn render(&mut self, ui: &mut Ui, ctx: &Context) {
        let state = match self.state.read() {
            Ok(state) => state,
            Err(e) => {
                debug!("Failed to read application state: {}", e);
                self.render_empty_workspace(ui, &Theme::dark());
                return;
            }
        };

        let theme = &state.theme;

        // Check if active module changed
        let current_module = state.active_module;
        if self.previous_module != current_module {
            self.handle_module_change(current_module);
            self.previous_module = current_module;
        }

        // Get the active module and render it
        if let Some(active_module) = current_module {
            if let Some(module) = state.modules.get(active_module) {
                self.render_module_content(ui, ctx, module, theme);
            } else {
                self.render_module_not_found(ui, theme, active_module);
            }
        } else {
            self.render_no_active_module(ui, theme);
        }
    }

    /// Handle module change
    fn handle_module_change(&self, new_module: Option<ModuleId>) {
        if let Some(module_id) = new_module {
            debug!("Switching to module: {:?}", module_id);

            // Activate the new module
            if let Ok(state) = self.state.read() {
                if let Some(module) = state.modules.get(module_id) {
                    // Call module's on_activate method
                    // Note: We need a mutable reference here, so we'd need to handle this differently
                    // in a real implementation
                }
            }
        }
    }

    /// Render the content of the active module
    fn render_module_content(
        &self,
        ui: &mut Ui,
        ctx: &Context,
        module: &dyn ApplicationModule,
        theme: &Theme,
    ) {
        // Render module header
        self.render_module_header(ui, module, theme);

        // Add some spacing
        ui.add_space(theme.spacing.medium);

        // Render the module's content
        let mut content_frame = egui::Frame::default()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(1.0, theme.colors.border))
            .rounding(theme.widgets.card_radius);

        content_frame.show(ui, |ui| {
            // Add padding inside the frame
            ui.add_space(theme.spacing.medium);

            // Create a clone of ui for the module to use
            let mut module_ui = ui.new_child();

            // Render the module's content
            // Note: In a real implementation, we would need to handle the mutable reference issue
            // For now, we'll show a placeholder

            self.render_module_placeholder(&mut module_ui, module, theme);

            // Add padding at the bottom
            ui.add_space(theme.spacing.medium);
        });

        // Update module state
        // Note: This would require a mutable reference in a real implementation
        // module.update(ctx);
    }

    /// Render module header with name and actions
    fn render_module_header(&self, ui: &mut Ui, module: &dyn ApplicationModule, theme: &Theme) {
        ui.horizontal(|ui| {
            // Module icon and name
            let header_text = format!("{} {}", module.icon(), module.name());
            ui.heading(egui::RichText::new(header_text).size(18.0).color(theme.colors.text));

            // Add spacer
            ui.add_space(theme.spacing.xlarge);

            // Module-specific actions
            self.render_module_actions(ui, module, theme);
        });

        // Add description if available
        let description = module.description();
        if !description.is_empty() {
            ui.add_space(theme.spacing.small);
            ui.label(egui::RichText::new(description).size(12.0).color(theme.colors.text_secondary));
        }
    }

    /// Render module-specific action buttons
    fn render_module_actions(&self, ui: &mut Ui, module: &dyn ApplicationModule, theme: &Theme) {
        // Module-specific actions would go here
        // For now, we'll add a refresh button
        if ui.button("🔄").on_hover_text("Refresh Module").clicked() {
            // TODO: Implement module refresh
            debug!("Refresh button clicked for module: {}", module.name());
        }

        // Settings button for module configuration
        if ui.button("⚙️").on_hover_text("Module Settings").clicked() {
            // TODO: Implement module settings
            debug!("Settings button clicked for module: {}", module.name());
        }
    }

    /// Render placeholder content for a module
    fn render_module_placeholder(&self, ui: &mut Ui, module: &dyn ApplicationModule, theme: &Theme) {
        ui.vertical_centered(|ui| {
            ui.add_space(theme.spacing.xlarge);

            // Large icon
            let icon_text = egui::RichText::new(module.icon()).size(64.0);
            ui.label(icon_text);

            ui.add_space(theme.spacing.medium);

            // Module name
            let name_text = egui::RichText::new(module.name()).size(24.0).color(theme.colors.text);
            ui.label(name_text);

            ui.add_space(theme.spacing.small);

            // Module description
            let description = module.description();
            if !description.is_empty() {
                let desc_text = egui::RichText::new(description).size(14.0).color(theme.colors.text_secondary);
                ui.label(desc_text);
            }

            ui.add_space(theme.spacing.large);

            // Placeholder content area
            ui.label(egui::RichText::new("Module content will appear here").size(16.0).color(theme.colors.text_secondary));

            ui.add_space(theme.spacing.medium);

            // Action buttons
            ui.horizontal(|ui| {
                if ui.button("Configure Module").clicked() {
                    // TODO: Implement module configuration
                    debug!("Configure module clicked: {}", module.name());
                }

                if ui.button("View Documentation").clicked() {
                    // TODO: Open module documentation
                    debug!("View documentation clicked: {}", module.name());
                }
            });

            ui.add_space(theme.spacing.xlarge);
        });
    }

    /// Render when no module is active
    fn render_no_active_module(&self, ui: &mut Ui, theme: &Theme) {
        self.render_empty_workspace(ui, theme);
    }

    /// Render when active module is not found
    fn render_module_not_found(&self, ui: &mut Ui, theme: &Theme, module_id: ModuleId) {
        ui.vertical_centered(|ui| {
            ui.add_space(theme.spacing.xlarge);

            // Error icon
            let error_text = egui::RichText::new("❌").size(48.0);
            ui.label(error_text);

            ui.add_space(theme.spacing.medium);

            // Error message
            let error_msg = format!("Module not found: {:?}", module_id);
            let msg_text = egui::RichText::new(error_msg).size(16.0).color(theme.colors.error);
            ui.label(msg_text);

            ui.add_space(theme.spacing.medium);

            // Suggestion
            let suggestion_text = egui::RichText::new("Please select a valid module from the navigation sidebar.")
                .size(14.0)
                .color(theme.colors.text_secondary);
            ui.label(suggestion_text);

            ui.add_space(theme.spacing.large);

            // Action button
            if ui.button("Go to Modules").clicked() {
                // TODO: Focus navigation sidebar
                debug!("Go to modules clicked");
            }
        });
    }

    /// Render empty workspace
    fn render_empty_workspace(&self, ui: &mut Ui, theme: &Theme) {
        ui.vertical_centered(|ui| {
            ui.add_space(theme.spacing.xlarge);

            // Welcome icon
            let welcome_text = egui::RichText::new("👋").size(64.0);
            ui.label(welcome_text);

            ui.add_space(theme.spacing.medium);

            // Welcome message
            let welcome_msg = "Welcome to {{project-name}}!";
            let msg_text = egui::RichText::new(welcome_msg).size(24.0).color(theme.colors.text);
            ui.label(msg_text);

            ui.add_space(theme.spacing.small);

            // Subtitle
            let subtitle = "Select a module from the navigation sidebar to get started.";
            let subtitle_text = egui::RichText::new(subtitle).size(16.0).color(theme.colors.text_secondary);
            ui.label(subtitle_text);

            ui.add_space(theme.spacing.large);

            // Quick actions
            ui.horizontal(|ui| {
                if ui.button("📁 Open File Browser").clicked() {
                    // TODO: Select file browser module
                    debug!("Open file browser clicked");
                }

                if ui.button("📝 Open Text Editor").clicked() {
                    // TODO: Select text editor module
                    debug!("Open text editor clicked");
                }

                if ui.button("📊 System Monitor").clicked() {
                    // TODO: Select system monitor module
                    debug!("Open system monitor clicked");
                }
            });

            ui.add_space(theme.spacing.xlarge);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::{ApplicationModule, ModuleId};

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

        fn description(&self) -> &str {
            "Test module for workspace testing"
        }

        fn render(&mut self, _ui: &mut egui::Ui, _ctx: &egui::Context) {
            // Test implementation
        }
    }

    #[test]
    fn test_workspace_creation() {
        let state = std::sync::Arc::new(std::sync::RwLock::new(
            crate::AppState::new().unwrap()
        ));
        let workspace = Workspace::new(state);

        // Should not panic
        assert_eq!(workspace.previous_module, None);
    }

    #[test]
    fn test_module_change_detection() {
        let state = std::sync::Arc::new(std::sync::RwLock::new(
            crate::AppState::new().unwrap()
        ));
        let workspace = Workspace::new(state);

        // Initially no module selected
        assert_eq!(workspace.previous_module, None);

        // After module change, should update
        // Note: This test would need to simulate the render call to test the change detection
    }
}