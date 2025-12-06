//! System monitor module implementation

use crate::{
    error::Result,
    modules::{ApplicationModule, ModuleId},
    ui::Theme,
};
use egui::{Color32, Context, Ui};
use std::time::{Duration, Instant};
use tracing::{debug, error, info};

/// System monitor module
#[derive(Debug)]
pub struct SystemMonitorModule {
    id: ModuleId,
    last_update: Instant,
    update_interval: Duration,
    system_info: SystemInfo,
    graphs: Vec<GraphData>,
}

#[derive(Debug, Clone)]
struct SystemInfo {
    cpu_usage: f32,
    memory_usage: MemoryInfo,
    disk_usage: Vec<DiskInfo>,
    network_info: NetworkInfo,
    processes: Vec<ProcessInfo>,
}

#[derive(Debug, Clone)]
struct MemoryInfo {
    total: u64,
    used: u64,
    available: u64,
    usage_percent: f32,
}

#[derive(Debug, Clone)]
struct DiskInfo {
    name: String,
    mount_point: String,
    total: u64,
    used: u64,
    available: u64,
    usage_percent: f32,
}

#[derive(Debug, Clone)]
struct NetworkInfo {
    bytes_sent: u64,
    bytes_received: u64,
    active_connections: u32,
}

#[derive(Debug, Clone)]
struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_percent: f32,
    memory_usage: u64,
}

#[derive(Debug, Clone)]
struct GraphData {
    values: Vec<f32>,
    max_points: usize,
    label: String,
    color: Color32,
}

impl SystemMonitorModule {
    /// Create a new system monitor module
    pub fn new() -> Self {
        Self {
            id: ModuleId::SystemMonitor,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(1),
            system_info: SystemInfo {
                cpu_usage: 0.0,
                memory_usage: MemoryInfo {
                    total: 0,
                    used: 0,
                    available: 0,
                    usage_percent: 0.0,
                },
                disk_usage: Vec::new(),
                network_info: NetworkInfo {
                    bytes_sent: 0,
                    bytes_received: 0,
                    active_connections: 0,
                },
                processes: Vec::new(),
            },
            graphs: vec![
                GraphData {
                    values: Vec::new(),
                    max_points: 60,
                    label: "CPU".to_string(),
                    color: Color32::from_rgb(255, 165, 0),
                },
                GraphData {
                    values: Vec::new(),
                    max_points: 60,
                    label: "Memory".to_string(),
                    color: Color32::from_rgb(0, 128, 255),
                },
            ],
        }
    }

    /// Update system information
    fn update_system_info(&mut self) -> Result<()> {
        if self.last_update.elapsed() < self.update_interval {
            return Ok(());
        }

        // Update CPU usage
        self.system_info.cpu_usage = self.get_cpu_usage()?;

        // Update memory usage
        self.system_info.memory_usage = self.get_memory_usage()?;

        // Update disk usage
        self.system_info.disk_usage = self.get_disk_usage()?;

        // Update network info
        self.system_info.network_info = self.get_network_info()?;

        // Update top processes
        self.system_info.processes = self.get_top_processes()?;

        // Update graphs
        self.update_graphs();

        self.last_update = Instant::now();
        Ok(())
    }

    /// Get CPU usage percentage
    fn get_cpu_usage(&self) -> Result<f32> {
        #[cfg(not(target_os = "unknown"))]
        {
            use std::sync::Mutex;

            static CPU_USAGE: Mutex<Option<f32>> = Mutex::new(None);

            let mut cpu_usage = CPU_USAGE.lock().unwrap();
            if let Some(usage) = *cpu_usage {
                return Ok(usage);
            }

            // Simulate CPU usage calculation
            // In a real implementation, you would use sysinfo or similar crate
            let usage = fastrand::f32() * 100.0;
            *cpu_usage = Some(usage);
            Ok(usage)
        }

        #[cfg(target_os = "unknown")]
        {
            Ok(fastrand::f32() * 100.0)
        }
    }

    /// Get memory usage information
    fn get_memory_usage(&self) -> Result<MemoryInfo> {
        #[cfg(not(target_os = "unknown"))]
        {
            use std::sync::Mutex;

            static MEMORY_INFO: Mutex<Option<MemoryInfo>> = Mutex::new(None);

            let mut mem_info = MEMORY_INFO.lock().unwrap();
            if let Some(info) = *mem_info {
                return Ok(info);
            }

            // Simulate memory usage calculation
            // In a real implementation, you would use sysinfo or similar crate
            let total = 8 * 1024 * 1024 * 1024; // 8GB
            let used = (total as f32 * fastrand::f32()) as u64;
            let available = total - used;

            let info = MemoryInfo {
                total,
                used,
                available,
                usage_percent: (used as f32 / total as f32) * 100.0,
            };

            *mem_info = Some(info.clone());
            Ok(info)
        }

        #[cfg(target_os = "unknown")]
        {
            let total = 8 * 1024 * 1024 * 1024; // 8GB
            let used = (total as f32 * fastrand::f32()) as u64;
            let available = total - used;

            Ok(MemoryInfo {
                total,
                used,
                available,
                usage_percent: (used as f32 / total as f32) * 100.0,
            })
        }
    }

    /// Get disk usage information
    fn get_disk_usage(&self) -> Result<Vec<DiskInfo>> {
        #[cfg(not(target_os = "unknown"))]
        {
            // Simulate disk usage calculation
            // In a real implementation, you would query actual disk partitions
            Ok(vec![
                DiskInfo {
                    name: "C:".to_string(),
                    mount_point: "/".to_string(),
                    total: 500 * 1024 * 1024 * 1024, // 500GB
                    used: 250 * 1024 * 1024 * 1024, // 250GB
                    available: 250 * 1024 * 1024 * 1024, // 250GB
                    usage_percent: 50.0,
                },
                DiskInfo {
                    name: "D:".to_string(),
                    mount_point: "/home".to_string(),
                    total: 1000 * 1024 * 1024 * 1024, // 1TB
                    used: 750 * 1024 * 1024 * 1024, // 750GB
                    available: 250 * 1024 * 1024 * 1024, // 250GB
                    usage_percent: 75.0,
                },
            ])
        }

        #[cfg(target_os = "unknown")]
        {
            Ok(vec![])
        }
    }

    /// Get network information
    fn get_network_info(&self) -> Result<NetworkInfo> {
        Ok(NetworkInfo {
            bytes_sent: fastrand::u64(0..1_000_000_000),
            bytes_received: fastrand::u64(0..5_000_000_000),
            active_connections: fastrand::u32(0..100),
        })
    }

    /// Get top processes
    fn get_top_processes(&self) -> Result<Vec<ProcessInfo>> {
        // Simulate top processes
        let mut processes = Vec::new();

        for i in 0..10 {
            processes.push(ProcessInfo {
                pid: 1000 + i,
                name: format!("process_{}.exe", i),
                cpu_percent: fastrand::f32() * 50.0,
                memory_usage: fastrand::u64(0..1_000_000_000),
            });
        }

        // Sort by CPU usage
        processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap());

        Ok(processes)
    }

    /// Update graph data
    fn update_graphs(&mut self) {
        // Update CPU graph
        if let Some(cpu_graph) = self.graphs.get_mut(0) {
            cpu_graph.values.push(self.system_info.cpu_usage);
            if cpu_graph.values.len() > cpu_graph.max_points {
                cpu_graph.values.remove(0);
            }
        }

        // Update memory graph
        if let Some(mem_graph) = self.graphs.get_mut(1) {
            mem_graph.values.push(self.system_info.memory_usage.usage_percent);
            if mem_graph.values.len() > mem_graph.max_points {
                mem_graph.values.remove(0);
            }
        }
    }

    /// Format bytes to human readable format
    fn format_bytes(&self, bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if bytes >= TB {
            format!("{:.1} TB", bytes as f64 / TB as f64)
        } else if bytes >= GB {
            format!("{:.1} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.1} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }

    /// Render system overview panel
    fn render_overview(&self, ui: &mut Ui, theme: &Theme) {
        ui.heading("System Overview");
        ui.add_space(theme.spacing.medium);

        // CPU and Memory
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("CPU Usage");
                ui.add_space(theme.spacing.small);
                let cpu_color = if self.system_info.cpu_usage > 80.0 {
                    theme.colors.error
                } else if self.system_info.cpu_usage > 60.0 {
                    theme.colors.warning
                } else {
                    theme.colors.success
                };
                ui.label(egui::RichText::new(format!("{:.1}%", self.system_info.cpu_usage))
                    .size(24.0)
                    .color(cpu_color)
                    .strong());
            });

            ui.add_space(theme.spacing.large);

            ui.vertical(|ui| {
                ui.label("Memory Usage");
                ui.add_space(theme.spacing.small);
                let mem_color = if self.system_info.memory_usage.usage_percent > 80.0 {
                    theme.colors.error
                } else if self.system_info.memory_usage.usage_percent > 60.0 {
                    theme.colors.warning
                } else {
                    theme.colors.success
                };
                ui.label(egui::RichText::new(format!("{:.1}%", self.system_info.memory_usage.usage_percent))
                    .size(24.0)
                    .color(mem_color)
                    .strong());
                ui.label(egui::RichText::new(format!("of {}", self.format_bytes(self.system_info.memory_usage.total)))
                    .size(12.0)
                    .weak());
            });
        });

        ui.add_space(theme.spacing.large);

        // Memory breakdown
        ui.heading("Memory");
        ui.add_space(theme.spacing.small);

        // Memory usage bar
        let memory_bar = egui::ProgressBar::new(self.system_info.memory_usage.usage_percent / 100.0)
            .fill(theme.colors.surface)
            .show_percentage();
        ui.add(memory_bar);

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("Used: {}", self.format_bytes(self.system_info.memory_usage.used)))
                .size(12.0));
            ui.add_space(theme.spacing.small);
            ui.label(egui::RichText::new(format!("Available: {}", self.format_bytes(self.system_info.memory_usage.available)))
                .size(12.0)
                .weak());
        });
    }

    /// Render graphs
    fn render_graphs(&mut self, ui: &mut Ui, theme: &Theme) {
        ui.heading("Performance Graphs");
        ui.add_space(theme.spacing.medium);

        for graph in &self.graphs {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&graph.label).size(14.0).strong());
                ui.add_space(theme.spacing.medium);
                if let Some(&latest) = graph.values.last() {
                    ui.label(egui::RichText::new(format!("{:.1}%", latest))
                        .size(12.0)
                        .color(graph.color));
                }
            });

            ui.add_space(theme.spacing.small);

            // Simple line graph
            let available_width = ui.available_width();
            let graph_height = 60.0;

            // Draw background
            let painter = ui.painter_at(ui.next_frame(), ui.available_rect_before_wrap());
            let rect = egui::Rect::from_min_max(
                ui.cursor().min,
                ui.cursor().min + egui::vec2(available_width, graph_height)
            );

            painter.rect_filled(rect, 0.0, theme.colors.surface);
            painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, theme.colors.border));

            // Draw graph line
            if graph.values.len() > 1 {
                let points: Vec<egui::Pos2> = graph.values
                    .iter()
                    .enumerate()
                    .map(|(i, &value)| {
                        let x = rect.min.x + (i as f32 / (graph.max_points - 1) as f32) * rect.width();
                        let y = rect.max.y - (value / 100.0) * rect.height();
                        egui::pos2(x, y)
                    })
                    .collect();

                painter.line_segment(points, egui::Stroke::new(2.0, graph.color));
            }

            ui.add_space(graph_height + theme.spacing.medium);
        }
    }

    /// Render disk usage
    fn render_disk_usage(&self, ui: &mut Ui, theme: &Theme) {
        ui.heading("Disk Usage");
        ui.add_space(theme.spacing.medium);

        for disk in &self.system_info.disk_usage {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&disk.name).size(14.0).strong());
                ui.add_space(theme.spacing.small);
                ui.label(egui::RichText::new(&disk.mount_point).size(12.0).weak());
            });

            ui.add_space(theme.spacing.small);

            // Disk usage bar
            let disk_bar = egui::ProgressBar::new(disk.usage_percent / 100.0)
                .fill(theme.colors.surface)
                .show_percentage();
            ui.add(disk_bar);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("Used: {}", self.format_bytes(disk.used)))
                    .size(12.0));
                ui.add_space(theme.spacing.small);
                ui.label(egui::RichText::new(format!("Available: {}", self.format_bytes(disk.available)))
                    .size(12.0)
                    .weak());
            });

            ui.add_space(theme.spacing.medium);
        }
    }

    /// Render network information
    fn render_network(&self, ui: &mut Ui, theme: &Theme) {
        ui.heading("Network");
        ui.add_space(theme.spacing.medium);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Bytes Sent:");
                ui.add_space(theme.spacing.small);
                ui.label(egui::RichText::new(self.format_bytes(self.system_info.network_info.bytes_sent))
                    .size(16.0)
                    .color(theme.colors.primary));
            });

            ui.add_space(theme.spacing.large);

            ui.vertical(|ui| {
                ui.label("Bytes Received:");
                ui.add_space(theme.spacing.small);
                ui.label(egui::RichText::new(self.format_bytes(self.system_info.network_info.bytes_received))
                    .size(16.0)
                    .color(theme.colors.primary));
            });

            ui.add_space(theme.spacing.large);

            ui.vertical(|ui| {
                ui.label("Active Connections:");
                ui.add_space(theme.spacing.small);
                ui.label(egui::RichText::new(format!("{}", self.system_info.network_info.active_connections))
                    .size(16.0)
                    .color(theme.colors.primary));
            });
        });
    }

    /// Render top processes
    fn render_processes(&self, ui: &mut Ui, theme: &Theme) {
        ui.heading("Top Processes");
        ui.add_space(theme.spacing.medium);

        // Table header
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("PID").size(12.0).strong());
            ui.add_space(theme.spacing.large);
            ui.label(egui::RichText::new("Name").size(12.0).strong());
            ui.add_space(theme.spacing.large);
            ui.label(egui::RichText::new("CPU").size(12.0).strong());
            ui.add_space(theme.spacing.medium);
            ui.label(egui::RichText::new("Memory").size(12.0).strong());
        });

        ui.add_space(theme.spacing.small);

        // Process rows
        for process in &self.system_info.processes {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("{}", process.pid)).size(11.0));
                ui.add_space(theme.spacing.large);
                ui.label(egui::RichText::new(&process.name).size(11.0));
                ui.add_space(theme.spacing.large);
                ui.label(egui::RichText::new(format!("{:.1}%", process.cpu_percent)).size(11.0));
                ui.add_space(theme.spacing.medium);
                ui.label(egui::RichText::new(self.format_bytes(process.memory_usage)).size(11.0));
            });
        }
    }
}

impl ApplicationModule for SystemMonitorModule {
    fn id(&self) -> ModuleId {
        self.id
    }

    fn name(&self) -> &str {
        "System Monitor"
    }

    fn icon(&self) -> &str {
        "📊"
    }

    fn description(&self) -> &str {
        "Monitor system resources including CPU, memory, disk, and network usage"
    }

    fn render(&mut self, ui: &mut Ui, ctx: &Context) {
        // Update system information
        if let Err(e) = self.update_system_info() {
            ui.colored_label(egui::Color32::RED, format!("Error updating system info: {}", e));
            return;
        }

        // Get theme for styling
        let theme = &ctx.data::<Theme>().unwrap_or(&Theme::dark());

        // Render different sections
        self.render_overview(ui, theme);
        ui.add_space(theme.spacing.large);

        self.render_graphs(ui, theme);
        ui.add_space(theme.spacing.large);

        self.render_disk_usage(ui, theme);
        ui.add_space(theme.spacing.large);

        self.render_network(ui, theme);
        ui.add_space(theme.spacing.large);

        self.render_processes(ui, theme);
    }

    fn update(&mut self, ctx: &Context) {
        // Request repaint for real-time updates
        ctx.request_repaint();
    }

    fn on_activate(&mut self) {
        info!("System monitor module activated");
        self.last_update = Instant::now();
    }

    fn on_deactivate(&mut self) {
        info!("System monitor module deactivated");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_monitor_creation() {
        let monitor = SystemMonitorModule::new();
        assert_eq!(monitor.id(), ModuleId::SystemMonitor);
        assert_eq!(monitor.name(), "System Monitor");
        assert_eq!(monitor.icon(), "📊");
    }

    #[test]
    fn test_bytes_formatting() {
        let monitor = SystemMonitorModule::new();

        assert_eq!(monitor.format_bytes(512), "512 B");
        assert_eq!(monitor.format_bytes(1536), "1.5 KB");
        assert_eq!(monitor.format_bytes(2097152), "2.0 MB");
        assert_eq!(monitor.format_bytes(1073741824), "1.0 GB");
    }

    #[test]
    fn test_graph_updates() {
        let mut monitor = SystemMonitorModule::new();

        // Initially graphs should be empty
        assert!(monitor.graphs[0].values.is_empty());
        assert!(monitor.graphs[1].values.is_empty());

        // Update system info should populate graphs
        monitor.update_graphs();

        // Graphs should have values after update
        assert!(!monitor.graphs[0].values.is_empty());
        assert!(!monitor.graphs[1].values.is_empty());
    }
}