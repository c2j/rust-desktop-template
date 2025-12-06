//! UI components for the desktop application

pub mod navigation;
pub mod theme;
pub mod workspace;
pub mod status_bar;
pub mod icons;

pub use navigation::Navigation;
pub use theme::{Theme, ThemeType};
pub use workspace::Workspace;