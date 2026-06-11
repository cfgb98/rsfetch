//! Color themes and the color-enablement decision.
//!
//! A [`Theme`] is a set of [`owo_colors::Style`]s applied to the label,
//! separator, and value of each row. [`ColorMode`] plus [`should_use_color`]
//! decide whether any color is emitted at all.

use std::fmt;
use std::str::FromStr;

use owo_colors::Style;
use serde::Deserialize;

/// When to emit ANSI color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorMode {
    /// Color when writing to a terminal that supports it.
    #[default]
    Auto,
    /// Always color.
    Always,
    /// Never color.
    Never,
}

/// Error returned when a string does not name a known [`ColorMode`].
#[derive(Debug, PartialEq, Eq)]
pub struct ParseColorModeError(pub String);

impl fmt::Display for ParseColorModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid color mode '{}' (expected auto, always, or never)", self.0)
    }
}

impl std::error::Error for ParseColorModeError {}

impl FromStr for ColorMode {
    type Err = ParseColorModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "auto" => Ok(ColorMode::Auto),
            "always" => Ok(ColorMode::Always),
            "never" => Ok(ColorMode::Never),
            _ => Err(ParseColorModeError(s.to_string())),
        }
    }
}

/// Decide whether to emit color, given the mode and whether the output stream is
/// a color-capable terminal.
pub fn should_use_color(mode: ColorMode, terminal_supports_color: bool) -> bool {
    match mode {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => terminal_supports_color,
    }
}

/// Styles applied to each component of a rendered row, plus the logo.
#[derive(Debug, Clone)]
pub struct Theme {
    pub label: Style,
    pub separator: Style,
    pub value: Style,
    pub logo: Style,
}

impl Theme {
    /// Look up a theme by name. Returns `None` for unknown names.
    pub fn by_name(name: &str) -> Option<Theme> {
        match name {
            "default" => Some(Theme {
                label: Style::new().cyan().bold(),
                separator: Style::new().dimmed(),
                value: Style::new(),
                logo: Style::new().green().bold(),
            }),
            "mono" => Some(Theme {
                label: Style::new().bold(),
                separator: Style::new(),
                value: Style::new(),
                logo: Style::new().bold(),
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_mode_parses() {
        assert_eq!("auto".parse(), Ok(ColorMode::Auto));
        assert_eq!("ALWAYS".parse(), Ok(ColorMode::Always));
        assert_eq!("never".parse(), Ok(ColorMode::Never));
        assert!("rainbow".parse::<ColorMode>().is_err());
    }

    #[test]
    fn always_forces_color_regardless_of_terminal() {
        assert!(should_use_color(ColorMode::Always, false));
        assert!(should_use_color(ColorMode::Always, true));
    }

    #[test]
    fn never_disables_color_regardless_of_terminal() {
        assert!(!should_use_color(ColorMode::Never, true));
        assert!(!should_use_color(ColorMode::Never, false));
    }

    #[test]
    fn auto_follows_terminal_support() {
        assert!(should_use_color(ColorMode::Auto, true));
        assert!(!should_use_color(ColorMode::Auto, false));
    }

    #[test]
    fn known_themes_resolve_and_unknown_does_not() {
        assert!(Theme::by_name("default").is_some());
        assert!(Theme::by_name("mono").is_some());
        assert!(Theme::by_name("nonsense").is_none());
    }
}
