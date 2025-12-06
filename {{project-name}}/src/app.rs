//! Main application logic and state management

use crate::{
    config::Config,
    error::Result,
    modules::{ApplicationModule, ModuleId, ModuleRegistry},
    ui::{Navigation, Theme, Workspace},
};
use egui::{Context, Ui};
use std::sync::{Arc, RwLock};
use tracing::{debug, info};

/// Application state shared across components
#[derive(Debug)]
pub struct AppState {
    /// Current active module
    pub active_module: Option<ModuleId>,
    /// Module registry
    pub modules: ModuleRegistry,
    /// Application configuration
    pub config: Config,
    /// UI theme
    pub theme: Theme,
}

impl AppState {
    /// Create new application state
    pub fn new() -> Result<Self> {
        info!("Initializing application state");

        // Load configuration
        let config = Config::load()?;

        // Initialize theme
        let theme = Theme::load("dark")?;

        // Create module registry
        let modules = ModuleRegistry::new();

        Ok(Self {
            active_module: None,
            modules,
            config,
            theme,
        })
    }

    /// Get the active module
    pub fn active_module(&self) -> Option<&dyn ApplicationModule> {
        self.active_module
            .and_then(|id| self.modules.get(id))
    }

    /// Set the active module
    pub fn set_active_module(&mut self, module_id: ModuleId) {
        debug!("Setting active module: {:?}", module_id);
        self.active_module = Some(module_id);
    }
}

/// Main application struct
pub struct App {
    /// Shared application state
    state: Arc<RwLock<AppState>>,
    /// Navigation component
    navigation: Navigation,
    /// Workspace component
    workspace: Workspace,
}

impl App {
    /// Create a new application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        debug!("Creating new application instance");

        // Initialize logging
        crate::logger::init();

        // Create application state
        let state = match AppState::new() {
            Ok(state) => Arc::new(RwLock::new(state)),
            Err(e) => {
                eprintln!("Failed to initialize application state: {}", e);
                std::process::exit(1);
            }
        };

        // Configure egui context
        configure_egui(&cc.egui_ctx);

        // Create UI components
        let navigation = Navigation::new(state.clone());
        let workspace = Workspace::new(state.clone());

        Self {
            state,
            navigation,
            workspace,
        }
    }

    /// Update application state
    fn update_state(&mut self, ctx: &Context) {
        if let Ok(mut state) = self.state.write() {
            // Update active module if navigation changed
            if let Some(new_module) = self.navigation.selected_module() {
                if state.active_module != Some(new_module) {
                    state.set_active_module(new_module);
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Update application state
        self.update_state(ctx);

        // Apply theme
        if let Ok(state) = self.state.read() {
            state.theme.apply(ctx);
        }

        // Main UI layout
        egui::CentralPanel::default().show(ctx, |ui| {
            // Top panel for global actions
            egui::TopBottomPanel::top("top_panel").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("rust-desktop-app");
                    ui.separator();

                    if ui.button("🎨").clicked() {
                        // Theme switcher
                        if let Ok(mut state) = self.state.write() {
                            // TODO: Implement theme switching
                        }
                    }
                });
            });

            // Left sidebar for navigation
            egui::SidePanel::left("navigation_panel")
                .resizable(true)
                .default_width(48.0)
                .min_width(40.0)
                .max_width(200.0)
                .show_inside(ui, |ui| {
                    self.navigation.render(ui, ctx);
                });

            // Main content area
            egui::CentralPanel::default()
                .frame(egui::Frame::default().inner_margin(8.0))
                .show_inside(ui, |ui| {
                    self.workspace.render(ui, ctx);
                });

            // Bottom status bar
            egui::TopBottomPanel::bottom("status_bar")
                .resizable(false)
                .min_height(24.0)
                .max_height(32.0)
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("rust-desktop-app v{}", env!("CARGO_PKG_VERSION")));

                        if let Ok(state) = self.state.read() {
                            if let Some(module_id) = state.active_module {
                                ui.separator();
                                ui.label(format!("Active: {:?}", module_id));
                            }
                        }

                        ui.separator();
                        ui.label(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    });
                });
        });
    }
}

/// Configure egui with application-specific settings
fn configure_egui(ctx: &Context) {
    let mut fonts = egui::FontDefinitions::default();

    // TODO: Add emoji font support when available
    // fonts.font_data.insert("emoji".to_string(), egui::FontData::default());
    // Note: Emoji font file commented out to avoid missing file error

    // Configure font families
    fonts.families.get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "emoji".to_string());

    ctx.set_fonts(fonts);

    // Configure style
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(6.0, 4.0);
    ctx.set_style(style);
}