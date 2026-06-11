//! rsfetch — a neofetch-style system information tool.
//!
//! The crate is split into layers, each returning plain data to the next so
//! every layer can be tested independently:
//!
//! - [`info`] collects raw [`info::SystemInfo`] (bytes as `u64`, never strings).
//! - [`format`] turns that raw data into human-readable display strings.

pub mod cli;
pub mod config;
pub mod field;
pub mod format;
pub mod info;
pub mod logo;
pub mod render;
pub mod theme;
