//! Command-line interface (clap).
//!
//! Every override is optional so that "not passed on the CLI" is distinct from
//! a value, letting [`crate::config::Config::resolve`] apply the precedence
//! CLI > config file > defaults.

use std::path::PathBuf;

use clap::Parser;

use crate::field::Field;
use crate::theme::ColorMode;

#[derive(Parser, Debug, Default)]
#[command(name = "rsfetch", version, about = "A neofetch-style system information tool")]
pub struct Cli {
    /// Path to a config file (defaults to $XDG_CONFIG_HOME/rsfetch/config.toml).
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Comma-separated fields to display, e.g. `host,os,cpu`.
    #[arg(long, value_delimiter = ',', value_name = "LIST")]
    pub fields: Option<Vec<Field>>,

    /// String placed between each label and its value.
    #[arg(long, value_name = "STR")]
    pub separator: Option<String>,

    /// Show every mounted disk instead of just the root filesystem.
    #[arg(long)]
    pub all_disks: bool,

    /// When to use color: auto, always, or never.
    #[arg(long, value_name = "WHEN")]
    pub color: Option<ColorMode>,

    /// Color theme name (e.g. `default`, `mono`).
    #[arg(long, value_name = "NAME")]
    pub theme: Option<String>,

    /// Logo to show: `auto`, `none`, or a name (e.g. `arch`, `fedora`, `wsl`).
    #[arg(long, value_name = "NAME")]
    pub logo: Option<String>,
}
