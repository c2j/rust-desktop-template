# {{project-name}}

A modern Rust desktop application template built with egui and eframe, providing a VS Code-like interface with modular architecture.

## Features

- 🖥️ **Modern Desktop UI** - Built with egui/eframe for fast, cross-platform desktop applications
- 🎨 **Theme System** - Dark and light themes with easy customization
- 📁 **Modular Architecture** - Extensible module system for different functionalities
- 🔀 **Navigation Panel** - VS Code-style sidebar navigation
- 📊 **Status Bar** - System information and application status display
- 🛠️ **Configuration Management** - Flexible configuration system
- 📝 **Error Handling** - Comprehensive error handling with tracing
- 🔌 **Plugin System** - Easy-to-extend module system

## Quick Start

### Prerequisites

- Rust 1.75+ (latest stable recommended)
- Cargo (included with Rust)

### Using as Template

```bash
# Create a new project from this template
cargo generate --git <this-repo-url> my-app
cd my-app

# Run your new application
cargo run
```

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd {{project-name}}

# Build and run
cargo run
```

## Development

### Project Structure

```
src/
├── app.rs              # Main application logic and state management
├── main.rs             # Application entry point
├── lib.rs              # Library entry point
├── error.rs            # Error handling and custom error types
├── logger.rs           # Logging configuration
├── config/             # Configuration management
│   ├── mod.rs          # Main configuration
│   └── theme.rs        # Theme configuration
├── modules/            # Application modules
│   ├── mod.rs          # Module system and trait definitions
│   ├── registry.rs     # Module registry for managing modules
│   ├── state.rs        # Module state management
│   ├── lifecycle.rs    # Module lifecycle events
│   ├── file_browser.rs # File browser module
│   ├── text_editor.rs  # Text editor module
│   └── system_monitor.rs # System monitor module
└── ui/                 # User interface components
    ├── mod.rs          # UI module definitions
    ├── theme.rs        # Theme system and color palettes
    ├── navigation.rs   # Navigation sidebar component
    ├── workspace.rs    # Main workspace component
    ├── status_bar.rs   # Status bar component
    └── icons.rs        # Icon definitions
```

### Adding New Modules

1. Create a new module file in `src/modules/`
2. Implement the `ApplicationModule` trait
3. Register the module in the module registry

Example:

```rust
// src/modules/my_module.rs
use crate::modules::{ApplicationModule, ModuleId};
use egui::{Context, Ui};

pub struct MyModule {
    // Module state
}

impl ApplicationModule for MyModule {
    fn id(&self) -> ModuleId {
        ModuleId::Custom("my_module".to_string())
    }

    fn name(&self) -> &str {
        "My Module"
    }

    fn icon(&self) -> &str {
        "🔧"
    }

    fn render(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.heading("My Module Content");
        // Your module's UI rendering code here
    }
}
```

### Customizing Themes

Themes are defined in `src/ui/theme.rs`. You can modify colors, spacing, and other visual properties:

```rust
let mut colors = ColorPalette::dark();
colors.primary = egui::Color32::from_rgb(0x4A, 0x90, 0xE2);
// Customize other colors as needed

let theme = Theme::dark()
    .with_colors(colors)
    .with_spacing(Spacing::default());
```

### Configuration

Application configuration is managed through the `Config` struct. You can:

- Add new configuration fields
- Implement custom validation
- Add support for environment variables
- Create configuration files (TOML, JSON, etc.)

## Architecture

### Module System

The application uses a plugin-based architecture:

- **ApplicationModule Trait**: Defines the interface for all modules
- **ModuleRegistry**: Manages module registration and lifecycle
- **AppState**: Shared application state accessible by all modules
- **Navigation**: VS Code-style sidebar for module switching

### UI Components

- **Navigation**: Sidebar for module navigation with icons and tooltips
- **Workspace**: Main content area where modules are rendered
- **StatusBar**: Bottom status bar showing system information
- **Theme System**: Centralized theme management with multiple built-in themes

### State Management

- **Arc<RwLock<AppState>>**: Thread-safe shared state
- **Module State**: Each module can manage its own state
- **Configuration**: Centralized configuration system

## Building

### Debug Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Check Code Quality

```bash
# Check for errors
cargo check

# Run Clippy for additional checks
cargo clippy

# Format code
cargo fmt
```

## Dependencies

Key dependencies include:

- `egui` 0.28.1 - Immediate mode GUI library
- `eframe` 0.28.1 - egui framework wrapper
- `serde` - Serialization/deserialization
- `tracing` - Structured logging
- `chrono` - Date and time handling
- `dirs` - Cross-platform directory paths

See `Cargo.toml` for the complete list.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust best practices and conventions
- Add tests for new functionality
- Update documentation for API changes
- Ensure all tests pass before submitting

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [egui](https://github.com/emilk/egui) - Immediate mode GUI library
- [eframe](https://github.com/emilk/egui) - egui framework wrapper
- [cargo-generate](https://github.com/cargo-generate/cargo-generate) - Project template generator

## Changelog

### v0.1.0
- Initial release with basic VS Code-like interface
- Module system with file browser, text editor, and system monitor
- Theme system with dark/light themes
- Configuration management
- Status bar with system information
- Cross-platform desktop support