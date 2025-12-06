//! System monitor module for displaying system information

use crate::modules::ApplicationModule;
use crate::modules::ModuleId;
use egui::{Context, Ui};

/// System monitor module
pub struct SystemMonitor {
    cpu_usage: f32,
    memory_usage: u64,
    uptime: std::time::Duration,
    last_update: std::time::Instant,
}

impl SystemMonitor {
    /// Create a new system monitor module
    pub fn new() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            uptime: std::time::Duration::from_secs(0),
            last_update: std::time::Instant::now(),
        }
    }

    /// Update system information
    fn update_system_info(&mut self) {
        // Simple mock system info - in real implementation, use actual system APIs
        if self.last_update.elapsed().as_secs() >= 1 {
            // Mock CPU usage with random value
            self.cpu_usage = fastrand::f32() * 100.0;

            // Mock memory usage
            self.memory_usage = fastrand::u64(0..8_000_000_000); // 0-8GB

            self.last_update = std::time::Instant::now();
        }
    }

    /// Format bytes to human readable format
    fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

impl ApplicationModule for SystemMonitor {
    fn id(&self) -> ModuleId {
        ModuleId::SystemMonitor
    }

    fn name(&self) -> &str {
        "System Monitor"
    }

    fn icon(&self) -> &str {
        "📊"
    }

    fn render(&mut self, ui: &mut Ui, _ctx: &Context) {
        ui.heading("📊 System Monitor");

        ui.separator();

        // CPU Usage
        ui.horizontal(|ui| {
            ui.label("CPU Usage:");
            ui.spinner();
            ui.label(format!("{:.1}%", self.cpu_usage));
        });

        // CPU Progress bar
        ui.add(egui::ProgressBar::new(self.cpu_usage / 100.0)
            .show_percentage()
            .desired_width(f32::INFINITY));

        ui.separator();

        // Memory Usage
        ui.horizontal(|ui| {
            ui.label("Memory Usage:");
            ui.label(self.format_bytes(self.memory_usage));
        });

        // Memory Progress bar (assuming 16GB total)
        let memory_percent = (self.memory_usage as f64 / 16_000_000_000.0) as f32;
        ui.add(egui::ProgressBar::new(memory_percent)
            .show_percentage()
            .desired_width(f32::INFINITY));

        ui.separator();

        // System Information
        ui.heading("System Information");
        ui.horizontal(|ui| {
            ui.label("OS:");
            ui.label(std::env::consts::OS);
        });

        ui.horizontal(|ui| {
            ui.label("Arch:");
            ui.label(std::env::consts::ARCH);
        });

        ui.horizontal(|ui| {
            ui.label("Family:");
            ui.label(std::env::consts::FAMILY);
        });

        ui.separator();

        // Process Information
        ui.heading("Process Information");
        ui.horizontal(|ui| {
            ui.label("PID:");
            ui.label(format!("{}", std::process::id()));
        });
    }

    fn update(&mut self, _ctx: &Context) {
        self.update_system_info();
    }

    fn on_activate(&mut self) {
        tracing::info!("System monitor activated");
    }

    fn on_deactivate(&mut self) {
        tracing::info!("System monitor deactivated");
    }
}