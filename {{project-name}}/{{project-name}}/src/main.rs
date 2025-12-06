use rust-desktop-app::App;
use eframe::{egui, NativeOptions};
use tracing::{info, error};

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting rust-desktop-app v{}", env!("CARGO_PKG_VERSION"));

    // Configure native options
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("rust-desktop-app")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // Run the application
    match eframe::run_native(
        "rust-desktop-app",
        options,
        Box::new(|cc| Box::new(App::new(cc))),
    ) {
        Ok(_) => info!("rust-desktop-app closed successfully"),
        Err(e) => error!("Failed to run rust-desktop-app: {}", e),
    }
}