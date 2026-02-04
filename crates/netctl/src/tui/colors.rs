// Color theme - Coral-Terracotta Orange (Pantone 7416 C inspired)
// Matching guestkit's beautiful color scheme

use ratatui::style::Color;

/// Primary coral orange (Pantone 7416 C)
pub const ORANGE: Color = Color::Rgb(222, 115, 86);

/// Darker terracotta for borders and accents
pub const DARK_ORANGE: Color = Color::Rgb(180, 85, 60);

/// Lighter coral for highlights
pub const LIGHT_ORANGE: Color = Color::Rgb(255, 145, 115);

/// Background color
pub const BG_COLOR: Color = Color::Reset;

/// Softer white for text
pub const TEXT_COLOR: Color = Color::Rgb(220, 220, 220);

/// Border color (uses dark orange)
pub const BORDER_COLOR: Color = DARK_ORANGE;

/// Brighter green for success states
pub const SUCCESS_COLOR: Color = Color::Rgb(50, 205, 50);

/// Deeper yellow for warnings
pub const WARNING_COLOR: Color = Color::Rgb(255, 200, 0);

/// Deep red for errors
pub const ERROR_COLOR: Color = Color::Rgb(220, 50, 47);

/// Soft blue for info
pub const INFO_COLOR: Color = Color::Rgb(100, 150, 255);
