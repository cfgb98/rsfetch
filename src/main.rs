use std::process::ExitCode;

use clap::Parser;
use supports_color::Stream;

use rsfetch::cli::Cli;
use rsfetch::config::{self, Config};
use rsfetch::format::{build_rows, render_block};
use rsfetch::info;
use rsfetch::logo;
use rsfetch::render::combine_columns;
use rsfetch::theme::{should_use_color, Theme};

/// Spaces between the logo column and the info column.
const GUTTER: usize = 3;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let file = match config::load(cli.config.as_deref()) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("rsfetch: {err}");
            return ExitCode::FAILURE;
        }
    };
    let config = Config::resolve(file, &cli);

    let Some(theme) = Theme::by_name(&config.theme) else {
        eprintln!("rsfetch: unknown theme '{}'", config.theme);
        return ExitCode::FAILURE;
    };
    let use_color = should_use_color(config.color, supports_color::on(Stream::Stdout).is_some());

    let info = info::collect();

    let logo = match logo::resolve(&config.logo, info.os_name.as_deref(), info.kernel_version.as_deref())
    {
        Ok(logo) => logo,
        Err(err) => {
            eprintln!("rsfetch: {err}");
            return ExitCode::FAILURE;
        }
    };

    let rows = build_rows(&info, &config.fields, config.show_all_disks);
    let info_lines = render_block(&rows, &config.separator, &theme, use_color);

    let output = match &logo {
        Some(logo) => combine_columns(logo, &info_lines, GUTTER, &theme.logo, use_color),
        None => info_lines,
    };
    for line in output {
        println!("{line}");
    }

    ExitCode::SUCCESS
}
