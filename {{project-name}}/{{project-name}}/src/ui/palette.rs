//! Advanced color palette management for theming

use crate::error::Result;
use egui::Color32;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Color palette for advanced theming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Primary colors
    pub primary: ColorSet,
    /// Secondary colors
    pub secondary: ColorSet,
    /// Surface colors
    pub surface: ColorSet,
    /// Text colors
    pub text: ColorSet,
    /// Border colors
    pub border: ColorSet,
    /// Status colors
    pub status: StatusColors,
    /// Interactive colors
    pub interactive: InteractiveColors,
    /// Custom colors
    pub custom: HashMap<String, Color32>,
}

/// Color set with light, normal, dark, and hover variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSet {
    /// Lightest variant
    pub light: Color32,
    /// Normal variant
    pub normal: Color32,
    /// Darkest variant
    pub dark: Color32,
    /// Hover variant
    pub hover: Color32,
}

/// Status colors for different states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusColors {
    pub success: ColorSet,
    pub warning: ColorSet,
    pub error: ColorSet,
    pub info: ColorSet,
}

/// Interactive colors for UI elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveColors {
    pub button: ColorSet,
    pub input: ColorSet,
    pub link: ColorSet,
    pub selection: ColorSet,
}

/// Predefined color schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorScheme {
    /// Light theme colors
    Light,
    /// Dark theme colors
    Dark,
    /// High contrast colors
    HighContrast,
    /// Sepia theme colors
    Sepia,
    /// Ocean theme colors
    Ocean,
    /// Forest theme colors
    Forest,
    /// Sunset theme colors
    Sunset,
    /// Custom scheme
    Custom,
}

impl ColorPalette {
    /// Create a new color palette
    pub fn new() -> Self {
        Self::default()
    }

    /// Create palette from color scheme
    pub fn from_scheme(scheme: ColorScheme) -> Self {
        match scheme {
            ColorScheme::Light => Self::light(),
            ColorScheme::Dark => Self::dark(),
            ColorScheme::HighContrast => Self::high_contrast(),
            ColorScheme::Sepia => Self::sepia(),
            ColorScheme::Ocean => Self::ocean(),
            ColorScheme::Forest => Self::forest(),
            ColorScheme::Sunset => Self::sunset(),
            ColorScheme::Custom => Self::default(),
        }
    }

    /// Create light theme palette
    pub fn light() -> Self {
        Self {
            primary: ColorSet {
                light: Color32::from_rgb(0x4D, 0x90, 0xFE),
                normal: Color32::from_rgb(0x00, 0x7A, 0xCC),
                dark: Color32::from_rgb(0x00, 0x5A, 0x99),
                hover: Color32::from_rgb(0x2A, 0x8A, 0xDE),
            },
            secondary: ColorSet {
                light: Color32::from_rgb(0x50, 0xA0, 0xFF),
                normal: Color32::from_rgb(0x26, 0xA0, 0xDA),
                dark: Color32::from_rgb(0x18, 0x70, 0xC3),
                hover: Color32::from_rgb(0x40, 0xB0, 0xFF),
            },
            surface: ColorSet {
                light: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                normal: Color32::from_rgb(0xF8, 0xF8, 0xF8),
                dark: Color32::from_rgb(0xE8, 0xE8, 0xE8),
                hover: Color32::from_rgb(0xF0, 0xF0, 0xF0),
            },
            text: ColorSet {
                light: Color32::from_rgb(0x60, 0x60, 0x60),
                normal: Color32::from_rgb(0x00, 0x00, 0x00),
                dark: Color32::from_rgb(0x00, 0x00, 0x00),
                hover: Color32::from_rgb(0x00, 0x00, 0x00),
            },
            border: ColorSet {
                light: Color32::from_rgb(0xE0, 0xE0, 0xE0),
                normal: Color32::from_rgb(0xD0, 0xD0, 0xD0),
                dark: Color32::from_rgb(0xB0, 0xB0, 0xB0),
                hover: Color32::from_rgb(0xC0, 0xC0, 0xC0),
            },
            status: StatusColors {
                success: ColorSet {
                    light: Color32::from_rgb(0xA8, 0xD5, 0x82),
                    normal: Color32::from_rgb(0x16, 0xC6, 0x0C),
                    dark: Color32::from_rgb(0x0E, 0x8E, 0x0A),
                    hover: Color32::from_rgb(0x20, 0xD0, 0x20),
                },
                warning: ColorSet {
                    light: Color32::from_rgb(0xFF, 0xCC, 0x82),
                    normal: Color32::from_rgb(0xFF, 0x8F, 0x00),
                    dark: Color32::from_rgb(0xCC, 0x70, 0x00),
                    hover: Color32::from_rgb(0xFF, 0xAA, 0x00),
                },
                error: ColorSet {
                    light: Color32::from_rgb(0xE5, 0x6B, 0x6B),
                    normal: Color32::from_rgb(0xE5, 0x14, 0x00),
                    dark: Color32::from_rgb(0xB0, 0x10, 0x00),
                    hover: Color32::from_rgb(0xFF, 0x30, 0x30),
                },
                info: ColorSet {
                    light: Color32::from_rgb(0x82, 0xA5, 0xD5),
                    normal: Color32::from_rgb(0x1E, 0x88, 0xE5),
                    dark: Color32::from_rgb(0x15, 0x68, 0xB3),
                    hover: Color32::from_rgb(0x30, 0xA0, 0xFF),
                },
            },
            interactive: InteractiveColors {
                button: ColorSet {
                    light: Color32::from_rgb(0xF5, 0xF5, 0xF5),
                    normal: Color32::from_rgb(0xEE, 0xEE, 0xEE),
                    dark: Color32::from_rgb(0xDD, 0xDD, 0xDD),
                    hover: Color32::from_rgb(0xE8, 0xE8, 0xE8),
                },
                input: ColorSet {
                    light: Color32::from_rgb(0xFF, 0xFF, 0xFF),
                    normal: Color32::from_rgb(0xFA, 0xFA, 0xFA),
                    dark: Color32::from_rgb(0xF5, 0xF5, 0xF5),
                    hover: Color32::from_rgb(0xFE, 0xFE, 0xFE),
                },
                link: ColorSet {
                    light: Color32::from_rgb(0x33, 0x66, 0xCC),
                    normal: Color32::from_rgb(0x00, 0x66, 0xCC),
                    dark: Color32::from_rgb(0x00, 0x4D, 0x99),
                    hover: Color32::from_rgb(0x00, 0x80, 0xFF),
                },
                selection: ColorSet {
                    light: Color32::from_rgb(0xCC, 0xCC, 0xCC),
                    normal: Color32::from_rgb(0x99, 0x99, 0x99),
                    dark: Color32::from_rgb(0x66, 0x66, 0x66),
                    hover: Color32::from_rgb(0xB3, 0xB3, 0xB3),
                },
            },
            custom: HashMap::new(),
        }
    }

    /// Create dark theme palette
    pub fn dark() -> Self {
        Self {
            primary: ColorSet {
                light: Color32::from_rgb(0x6D, 0xA0, 0xFF),
                normal: Color32::from_rgb(0x00, 0x7A, 0xCC),
                dark: Color32::from_rgb(0x00, 0x5A, 0x99),
                hover: Color32::from_rgb(0x2A, 0x8A, 0xDE),
            },
            secondary: ColorSet {
                light: Color32::from_rgb(0x70, 0xB0, 0xFF),
                normal: Color32::from_rgb(0x26, 0xA0, 0xDA),
                dark: Color32::from_rgb(0x18, 0x70, 0xC3),
                hover: Color32::from_rgb(0x40, 0xB0, 0xFF),
            },
            surface: ColorSet {
                light: Color32::from_rgb(0x35, 0x35, 0x35),
                normal: Color32::from_rgb(0x25, 0x25, 0x25),
                dark: Color32::from_rgb(0x1E, 0x1E, 0x1E),
                hover: Color32::from_rgb(0x2E, 0x2E, 0x2E),
            },
            text: ColorSet {
                light: Color32::from_rgb(0xA0, 0xA0, 0xA0),
                normal: Color32::from_rgb(0xD4, 0xD4, 0xD4),
                dark: Color32::from_rgb(0xE0, 0xE0, 0xE0),
                hover: Color32::from_rgb(0xB0, 0xB0, 0xB0),
            },
            border: ColorSet {
                light: Color32::from_rgb(0x55, 0x55, 0x55),
                normal: Color32::from_rgb(0x45, 0x45, 0x45),
                dark: Color32::from_rgb(0x35, 0x35, 0x35),
                hover: Color32::from_rgb(0x55, 0x55, 0x55),
            },
            status: StatusColors {
                success: ColorSet {
                    light: Color32::from_rgb(0xB8, 0xE6, 0x86),
                    normal: Color32::from_rgb(0x89, 0xD1, 0x85),
                    dark: Color32::from_rgb(0x5E, 0x91, 0x5C),
                    hover: Color32::from_rgb(0x30, 0xD0, 0x30),
                },
                warning: ColorSet {
                    light: Color32::from_rgb(0xFF, 0xD8, 0x82),
                    normal: Color32::from_rgb(0xFF, 0xCC, 0x02),
                    dark: Color32::from_rgb(0xCC, 0xA0, 0x00),
                    hover: Color32::from_rgb(0xFF, 0xAA, 0x00),
                },
                error: ColorSet {
                    light: Color32::from_rgb(0xF4, 0x7B, 0x47),
                    normal: Color32::from_rgb(0xF4, 0x47, 0x47),
                    dark: Color32::from_rgb(0xB0, 0x30, 0x00),
                    hover: Color32::from_rgb(0xFF, 0x60, 0x60),
                },
                info: ColorSet {
                    light: Color32::from_rgb(0x92, 0xB5, 0xE5),
                    normal: Color32::from_rgb(0x1E, 0x88, 0xE5),
                    dark: Color32::from_rgb(0x15, 0x68, 0xB3),
                    hover: Color32::from_rgb(0x30, 0xA0, 0xFF),
                },
            },
            interactive: InteractiveColors {
                button: ColorSet {
                    light: Color32::from_rgb(0x45, 0x45, 0x45),
                    normal: Color32::from_rgb(0x35, 0x35, 0x35),
                    dark: Color32::from_rgb(0x25, 0x25, 0x25),
                    hover: Color32::from_rgb(0x55, 0x55, 0x55),
                },
                input: ColorSet {
                    light: Color32::from_rgb(0x45, 0x45, 0x45),
                    normal: Color32::from_rgb(0x30, 0x30, 0x30),
                    dark: Color32::from_rgb(0x20, 0x20, 0x20),
                    hover: Color32::from_rgb(0x40, 0x40, 0x40),
                },
                link: ColorSet {
                    light: Color32::from_rgb(0x66, 0x99, 0xFF),
                    normal: Color32::from_rgb(0x40, 0x80, 0xFF),
                    dark: Color32::from_rgb(0x33, 0x66, 0xCC),
                    hover: Color32::from_rgb(0x60, 0xA0, 0xFF),
                },
                selection: ColorSet {
                    light: Color32::from_rgb(0x66, 0x66, 0x66),
                    normal: Color32::from_rgb(0x55, 0x55, 0x55),
                    dark: Color32::from_rgb(0x44, 0x44, 0x44),
                    hover: Color32::from_rgb(0x77, 0x77, 0x77),
                },
            },
            custom: HashMap::new(),
        }
    }

    /// Create high contrast palette
    pub fn high_contrast() -> Self {
        Self {
            primary: ColorSet {
                light: Color32::WHITE,
                normal: Color32::WHITE,
                dark: Color32::GRAY,
                hover: Color32::WHITE,
            },
            secondary: ColorSet {
                light: Color32::WHITE,
                normal: Color32::WHITE,
                dark: Color32::GRAY,
                hover: Color32::WHITE,
            },
            surface: ColorSet {
                light: Color32::BLACK,
                normal: Color32::BLACK,
                dark: Color32::BLACK,
                hover: Color32::BLACK,
            },
            text: ColorSet {
                light: Color32::WHITE,
                normal: Color32::WHITE,
                dark: Color32::WHITE,
                hover: Color32::WHITE,
            },
            border: ColorSet {
                light: Color32::WHITE,
                normal: Color32::WHITE,
                dark: Color32::GRAY,
                hover: Color32::WHITE,
            },
            status: StatusColors {
                success: ColorSet {
                    light: Color32::WHITE,
                    normal: Color32::WHITE,
                    dark: Color32::GRAY,
                    hover: Color32::WHITE,
                },
                warning: ColorSet {
                    light: Color32::WHITE,
                    normal: Color32::YELLOW,
                    dark: Color32::YELLOW,
                    hover: Color32::WHITE,
                },
                error: ColorSet {
                    light: Color32::WHITE,
                    normal: Color32::WHITE,
                    dark: Color32::RED,
                    hover: Color32::WHITE,
                },
                info: ColorSet {
                    light: Color32::WHITE,
                    normal: Color32::WHITE,
                    dark: Color32::CYAN,
                    hover: Color32::WHITE,
                },
            },
            interactive: InteractiveColors {
                button: ColorSet {
                    light: Color32::WHITE,
                    normal: Color32::WHITE,
                    dark: Color32::GRAY,
                    hover: Color32::WHITE,
                },
                input: ColorSet {
                    light: Color32::WHITE,
                    normal: Color32::WHITE,
                    dark: Color32::GRAY,
                    hover: Color32::WHITE,
                },
                link: ColorSet {
                    light: Color32::WHITE,
                    normal: Color32::WHITE,
                    dark: Color32::YELLOW,
                    hover: Color32::WHITE,
                },
                selection: ColorSet {
                    light: Color32::WHITE,
                    normal: Color32::WHITE,
                    dark: Color32::WHITE,
                    hover: Color32::WHITE,
                },
            },
            custom: HashMap::new(),
        }
    }

    /// Create sepia palette
    pub fn sepia() -> Self {
        Self {
            primary: ColorSet {
                light: Color32::from_rgb(0x8B, 0x73, 0x55),
                normal: Color32::from_rgb(0x6B, 0x5A, 0x42),
                dark: Color32::from_rgb(0x4B, 0x3A, 0x22),
                hover: Color32::from_rgb(0x9B, 0x8A, 0x62),
            },
            secondary: ColorSet {
                light: Color32::from_rgb(0x9B, 0x8A, 0x6A),
                normal: Color32::from_rgb(0x7B, 0x6A, 0x4A),
                dark: Color32::from_rgb(0x5B, 0x4A, 0x2A),
                hover: Color32::from_rgb(0xAB, 0x9A, 0x7A),
            },
            surface: ColorSet {
                light: Color32::from_rgb(0xF5, 0xEF, 0xE5),
                normal: Color32::from_rgb(0xEB, 0xE5, 0xD5),
                dark: Color32::from_rgb(0xDB, 0xD5, 0xC5),
                hover: Color32::from_rgb(0xF0, 0xEA, 0xDA),
            },
            text: ColorSet {
                light: Color32::from_rgb(0x5C, 0x4A, 0x32),
                normal: Color32::from_rgb(0x3C, 0x2A, 0x22),
                dark: Color32::from_rgb(0x2C, 0x1A, 0x12),
                hover: Color32::from_rgb(0x4C, 0x3A, 0x22),
            },
            border: ColorSet {
                light: Color32::from_rgb(0xC4, 0xAE, 0x8E),
                normal: Color32::from_rgb(0xB4, 0x9E, 0x7E),
                dark: Color32::from_rgb(0xA4, 0x8E, 0x6E),
                hover: Color32::from_rgb(0xC9, 0xB3, 0x93),
            },
            status: StatusColors {
                success: ColorSet {
                    light: Color32::from_rgb(0x7A, 0x6B, 0x50),
                    normal: Color32::from_rgb(0x5A, 0x4B, 0x30),
                    dark: Color32::from_rgb(0x4A, 0x3B, 0x20),
                    hover: Color32::from_rgb(0x6A, 0x5B, 0x40),
                },
                warning: ColorSet {
                    light: Color32::from_rgb(0x8B, 0x73, 0x43),
                    normal: Color32::from_rgb(0x6B, 0x53, 0x23),
                    dark: Color32::from_rgb(0x5B, 0x43, 0x13),
                    hover: Color32::from_rgb(0x7B, 0x63, 0x33),
                },
                error: ColorSet {
                    light: Color32::from_rgb(0x8B, 0x52, 0x43),
                    normal: Color32::from_rgb(0x6B, 0x32, 0x23),
                    dark: Color32::from_rgb(0x5B, 0x22, 0x13),
                    hover: Color32::from_rgb(0x7B, 0x42, 0x33),
                },
                info: ColorSet {
                    light: Color32::from_rgb(0x7A, 0x6B, 0x73),
                    normal: Color32::from_rgb(0x5A, 0x4B, 0x43),
                    dark: Color32::from_rgb(0x4A, 0x3B, 0x33),
                    hover: Color32::from_rgb(0x6A, 0x5B, 0x53),
                },
            },
            interactive: InteractiveColors {
                button: ColorSet {
                    light: Color32::from_rgb(0xC4, 0xAE, 0x8E),
                    normal: Color32::from_rgb(0xB4, 0x9E, 0x7E),
                    dark: Color32::from_rgb(0xA4, 0x8E, 0x6E),
                    hover: Color32::from_rgb(0xC9, 0xB3, 0x93),
                },
                input: ColorSet {
                    light: Color32::from_rgb(0xEB, 0xE5, 0xD5),
                    normal: Color32::from_rgb(0xDB, 0xD5, 0xC5),
                    dark: Color32::from_rgb(0xCB, 0xC5, 0xB5),
                    hover: Color32::from_rgb(0xF0, 0xEA, 0xDA),
                },
                link: ColorSet {
                    light: Color32::from_rgb(0x6B, 0x4A, 0x3A),
                    normal: Color32::from_rgb(0x5B, 0x3A, 0x2A),
                    dark: Color32::from_rgb(0x4B, 0x2A, 0x1A),
                    hover: Color32::from_rgb(0x6B, 0x4A, 0x3A),
                },
                selection: ColorSet {
                    light: Color32::from_rgb(0x7A, 0x6B, 0x50),
                    normal: Color32::from_rgb(0x5A, 0x4B, 0x30),
                    dark: Color32::from_rgb(0x4A, 0x3B, 0x20),
                    hover: Color32::from_rgb(0x6A, 0x5B, 0x40),
                },
            },
            custom: HashMap::new(),
        }
    }

    /// Create ocean palette
    pub fn ocean() -> Self {
        Self {
            primary: ColorSet {
                light: Color32::from_rgb(0x6A, 0xA0, 0xE0),
                normal: Color32::from_rgb(0x3A, 0x70, 0xB0),
                dark: Color32::from_rgb(0x2A, 0x50, 0x80),
                hover: Color32::from_rgb(0x4A, 0x80, 0xC0),
            },
            secondary: ColorSet {
                light: Color32::from_rgb(0x7A, 0xB0, 0xF0),
                normal: Color32::from_rgb(0x4A, 0x80, 0xC0),
                dark: Color32::from_rgb(0x3A, 0x60, 0x90),
                hover: Color32::from_rgb(0x5A, 0x90, 0xD0),
            },
            surface: ColorSet {
                light: Color32::from_rgb(0xE8, 0xF4, 0xFC),
                normal: Color32::from_rgb(0xD8, 0xE8, 0xF8),
                dark: Color32::from_rgb(0xC8, 0xD8, 0xE8),
                hover: Color32::from_rgb(0xE0, 0xF0, 0xF8),
            },
            text: ColorSet {
                light: Color32::from_rgb(0x40, 0x60, 0x80),
                normal: Color32::from_rgb(0x20, 0x40, 0x60),
                dark: Color32::from_rgb(0x10, 0x30, 0x50),
                hover: Color32::from_rgb(0x30, 0x50, 0x70),
            },
            border: ColorSet {
                light: Color32::from_rgb(0x80, 0xA0, 0xC0),
                normal: Color32::from_rgb(0x60, 0x80, 0xA0),
                dark: Color32::from_rgb(0x40, 0x60, 0x80),
                hover: Color32::from_rgb(0x70, 0x90, 0xB0),
            },
            status: StatusColors {
                success: ColorSet {
                    light: Color32::from_rgb(0x7A, 0xE0, 0xA0),
                    normal: Color32::from_rgb(0x4A, 0xB0, 0x70),
                    dark: Color32::from_rgb(0x3A, 0x80, 0x50),
                    hover: Color32::from_rgb(0x5A, 0xC0, 0x80),
                },
                warning: ColorSet {
                    light: Color32::from_rgb(0xF0, 0xE0, 0x80),
                    normal: Color32::from_rgb(0xD0, 0xC0, 0x60),
                    dark: Color32::from_rgb(0xB0, 0xA0, 0x40),
                    hover: Color32::from_rgb(0xE0, 0xD0, 0x70),
                },
                error: ColorSet {
                    light: Color32::from_rgb(0xA0, 0xE0, 0xF0),
                    normal: Color32::from_rgb(0x80, 0xC0, 0xE0),
                    dark: Color32::from_rgb(0x60, 0xA0, 0xC0),
                    hover: Color32::from_rgb(0x90, 0xD0, 0xF0),
                },
                info: ColorSet {
                    light: Color32::from_rgb(0xA0, 0xD0, 0xF0),
                    normal: Color32::from_rgb(0x80, 0xB0, 0xE0),
                    dark: Color32::from_rgb(0x60, 0x90, 0xC0),
                    hover: Color32::from_rgb(0x90, 0xC0, 0xF0),
                },
            },
            interactive: InteractiveColors {
                button: ColorSet {
                    light: Color32::from_rgb(0xD0, 0xE8, 0xF8),
                    normal: Color32::from_rgb(0xB0, 0xC8, 0xE8),
                    dark: Color32::from_rgb(0x90, 0xA8, 0xC8),
                    hover: Color32::from_rgb(0xC0, 0xD8, 0xF0),
                },
                input: ColorSet {
                    light: Color32::from_rgb(0xF0, 0xF8, 0xFC),
                    normal: Color32::from_rgb(0xE8, 0xF0, 0xF8),
                    dark: Color32::from_rgb(0xD8, 0xE0, 0xE8),
                    hover: Color32::from_rgb(0xF0, 0xF8, 0xFC),
                },
                link: ColorSet {
                    light: Color32::from_rgb(0x7A, 0x9A, 0xE0),
                    normal: Color32::from_rgb(0x5A, 0x7A, 0xC0),
                    dark: Color32::from_rgb(0x4A, 0x5A, 0xA0),
                    hover: Color32::from_rgb(0x6A, 0x8A, 0xE0),
                },
                selection: ColorSet {
                    light: Color32::from_rgb(0xA0, 0xC0, 0xE0),
                    normal: Color32::from_rgb(0x80, 0xA0, 0xC0),
                    dark: Color32::from_rgb(0x60, 0x80, 0xA0),
                    hover: Color32::from_rgb(0x90, 0xB0, 0xD0),
                },
            },
            custom: HashMap::new(),
        }
    }

    /// Create forest palette
    pub fn forest() -> Self {
        Self {
            primary: ColorSet {
                light: Color32::from_rgb(0x7A, 0xB8, 0x6A),
                normal: Color32::from_rgb(0x4A, 0x88, 0x4A),
                dark: Color32::from_rgb(0x2A, 0x58, 0x2A),
                hover: Color32::from_rgb(0x5A, 0x98, 0x5A),
            },
            secondary: ColorSet {
                light: Color32::from_rgb(0x8A, 0xC8, 0x7A),
                normal: Color32::from_rgb(0x6A, 0xA8, 0x5A),
                dark: Color32::from_rgb(0x4A, 0x88, 0x3A),
                hover: Color32::from_rgb(0x7A, 0xB8, 0x6A),
            },
            surface: ColorSet {
                light: Color32::from_rgb(0xE8, 0xF4, 0xE0),
                normal: Color32::from_rgb(0xD8, 0xE8, 0xD0),
                dark: Color32::from_rgb(0xC8, 0xD8, 0xC0),
                hover: Color32::from_rgb(0xE0, 0xF0, 0xE0),
            },
            text: ColorSet {
                light: Color32::from_rgb(0x40, 0x50, 0x30),
                normal: Color32::from_rgb(0x30, 0x40, 0x20),
                dark: Color32::from_rgb(0x20, 0x30, 0x10),
                hover: Color32::from_rgb(0x50, 0x60, 0x40),
            },
            border: ColorSet {
                light: Color32::from_rgb(0x80, 0x90, 0x70),
                normal: Color32::from_rgb(0x60, 0x70, 0x50),
                dark: Color32::from_rgb(0x40, 0x50, 0x30),
                hover: Color32::from_rgb(0x70, 0x80, 0x60),
            },
            status: StatusColors {
                success: ColorSet {
                    light: Color32::from_rgb(0x9A, 0xC8, 0x8A),
                    normal: Color32::from_rgb(0x6A, 0xA8, 0x6A),
                    dark: Color32::from_rgb(0x4A, 0x88, 0x4A),
                    hover: Color32::from_rgb(0x7A, 0xB8, 0x6A),
                },
                warning: ColorSet {
                    light: Color32::from_rgb(0xB8, 0xC8, 0x6A),
                    normal: Color32::from_rgb(0x98, 0xA8, 0x4A),
                    dark: Color32::from_rgb(0x78, 0x88, 0x2A),
                    hover: Color32::from_rgb(0xA8, 0xB8, 0x5A),
                },
                error: ColorSet {
                    light: Color32::from_rgb(0xA0, 0xB8, 0x6A),
                    normal: Color32::from_rgb(0x80, 0x88, 0x4A),
                    dark: Color32::from_rgb(0x60, 0x68, 0x2A),
                    hover: Color32::from_rgb(0x90, 0x98, 0x5A),
                },
                info: ColorSet {
                    light: Color32::from_rgb(0x8A, 0xA8, 0xC8),
                    normal: Color32::from_rgb(0x6A, 0x88, 0xA8),
                    dark: Color32::from_rgb(0x4A, 0x68, 0x88),
                    hover: Color32::from_rgb(0x7A, 0x88, 0xB8),
                },
            },
            interactive: InteractiveColors {
                button: ColorSet {
                    light: Color32::from_rgb(0xD0, 0xE0, 0xC0),
                    normal: Color32::from_rgb(0xC0, 0xD0, 0xB0),
                    dark: Color32::from_rgb(0xA0, 0xB0, 0x90),
                    hover: Color32::from_rgb(0xD8, 0xE8, 0xC8),
                },
                input: ColorSet {
                    light: Color32::from_rgb(0xF0, 0xF8, 0xE0),
                    normal: Color32::from_rgb(0xE0, 0xF0, 0xD0),
                    dark: Color32::from_rgb(0xD0, 0xE0, 0xC0),
                    hover: Color32::from_rgb(0xF0, 0xF8, 0xE0),
                },
                link: ColorSet {
                    light: Color32::from_rgb(0x6A, 0x8A, 0x4A),
                    normal: Color32::from_rgb(0x5A, 0x7A, 0x3A),
                    dark: Color32::from_rgb(0x4A, 0x6A, 0x2A),
                    hover: Color32::from_rgb(0x6A, 0x8A, 0x4A),
                },
                selection: ColorSet {
                    light: Color32::from_rgb(0xA0, 0xC0, 0x80),
                    normal: Color32::from_rgb(0x80, 0xA0, 0x60),
                    dark: Color32::from_rgb(0x60, 0x80, 0x40),
                    hover: Color32::from_rgb(0x90, 0xB0, 0x70),
                },
            },
            custom: HashMap::new(),
        }
    }

    /// Create sunset palette
    pub fn sunset() -> Self {
        Self {
            primary: ColorSet {
                light: Color32::from_rgb(0xE0, 0x70, 0x6A),
                normal: Color32::from_rgb(0xC0, 0x50, 0x4A),
                dark: Color32::from_rgb(0x90, 0x30, 0x2A),
                hover: Color32::from_rgb(0xD0, 0x60, 0x5A),
            },
            secondary: ColorSet {
                light: Color32::from_rgb(0xF0, 0x90, 0x8A),
                normal: Color32::from_rgb(0xD0, 0x70, 0x6A),
                dark: Color32::from_rgb(0xB0, 0x50, 0x4A),
                hover: Color32::from_rgb(0xE0, 0x80, 0x7A),
            },
            surface: ColorSet {
                light: Color32::from_rgb(0xF8, 0xE8, 0xE0),
                normal: Color32::from_rgb(0xF0, 0xD8, 0xD0),
                dark: Color32::from_rgb(0xE0, 0xC8, 0xC0),
                hover: Color32::from_rgb(0xF5, 0xE0, 0xD8),
            },
            text: ColorSet {
                light: Color32::from_rgb(0x60, 0x30, 0x2A),
                normal: Color32::from_rgb(0x40, 0x20, 0x1A),
                dark: Color32::from_rgb(0x50, 0x30, 0x2A),
                hover: Color32::from_rgb(0x70, 0x40, 0x3A),
            },
            border: ColorSet {
                light: Color32::from_rgb(0xC0, 0x70, 0x6A),
                normal: Color32::from_rgb(0xA0, 0x50, 0x4A),
                dark: Color32::from_rgb(0x80, 0x30, 0x2A),
                hover: Color32::from_rgb(0xB0, 0x60, 0x5A),
            },
            status: StatusColors {
                success: ColorSet {
                    light: Color32::from_rgb(0xB0, 0xE0, 0x9A),
                    normal: Color32::from_rgb(0x80, 0xC0, 0x6A),
                    dark: Color32::from_rgb(0x60, 0xA0, 0x4A),
                    hover: Color32::from_rgb(0x90, 0xD0, 0x7A),
                },
                warning: ColorSet {
                    light: Color32::from_rgb(0xE0, 0xC0, 0x8A),
                    normal: Color32::from_rgb(0xC0, 0xA0, 0x6A),
                    dark: Color32::from_rgb(0xA0, 0x80, 0x4A),
                    hover: Color32::from_rgb(0xD0, 0xB0, 0x7A),
                },
                error: ColorSet {
                    light: Color32::from_rgb(0xE0, 0x90, 0x8A),
                    normal: Color32::from_rgb(0xC0, 0x70, 0x6A),
                    dark: Color32::from_rgb(0xA0, 0x50, 0x4A),
                    hover: Color32::from_rgb(0xD0, 0x80, 0x7A),
                },
                info: ColorSet {
                    light: Color32::from_rgb(0xE0, 0xA0, 0xCA),
                    normal: Color32::from_rgb(0xC0, 0x80, 0xAA),
                    dark: Color32::from_rgb(0xA0, 0x60, 0x8A),
                    hover: Color32::from_rgb(0xD0, 0x90, 0xBA),
                },
            },
            interactive: InteractiveColors {
                button: ColorSet {
                    light: Color32::from_rgb(0xD0, 0x80, 0x7A),
                    normal: Color32::from_rgb(0xC0, 0x70, 0x6A),
                    dark: Color32::from_rgb(0xB0, 0x60, 0x5A),
                    hover: Color32::from_rgb(0xD0, 0x80, 0x7A),
                },
                input: ColorSet {
                    light: Color32::from_rgb(0xF0, 0xD8, 0xD0),
                    normal: Color32::from_rgb(0xE0, 0xC8, 0xC0),
                    dark: Color32::from_rgb(0xD0, 0xB8, 0xB0),
                    hover: Color32::from_rgb(0xF0, 0xD8, 0xD0),
                },
                link: ColorSet {
                    light: Color32::from_rgb(0xE0, 0x80, 0x7A),
                    normal: Color32::from_rgb(0xC0, 0x70, 0x6A),
                    dark: Color32::from_rgb(0xA0, 0x60, 0x5A),
                    hover: Color32::from_rgb(0xE0, 0x80, 0x7A),
                },
                selection: ColorSet {
                    light: Color32::from_rgb(0xC0, 0x60, 0x5A),
                    normal: Color32::from_rgb(0xA0, 0x50, 0x4A),
                    dark: Color32::from_rgb(0x80, 0x40, 0x3A),
                    hover: Color32::from_rgb(0xB0, 0x60, 0x5A),
                },
            },
            custom: HashMap::new(),
        }
    }

    /// Add custom color
    pub fn add_custom_color(&mut self, name: &str, color: Color32) {
        self.custom.insert(name.to_string(), color);
    }

    /// Get custom color
    pub fn get_custom_color(&self, name: &str) -> Option<Color32> {
        self.custom.get(name).copied()
    }

    /// Remove custom color
    pub fn remove_custom_color(&mut self, name: &str) -> Option<Color32> {
        self.custom.remove(name)
    }

    /// Get all custom colors
    pub fn get_custom_colors(&self) -> HashMap<String, Color32> {
        self.custom.clone()
    }

    /// Save palette to file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        info!("Saving color palette to: {:?}", path);

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;

        debug!("Saved color palette successfully");
        Ok(())
    }

    /// Load palette from file
    pub fn load_from_file(&mut self, path: &std::path::Path) -> Result<()> {
        info!("Loading color palette from: {:?}", path);

        if !path.exists() {
            warn!("Color palette file does not exist: {:?}", path);
            return Ok(());
        }

        let content = std::fs::read_to_string(path)?;
        let loaded_palette: ColorPalette = serde_json::from_str(&content)?;

        *self = loaded_palette;

        info!("Loaded color palette successfully");
        Ok(())
    }

    /// Blend with another palette
    pub fn blend_with(&mut self, other: &ColorPalette, factor: f32) {
        fn blend_colors(c1: Color32, c2: Color32, factor: f32) -> Color32 {
            let [r1, g1, b1, a1] = c1.to_array();
            let [r2, g2, b2, a2] = c2.to_array();

            let factor = factor.clamp(0.0, 1.0);
            let inv_factor = 1.0 - factor;

            Color32::from_rgba(
                (r1 as f32 * inv_factor + r2 as f32 * factor) as u8,
                (g1 as f32 * inv_factor + g2 as f32 * factor) as u8,
                (1),
                (a1 as f32 * inv_factor + a2 as f32 * factor) as u8,
            )
        }

        self.primary.light = blend_colors(self.primary.light, other.primary.light, factor);
        self.primary.normal = blend_colors(self.primary.normal, other.primary.normal, factor);
        self.primary.dark = blend_colors(self.primary.dark, other.primary.dark, factor);
        self.primary.hover = blend_colors(self.primary.hover, other.primary.hover, factor);

        // Similar blending for other color sets...
        debug!("Blended color palette with factor: {}", factor);
    }

    /// Validate palette colors for accessibility
    pub fn validate_accessibility(&self) -> AccessibilityReport {
        let mut report = AccessibilityReport::new();

        // Check contrast ratios
        self.check_text_contrast(&mut report);
        self.check_color_blindness(&mut report);
        self.check_color_harmony(&mut report);

        report
    }

    fn check_text_contrast(&self, report: &mut AccessibilityReport) {
        // Check text on surface contrast
        let surface_normal = self.surface.normal;
        let text_normal = self.text.normal;

        let contrast = self.calculate_contrast_ratio(text_normal, surface_normal);
        if contrast < 4.5 {
            report.add_issue(AccessibilityIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::Contrast,
                description: format!("Text on surface contrast too low: {:.2}", contrast),
                suggestion: "Increase text brightness or darken surface".to_string(),
            });
        } else if contrast < 7.0 {
            report.add_issue(AccessibilityIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Contrast,
                description: format!("Text on surface contrast could be improved: {:.2}", contrast),
                suggestion: "Consider increasing contrast for better readability".to_string(),
            });
        }
    }

    fn check_color_blindness(&self, report: &mut AccessibilityReport) {
        // Check if colors are distinguishable for common forms of color blindness
        let issues = self.check_color_blindness_internal(&self.primary.normal, &self.secondary.normal);
        for issue in issues {
            report.add_issue(issue);
        }
    }

    fn check_color_harmony(&self, report: &mut AccessibilityReport) {
        // Check if color combinations are harmonious
        // This is a simplified check - real implementation would use color theory
        if self.is_clashing_colors(&self.primary.normal, &self.error.error.normal) {
            report.add_issue(AccessibilityIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::Harmony,
                description: "Primary and error colors may clash".to_string(),
                suggestion: "Consider using different color combinations".to_string(),
            });
        }
    }

    fn calculate_contrast_ratio(&self, color1: Color32, color2: Color32) -> f32 {
        // Simplified contrast ratio calculation
        let [r1, g1, b1, _] = color1.to_array();
        let [r2, g2, b2, _] = color2.to_array();

        let l1 = (0.299 * r1 as f32 + 0.587 * g1 as f32 + 0.114 * b1 as f32) / 255.0;
        let l2 = (0.299 * r2 as f32 + 0.587 * g2 as f32 + 0.114 * b2 as f32) / 255.0;

        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };

        if lighter == 0.0 {
            return 1.0; // Avoid division by zero
        }

        darker.max(0.05) / lighter.min(1.0 - 0.05)
    }

    fn check_color_blindness_internal(&self, color1: Color32, color2: Color32) -> Vec<AccessibilityIssue> {
        let mut issues = Vec::new();

        // Check for red-green color blindness (most common)
        if self.similar_for_colorblindness(color1, color2, ColorBlindnessType::RedGreen) {
            issues.push(AccessibilityIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::ColorBlindness,
                description: "Colors may be indistinguishable for red-green color blindness".to_string(),
                suggestion: "Use blue or other distinguishing colors".to_string(),
            });
        }

        // Check for blue-yellow color blindness
        if self.similar_for_colorblindness(color1, color2, ColorBlindnessType::BlueYellow) {
            issues.push(AccessibilityIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::ColorBlindness,
                description: "Colors may be indistinguishable for blue-yellow color blindness".to_string(),
                suggestion: "Consider using more distinct color combinations".to_string(),
            });
        }

        issues
    }

    fn similar_for_colorblindness(&self, color1: Color32, color2: Color32, blindness_type: ColorBlindnessType) -> bool {
        let [r1, g1, b1, _] = color1.to_array();
        let [r2, g2, b2, _] = color2.to_array();

        // Simplified simulation of color blindness
        match blindness_type {
            ColorBlindnessType::RedGreen => {
                // Reduce red/green discrimination
                let rg1 = (r1 as f32 + g1 as f32) / 2.0;
                let rg2 = (r2 as f32 + g2 as f32) / 2.0;
                (rg1 - rg2).abs() < 30.0
            }
            ColorBlindnessType::BlueYellow => {
                // Reduce blue/yellow discrimination
                let by1 = (b1 as f32 + (r1 as f32 + g1 as f32) / 2.0) / 2.0;
                let by2 = (b2 as f32 + (r2 as f32 + g2 as f32) / 2.0) / 2.0;
                (by1 - by2).abs() < 30.0
            }
        }
    }

    fn is_clashing_colors(&self, color1: Color32, color2: Color32) -> bool {
        // Simplified clash detection - in reality, this would be much more sophisticated
        let [r1, g1, b1, _] = color1.to_array();
        let [r2, g2, b2, _] = color2.to_array();

        let hue1 = self.calculate_hue(r1, g1, b1);
        let hue2 = self.calculate_hue(r2, g2, b2);

        // If hues are very close and brightness is similar, they may clash
        let hue_diff = (hue1 - hue2).abs().min(360.0 - (hue1 - hue2).abs());
        let brightness_diff = ((r1 + g1 + b1) / 3).abs_diff((r2 + g2 + b2) / 3) as i32) as f32 / 255.0;

        hue_diff < 10.0 && brightness_diff < 0.2
    }

    fn calculate_hue(&self, r: u8, g: u8, b: u8) -> f32 {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        if delta == 0.0 {
            return 0.0;
        }

        let hue = if max == r {
            (g - b) / delta
        } else if max == g {
            2.0 + (b - r) / delta
        } else {
            4.0 + (r - g) / delta
        };

        if hue < 0.0 {
            hue + 360.0
        } else {
            hue
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::dark() // Use dark theme as default
    }
}

#[derive(Debug, Clone)]
pub struct AccessibilityReport {
    issues: Vec<AccessibilityIssue>,
}

#[derive(Debug, Clone)]
pub struct AccessibilityIssue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub description: String,
    pub suggestion: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueCategory {
    Contrast,
    ColorBlindness,
    Harmony,
    Sizing,
    Focus,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColorBlindnessType {
    RedGreen,
    BlueYellow,
}

impl AccessibilityReport {
    pub fn new() -> Self {
        Self { issues: Vec::new() }
    }

    pub fn add_issue(&mut self, issue: AccessibilityIssue) {
        self.issues.push(issue);
    }

    pub fn get_issues(&self) -> &[AccessibilityIssue] {
        &self.issues
    }

    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|issue| matches!(issue.severity, IssueSeverity::Error))
    }

    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|issue| matches!(issue.severity, IssueSeverity::Warning))
    }

    pub fn is_accessible(&self) -> bool {
        !self.has_errors()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_palette_creation() {
        let palette = ColorPalette::new();
        assert_eq!(palette.primary.normal, Color32::from_rgb(0x00, 0x7A, 0xCC));
    }

    #[test]
    fn test_color_palette_schemes() {
        let light = ColorPalette::from_scheme(ColorScheme::Light);
        let dark = ColorPalette::from_scheme(ColorScheme::Dark);
        let high_contrast = ColorPalette::from_scheme(ColorScheme::HighContrast);

        assert_ne!(light.surface.normal, dark.surface.normal);
        assert_eq!(high_contrast.text.normal, Color32::WHITE);
    }

    #[test]
    fn test_custom_colors() {
        let mut palette = ColorPalette::new();

        palette.add_custom_color("accent", Color32::from_rgb(255, 0, 128));
        assert_eq!(palette.get_custom_color("accent"), Some(Color32::from_rgb(255, 0, 128)));

        palette.remove_custom_color("accent");
        assert_eq!(palette.get_custom_color("accent"), None);
    }

    #[test]
    test_color_blending() {
        let mut palette1 = ColorPalette::light();
        let palette2 = ColorPalette::dark();

        let original_primary = palette1.primary.normal;
        palette1.blend_with(&palette2, 0.5);

        // Should be different after blending
        assert_ne!(palette1.primary.normal, original_primary);
    }

    #[test]
    fn test_accessibility_validation() {
        let palette = ColorPalette::new();
        let report = palette.validate_accessibility();

        // Should not have errors for default palette
        assert!(!report.has_errors());
    }

    #[test]
    fn test_contrast_calculation() {
        let palette = ColorPalette::new();
        let text = palette.text.normal;
        let surface = palette.surface.normal;

        let contrast = palette.calculate_contrast_ratio(text, surface);

        // Should be reasonable for default theme
        assert!(contrast > 1.0);
        assert!(contrast < 20.0);
    }
}