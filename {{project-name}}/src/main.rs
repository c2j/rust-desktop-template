use test_template::App;
use eframe::{egui, NativeOptions};
use tracing::{info, error};

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting test-template v{}", env!("CARGO_PKG_VERSION"));

    // Configure native options
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("test-template")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // Run the application
    match eframe::run_native(
        "test-template",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    ) {
        Ok(_) => info!("test-template closed successfully"),
        Err(e) => error!("Failed to run test-template: {}", e),
    }
}