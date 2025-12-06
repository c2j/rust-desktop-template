# Quick Start Guide

Welcome to the **{{project-name}}** desktop application! This guide will help you get up and running quickly.

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ installed via [rustup](https://rustup.rs/)
- Basic understanding of Rust programming

### Create Your Project

```bash
cargo generate --git https://github.com/your-org/rust-desktop-template.git
cd <your-project-name>
cargo run
```

That's it! You now have a working desktop application with a VS Code-like interface.

## 📖 Application Interface

Your new application features:

### Navigation Sidebar (Left)
- 🏠 **Home** - Dashboard and overview
- 📁 **File Browser** - Browse and manage files
- 📝 **Text Editor** - Edit text files with syntax highlighting
- 📊 **System Monitor** - View system resource usage
- ⚙️ **Settings** - Configure application settings

### Main Content Area (Center)
- **Dynamic Content** - Content changes based on selected module
- **Module-Specific Features** - Each module provides its own functionality
- **Responsive Layout** - Adapts to window resizing

### Status Bar (Bottom)
- Application status and version
- Currently active module
- System information
- Current time

## 🎯 Common Tasks

### File Management

1. Click **📁 File Browser** in the navigation
2. Navigate through directories using the file tree
3. Double-click folders to enter them
4. Double-click files to open them
5. Use the toolbar buttons for additional actions

### Text Editing

1. Click **📝 Text Editor** in the navigation
2. Start typing to edit the welcome text
3. Use **📂 Open** to load a text file
4. Use **💾 Save** to save your changes
5. Use keyboard shortcuts for efficiency

### System Monitoring

1. Click **📊 System Monitor** in the navigation
2. View real-time CPU and memory usage
3. Monitor disk and network activity
4. View top resource-consuming processes

## 🎨 Customization

### Theme Customization

The template comes with multiple built-in themes:

- **Dark Theme** - Easy on the eyes (default)
- **Light Theme** - Bright and clean
- **High Contrast** - Maximum accessibility
- **Sepia Theme** - Warm, vintage look
- **Ocean Theme** - Cool, aquatic colors
- **Forest Theme** - Natural, earthy tones
- **Sunset Theme** - Warm, sunset colors

To change themes, edit `project.toml`:

```toml
[project]
default_theme = "ocean"  # or "light", "dark", etc.
```

### Custom Themes

Create custom themes by editing `theme.json`:

```json
{
  "name": "My Custom Theme",
  "type": "custom",
  "colors": {
    "primary": "#007ACC",
    "background": "#1E1E1E",
    "text": "#D4D4D4",
    "border": "#454545"
  }
}
```

### Module Customization

Add custom modules by implementing the `ApplicationModule` trait:

```rust
use crate::modules::ApplicationModule;
use egui::{Context, Ui};

pub struct MyCustomModule {
    // Your module state here
}

impl ApplicationModule for MyCustomModule {
    fn id(&self) -> crate::modules::ModuleId {
        crate::modules::ModuleId::Custom(1)
    }

    fn name(&self) -> &str {
        "My Custom Module"
    }

    fn icon(&self) -> &str {
        "🚀"
    }

    fn render(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.heading("My Custom Module");
        ui.label("This is where your content goes!");
    }
}
```

## ⚙️ Configuration

### Project Configuration

Edit `project.toml` to customize project settings:

```toml
[project]
name = "My Desktop App"
version = "0.1.0"
default_theme = "dark"

[layout]
left_sidebar_width = 48.0
right_sidebar_width = 300.0
show_status_bar = true
window_min_size = [800, 600]
window_default_size = [1200, 800]
```

### Module Configuration

Create `config/modules/my_module.toml` for module-specific settings:

```toml
[module]
enabled = true
auto_load = true
auto_activate = false

[ui.show_in_navigation = true
navigation_position = 5
shortcut = "Ctrl+Shift+M"

[permissions]
file_system_access = true
network_access = false
system_info_access = false
settings_access = true

[settings]
custom_setting = "value"
number_setting = 42
```

## 🔧 Development

### Building Your Application

```bash
# Development build (faster, includes debugging)
cargo run

# Release build (optimized for production)
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

### Cross-Platform Building

```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# macOS (Intel)
cargo build --release --target x86_64-apple-darwin

# macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

### Adding Dependencies

Add dependencies to `Cargo.toml`:

```toml
[dependencies]
# Add your dependencies here
your-crate = "1.0.0"
```

Then run:

```bash
cargo check  # Verify the dependency resolves
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test module_name

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test --test-threads=4
```

### Writing Tests

Create unit tests in `tests/`:

```rust
use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = MyCustomModule::new();
        assert_eq!(module.name(), "My Custom Module");
    }

    #[test]
    fn test_ui_rendering() {
        // Test UI rendering here
    }
}
```

### Integration Tests

Create integration tests in `tests/integration/`:

```rust
use crate::MyCustomModule;

#[test]
fn test_module_integration() {
    let app = crate::App::new(&cc);
    // Test integration scenarios here
}
```

## 📚 Documentation

### Generated Documentation

```bash
# Generate and view documentation
cargo doc --open
```

### Code Documentation

Use `///` comments to document your code:

```rust
/// Adds two numbers together
///
/// # Arguments
/// * `a` - First number to add
/// * `b` - Second number to add
///
/// # Returns
/// The sum of `a` and `b`
///
/// # Examples
/// ```
/// let result = add_numbers(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```

## 🐛 Deployment

### Creating Distribution

```bash
# Create release build
cargo build --release

# The binary will be in:
# target/release/your-app (Linux/macOS)
# target/release/your-app.exe (Windows)
```

### Cross-Platform Packages

For platform-specific packages, use tools like:

- **Windows**: NSIS, WiX Toolset
- **macOS**: `create-dmg`, `pkgbuild`
- **Linux**: AppImage, Debian packages

### Code Signing

For distribution, sign your application:

```bash
# macOS code signing
codesign -s "Developer ID Application: Your ID" target/release/your-app

# Windows code signing
signtool sign /a /fd "Your Certificate" target/release/your-app.exe
```

## 🔍 Debugging

### Logging

The application uses structured logging with tracing. Enable debug logging:

```bash
RUST_LOG=debug cargo run
```

### Panic Information

For crash information, set the backtrace:

```bash
RUST_BACKTRACE=1 cargo run
```

### Profiling

Use profiling tools to analyze performance:

```bash
# CPU profiling
cargo install cargo-flamegraph
cargo flamegraph --bin target/release/your-app

# Memory profiling
cargo install cargo-valgrind
valgrind --tool=massif target/release/your-app
```

## 🤝 Getting Help

### Common Issues

**Application won't start:**
1. Check Rust version: `rustc --version`
2. Update dependencies: `cargo update`
3. Clean build: `cargo clean && cargo build`

**UI looks wrong:**
1. Check theme settings
2. Verify egui/eframe versions
3. Check system font availability

**Performance issues:**
1. Use release build: `cargo build --release`
2. Profile with tools mentioned above
3. Check for expensive operations in render loops

### Community Support

- 🐛 [Issues](https://github.com/your-org/rust-desktop-template/issues) - Report bugs and request features
- 💬 [Discussions](https://github.com/your-org/rust-desktop-template/discussions) - Ask questions and share ideas
- 📧 [Wiki](https://github.com/your-org/rust-desktop-template/wiki) - Additional documentation

### Professional Support

For enterprise support or custom development services:

- 📧 Email: support@example.com
- 💬 Website: https://your-company.com
- 📞 Phone: +1 (555) 123-4567

## 🎉 Next Steps

Congratulations! You now have a working Rust desktop application. Here are some ideas for what you can do next:

### Immediate Improvements

1. **Add Your First Module**: Implement a custom module for your specific use case
2. **Customize the Theme**: Create a unique color scheme or visual style
3. **Add Settings**: Create application-specific configuration options
4. **Add Tests**: Write comprehensive tests for your functionality

### Advanced Features

1. **Database Integration**: Add database support for data persistence
2. **Network Features**: Implement REST API clients or web views
3. **Plugin System**: Create a plugin architecture for extensibility
4. **Automation**: Add scripting capabilities or task automation
5. **Internationalization**: Add multi-language support

### Deployment

1. **Distribution**: Create installers for your target platforms
2. **Auto-Updater**: Implement automatic update mechanisms
3. **Telemetry**: Add usage analytics and crash reporting
4. **CI/CD**: Set up automated builds and releases

---

**Happy coding!** 🚀

For more detailed information, see the [Architecture Overview](architecture.md) or [Module Development Guide](module-development.md).