//! Navigation component for the application

use crate::{
    app::AppState,
    error::Result,
    modules::{ApplicationModule, ModuleId},
    ui::Theme,
};
use egui::{Context, Response, ScrollArea, Ui, Vec2};
use std::sync::{Arc, RwLock};
use tracing::debug;

/// Navigation component that handles module switching
#[derive(Debug)]
pub struct Navigation {
    /// Shared application state
    state: Arc<RwLock<AppState>>,
    /// Currently selected module
    selected_module: Option<ModuleId>,
}

impl Navigation {
    /// Create a new navigation component
    pub fn new(state: Arc<RwLock<AppState>>) -> Self {
        Self {
            state,
            selected_module: None,
        }
    }

    /// Get the currently selected module
    pub fn selected_module(&self) -> Option<ModuleId> {
        self.selected_module
    }

    /// Set the selected module
    pub fn set_selected_module(&mut self, module_id: Option<ModuleId>) {
        self.selected_module = module_id;

        // Update application state
        if let Ok(mut state) = self.state.write() {
            state.active_module = module_id;
        }
    }

    /// Render the navigation UI
    pub fn render(&mut self, ui: &mut Ui, ctx: &Context) {
        let state = match self.state.read() {
            Ok(state) => state,
            Err(e) => {
                debug!("Failed to read application state: {}", e);
                return;
            }
        };

        // Calculate module item height
        let item_height = 40.0;

        // Get module IDs sorted by position
        let module_ids = state.modules.module_ids();

        // Create scrollable navigation area
        ScrollArea::vertical()
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Render navigation header
                    ui.add_space(8.0);
                    self.render_navigation_header(ui, &state.theme);
                    ui.add_space(8.0);

                    // Render navigation items
                    for module_id in module_ids {
                        let is_selected = self.selected_module == Some(module_id);

                        if let Some(module) = state.modules.get(module_id) {
                            self.render_navigation_item(ui, module_id, module, is_selected, item_height);
                        }
                    }

                    ui.add_space(16.0);

                    // Render navigation footer with status
                    self.render_navigation_footer(ui, &state.theme);
                });
            });
    }

    /// Render navigation header
    fn render_navigation_header(&self, ui: &mut Ui, theme: &Theme) {
        ui.centered_and_justified(|ui| {
            // Logo or application icon
            let button = egui::Button::new("🏠")
                .fill(theme.colors.surface)
                .stroke(egui::Stroke::NONE)
                .min_size(Vec2::new(32.0, 32.0));

            if ui.add(button).hovered() {
                ui.label(egui::RichText::new("Home").size(10.0).color(theme.colors.text_secondary));
            }
        });
        ui.add_space(8.0);
    }

    /// Render a single navigation item
    fn render_navigation_item(
        &self,
        ui: &mut Ui,
        module_id: ModuleId,
        module: &dyn crate::modules::ApplicationModule,
        is_selected: bool,
        item_height: f32,
    ) {
        let state = self.state.read().unwrap_or_else(|_| {
            crate::AppState::new().unwrap_or_else(|_| {
                // Fallback state
                crate::AppState::new().unwrap()
            })
        });

        let theme = &state.theme;

        // Create button for navigation item
        let button = egui::Button::new(egui::RichText::new(format!("{} {}", module.icon(), module.name()))
            .size(11.0)
            .color(if is_selected {
                theme.colors.text
            } else {
                theme.colors.text_secondary
            }))
        .fill(if is_selected {
            theme.colors.primary
        } else {
            egui::Color32::TRANSPARENT
        })
        .stroke(if is_selected {
            egui::Stroke::new(1.0, theme.colors.primary)
        } else {
            egui::Stroke::NONE
        })
        .min_size(Vec2::new(40.0, item_height))
        .rounding(egui::Rounding::same(4.0));

        let response = ui.add(button);

        // TODO: Show tooltip on hover
        if response.hovered() {
            // tooltip_text not available in egui 0.28
        }

        // Handle selection
        if response.clicked() {
            debug!("Navigation clicked: {:?}", module_id);
            self.set_selected_module(Some(module_id));

            // Activate the module
            if let Ok(mut state) = self.state.write() {
                if let Err(e) = state.modules.activate_module(module_id) {
                    debug!("Failed to activate module {:?}: {}", module_id, e);
                }
            }
        }
    }

    /// Render navigation footer
    fn render_navigation_footer(&mut self, ui: &mut Ui, theme: &Theme) {
        ui.add_space(8.0);

        // Theme switcher
        let theme_button = egui::Button::new("🎨")
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(1.0, theme.colors.border))
            .min_size(Vec2::new(32.0, 32.0))
            .rounding(egui::Rounding::same(4.0));

        let response = ui.add(theme_button);

        if response.hovered() {
            ui.label(egui::RichText::new("Theme").size(10.0).color(theme.colors.text_secondary));
        }

        if response.clicked() {
            // TODO: Implement theme switcher
            debug!("Theme switcher clicked");
        }

        ui.add_space(4.0);

        // Settings button
        let settings_button = egui::Button::new("⚙️")
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(1.0, theme.colors.border))
            .min_size(Vec2::new(32.0, 32.0))
            .rounding(egui::Rounding::same(4.0));

        let response = ui.add(settings_button);

        if response.hovered() {
            ui.label(egui::RichText::new("Settings").size(10.0).color(theme.colors.text_secondary));
        }

        if response.clicked() {
            debug!("Settings button clicked");
            // TODO: Implement set_selected_module
        }
    }

    /// Initialize example modules (for testing and demonstration)
    pub fn initialize_example_modules(&self) -> Result<()> {
        if let Ok(mut state) = self.state.write() {
            use crate::modules::create_example_modules;

            for module in create_example_modules() {
                state.modules.register(module);
            }

            // Select the first module as default
            if let Some(first_module) = state.modules.module_ids().first() {
                self.set_selected_module(Some(*first_module));
            }
        }

        Ok(())
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

        fn render(&mut self, _ui: &mut egui::Ui, _ctx: &egui::Context) {
            // Test implementation
        }
    }

    #[test]
    fn test_navigation_creation() {
        let state = std::sync::Arc::new(std::sync::RwLock::new(
            crate::AppState::new().unwrap()
        ));
        let navigation = Navigation::new(state);
        assert!(navigation.selected_module().is_none());
    }

    #[test]
    fn test_module_selection() {
        let state = std::sync::Arc::new(std::sync::RwLock::new(
            crate::AppState::new().unwrap()
        ));
        let mut navigation = Navigation::new(state);

        navigation.set_selected_module(Some(ModuleId::Home));
        assert_eq!(navigation.selected_module(), Some(ModuleId::Home));
    }
}