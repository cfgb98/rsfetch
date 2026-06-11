//! Configuration: the on-disk [`ConfigFile`], the resolved [`Config`] the rest
//! of the program consumes, and the precedence rules that merge them.
//!
//! Precedence, highest first: **CLI flags > config file > built-in defaults.**

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::cli::Cli;
use crate::field::Field;
use crate::theme::ColorMode;

/// A config file as read from disk. Every field is optional so a partial file
/// only overrides what it specifies.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ConfigFile {
    pub fields: Option<Vec<Field>>,
    pub separator: Option<String>,
    pub all_disks: Option<bool>,
    pub color: Option<ColorMode>,
    pub theme: Option<String>,
    pub logo: Option<String>,
}

impl ConfigFile {
    /// Parse a config file from TOML text.
    pub fn from_toml(text: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(text)
    }
}

/// Fully-resolved configuration, with every value present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub fields: Vec<Field>,
    pub separator: String,
    pub show_all_disks: bool,
    pub color: ColorMode,
    pub theme: String,
    pub logo: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            fields: Field::default_order(),
            separator: ":".to_string(),
            show_all_disks: false,
            color: ColorMode::default(),
            theme: "default".to_string(),
            logo: "auto".to_string(),
        }
    }
}

impl Config {
    /// Resolve the final configuration by layering: defaults, then the config
    /// file's set values, then the CLI's set values.
    pub fn resolve(file: ConfigFile, cli: &Cli) -> Config {
        let mut config = Config::default();

        // Layer 2: config file overrides defaults.
        if let Some(fields) = file.fields {
            config.fields = fields;
        }
        if let Some(separator) = file.separator {
            config.separator = separator;
        }
        if let Some(all_disks) = file.all_disks {
            config.show_all_disks = all_disks;
        }

        if let Some(color) = file.color {
            config.color = color;
        }
        if let Some(theme) = file.theme {
            config.theme = theme;
        }
        if let Some(logo) = file.logo {
            config.logo = logo;
        }

        // Layer 3: CLI overrides the config file.
        if let Some(fields) = &cli.fields {
            config.fields = fields.clone();
        }
        if let Some(separator) = &cli.separator {
            config.separator = separator.clone();
        }
        if cli.all_disks {
            config.show_all_disks = true;
        }
        if let Some(color) = cli.color {
            config.color = color;
        }
        if let Some(theme) = &cli.theme {
            config.theme = theme.clone();
        }
        if let Some(logo) = &cli.logo {
            config.logo = logo.clone();
        }

        config
    }
}

/// Errors that can occur while loading a config file.
#[derive(Debug)]
pub enum ConfigError {
    /// An explicitly-requested config file could not be read.
    Read { path: PathBuf, source: std::io::Error },
    /// The config file was found but could not be parsed.
    Parse { path: PathBuf, source: toml::de::Error },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Read { path, source } => {
                write!(f, "could not read config '{}': {source}", path.display())
            }
            ConfigError::Parse { path, source } => {
                write!(f, "invalid config '{}': {source}", path.display())
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// The default config path: `$XDG_CONFIG_HOME/rsfetch/config.toml`, falling back
/// to `$HOME/.config/rsfetch/config.toml`. Returns `None` if neither is set.
pub fn default_config_path() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return Some(Path::new(&xdg).join("rsfetch/config.toml"));
        }
    }
    std::env::var("HOME")
        .ok()
        .filter(|home| !home.is_empty())
        .map(|home| Path::new(&home).join(".config/rsfetch/config.toml"))
}

/// Load a config file. An explicit path that cannot be read is an error; the
/// default path simply being absent yields an empty (all-default) config.
pub fn load(explicit: Option<&Path>) -> Result<ConfigFile, ConfigError> {
    let (path, required) = match explicit {
        Some(p) => (p.to_path_buf(), true),
        None => match default_config_path() {
            Some(p) => (p, false),
            None => return Ok(ConfigFile::default()),
        },
    };

    match fs::read_to_string(&path) {
        Ok(text) => ConfigFile::from_toml(&text).map_err(|source| ConfigError::Parse { path, source }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound && !required => {
            Ok(ConfigFile::default())
        }
        Err(source) => Err(ConfigError::Read { path, source }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_when_nothing_set() {
        let cfg = Config::resolve(ConfigFile::default(), &Cli::default());
        assert_eq!(cfg, Config::default());
    }

    #[test]
    fn config_file_overrides_defaults() {
        let file = ConfigFile {
            fields: Some(vec![Field::Host, Field::Cpu]),
            separator: Some(" =>".to_string()),
            all_disks: Some(true),
            color: Some(ColorMode::Never),
            theme: Some("mono".to_string()),
            logo: Some("arch".to_string()),
        };
        let cfg = Config::resolve(file, &Cli::default());
        assert_eq!(cfg.fields, vec![Field::Host, Field::Cpu]);
        assert_eq!(cfg.separator, " =>");
        assert!(cfg.show_all_disks);
        assert_eq!(cfg.color, ColorMode::Never);
        assert_eq!(cfg.theme, "mono");
        assert_eq!(cfg.logo, "arch");
    }

    #[test]
    fn cli_overrides_config_file() {
        let file = ConfigFile {
            fields: Some(vec![Field::Host]),
            separator: Some("X".to_string()),
            all_disks: Some(false),
            color: Some(ColorMode::Never),
            theme: Some("mono".to_string()),
            logo: Some("arch".to_string()),
        };
        let cli = Cli {
            fields: Some(vec![Field::Cpu, Field::Memory]),
            separator: Some(":".to_string()),
            all_disks: true,
            color: Some(ColorMode::Always),
            theme: Some("default".to_string()),
            logo: Some("none".to_string()),
            ..Cli::default()
        };
        let cfg = Config::resolve(file, &cli);
        assert_eq!(cfg.fields, vec![Field::Cpu, Field::Memory]);
        assert_eq!(cfg.separator, ":");
        assert!(cfg.show_all_disks, "CLI --all-disks should force-enable");
        assert_eq!(cfg.color, ColorMode::Always);
        assert_eq!(cfg.theme, "default");
        assert_eq!(cfg.logo, "none");
    }

    #[test]
    fn empty_toml_parses_to_all_none() {
        let file = ConfigFile::from_toml("").unwrap();
        assert!(file.fields.is_none());
        assert!(file.separator.is_none());
        assert!(file.all_disks.is_none());
    }

    #[test]
    fn parses_partial_toml() {
        let file = ConfigFile::from_toml(r#"separator = " >""#).unwrap();
        assert_eq!(file.separator.as_deref(), Some(" >"));
        assert!(file.fields.is_none());
    }

    #[test]
    fn malformed_toml_is_error() {
        assert!(ConfigFile::from_toml("fields = [").is_err());
    }
}
