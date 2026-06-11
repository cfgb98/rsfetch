//! Presentation layer: turns raw [`crate::info`] data into display strings.

pub mod bytes;
pub mod units;

use crate::field::Field;
use crate::info::{DiskInfo, MemoryInfo, SystemInfo};
use crate::theme::Theme;
use bytes::humanize_bytes;
use units::humanize_uptime;

/// Build the `(label, value)` rows for the requested fields, in order.
///
/// Optional fields with no data (e.g. an unknown hostname, or swap when there
/// is none) are skipped rather than shown as "unknown". Disks expand to one row
/// per selected mount: just the root filesystem unless `show_all_disks` is set.
pub fn build_rows(
    info: &SystemInfo,
    fields: &[Field],
    show_all_disks: bool,
) -> Vec<(String, String)> {
    let mut rows: Vec<(String, String)> = Vec::new();

    for &field in fields {
        match field {
            Field::Host => {
                if let Some(host) = &info.host_name {
                    rows.push(("Host".into(), host.clone()));
                }
            }
            Field::Os => {
                if let Some(os) = os_value(info) {
                    rows.push(("OS".into(), os));
                }
            }
            Field::Kernel => {
                if let Some(kernel) = &info.kernel_version {
                    rows.push(("Kernel".into(), kernel.clone()));
                }
            }
            Field::Shell => {
                if let Some(shell) = &info.shell {
                    rows.push(("Shell".into(), shell.clone()));
                }
            }
            Field::Uptime => rows.push(("Uptime".into(), humanize_uptime(info.uptime_secs))),
            Field::Cpu => rows.push(("CPU".into(), cpu_value(info))),
            Field::Memory => rows.push(("Memory".into(), mem_value(&info.memory))),
            Field::Swap => {
                if info.memory.swap_total > 0 {
                    rows.push(("Swap".into(), swap_value(&info.memory)));
                }
            }
            Field::Disks => {
                for disk in select_disks(&info.disks, show_all_disks) {
                    rows.push((format!("Disk ({})", disk.mount_point), disk_value(disk)));
                }
            }
        }
    }

    rows
}

fn os_value(info: &SystemInfo) -> Option<String> {
    match (&info.os_name, &info.os_version) {
        (Some(name), Some(version)) => Some(format!("{name} {version}")),
        (Some(name), None) => Some(name.clone()),
        _ => None,
    }
}

fn cpu_value(info: &SystemInfo) -> String {
    match &info.cpu_model {
        Some(model) => format!("{model} ({} cores)", info.cpu_count),
        None => format!("{} cores", info.cpu_count),
    }
}

fn mem_value(mem: &MemoryInfo) -> String {
    let percent = if mem.total > 0 {
        // u128 keeps the `* 100` from overflowing for any plausible memory size.
        (mem.used as u128 * 100 / mem.total as u128) as u64
    } else {
        0
    };
    format!(
        "{} / {} ({percent}%)",
        humanize_bytes(mem.used),
        humanize_bytes(mem.total)
    )
}

fn swap_value(mem: &MemoryInfo) -> String {
    format!(
        "{} / {}",
        humanize_bytes(mem.swap_used),
        humanize_bytes(mem.swap_total)
    )
}

fn disk_value(disk: &DiskInfo) -> String {
    let used = disk.total.saturating_sub(disk.available);
    format!("{} / {}", humanize_bytes(used), humanize_bytes(disk.total))
}

/// Select which disks to display: the root mount only, or all of them.
fn select_disks(disks: &[DiskInfo], show_all: bool) -> impl Iterator<Item = &DiskInfo> {
    disks
        .iter()
        .filter(move |disk| show_all || disk.mount_point == "/")
}

/// Render `(label, value)` rows into aligned `"label<pad><sep> value"` lines.
///
/// Labels are padded to the widest label so the separators — and therefore the
/// values — line up. Padding is always computed from the *plain* label width;
/// color is applied afterwards so ANSI escape bytes never affect alignment.
pub fn render_block(
    rows: &[(String, String)],
    separator: &str,
    theme: &Theme,
    use_color: bool,
) -> Vec<String> {
    let width = rows
        .iter()
        .map(|(label, _)| label.chars().count())
        .max()
        .unwrap_or(0);

    rows.iter()
        .map(|(label, value)| {
            let pad = " ".repeat(width - label.chars().count());
            if use_color {
                format!(
                    "{}{pad}{} {}",
                    theme.label.style(label),
                    theme.separator.style(separator),
                    theme.value.style(value),
                )
            } else {
                format!("{label}{pad}{separator} {value}")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(l, v)| (l.to_string(), v.to_string()))
            .collect()
    }

    fn sample() -> SystemInfo {
        SystemInfo {
            host_name: Some("testhost".into()),
            os_name: Some("TestOS".into()),
            os_version: Some("1.0".into()),
            kernel_version: Some("5.0".into()),
            shell: Some("/bin/zsh".into()),
            uptime_secs: 3661,
            cpu_model: Some("TestCPU".into()),
            cpu_count: 4,
            memory: MemoryInfo {
                total: 16 * 1024 * 1024 * 1024,
                used: 8 * 1024 * 1024 * 1024,
                swap_total: 0,
                swap_used: 0,
            },
            disks: vec![
                DiskInfo {
                    mount_point: "/".into(),
                    total: 100 * 1024 * 1024 * 1024,
                    available: 40 * 1024 * 1024 * 1024,
                    file_system: "ext4".into(),
                },
                DiskInfo {
                    mount_point: "/mnt/c".into(),
                    total: 200,
                    available: 100,
                    file_system: "9p".into(),
                },
            ],
        }
    }

    #[test]
    fn mem_value_formats_used_total_and_percent() {
        let mem = MemoryInfo {
            total: 16 * 1024 * 1024 * 1024,
            used: 8 * 1024 * 1024 * 1024,
            swap_total: 0,
            swap_used: 0,
        };
        assert_eq!(mem_value(&mem), "8.0 GiB / 16.0 GiB (50%)");
    }

    #[test]
    fn mem_value_handles_zero_total_without_dividing_by_zero() {
        assert_eq!(mem_value(&MemoryInfo::default()), "0 B / 0 B (0%)");
    }

    #[test]
    fn build_rows_respects_field_order_and_values() {
        let out = build_rows(&sample(), &[Field::Cpu, Field::Host], false);
        assert_eq!(
            out,
            rows(&[("CPU", "TestCPU (4 cores)"), ("Host", "testhost")])
        );
    }

    #[test]
    fn build_rows_skips_absent_optional_fields() {
        let mut info = sample();
        info.host_name = None;
        let out = build_rows(&info, &[Field::Host, Field::Uptime], false);
        assert_eq!(out, rows(&[("Uptime", "1h 1m")]));
    }

    #[test]
    fn build_rows_hides_swap_when_absent() {
        let out = build_rows(&sample(), &[Field::Swap], false);
        assert!(out.is_empty());
    }

    #[test]
    fn build_rows_shows_root_disk_only_by_default() {
        let out = build_rows(&sample(), &[Field::Disks], false);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0, "Disk (/)");
    }

    #[test]
    fn build_rows_shows_all_disks_when_requested() {
        let out = build_rows(&sample(), &[Field::Disks], true);
        let labels: Vec<&str> = out.iter().map(|(l, _)| l.as_str()).collect();
        assert_eq!(labels, vec!["Disk (/)", "Disk (/mnt/c)"]);
    }

    fn theme() -> Theme {
        Theme::by_name("default").unwrap()
    }

    /// Remove ANSI SGR escape sequences so visible width can be asserted.
    fn strip_ansi(s: &str) -> String {
        let mut out = String::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\u{1b}' {
                for c2 in chars.by_ref() {
                    if c2 == 'm' {
                        break;
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    #[test]
    fn empty_input_yields_no_lines() {
        assert!(render_block(&[], ":", &theme(), false).is_empty());
    }

    #[test]
    fn pads_labels_so_separators_align() {
        let out = render_block(&rows(&[("OS", "Fedora"), ("Kernel", "6.6")]), ":", &theme(), false);
        assert_eq!(out, vec!["OS    : Fedora", "Kernel: 6.6"]);
    }

    #[test]
    fn single_row_has_no_padding() {
        let out = render_block(&rows(&[("CPU", "i9")]), ":", &theme(), false);
        assert_eq!(out, vec!["CPU: i9"]);
    }

    #[test]
    fn colored_lines_preserve_visible_alignment() {
        let input = rows(&[("OS", "Fedora"), ("Kernel", "6.6")]);
        let out = render_block(&input, ":", &theme(), true);
        // Color is actually applied...
        assert!(out[0].contains('\u{1b}'), "expected ANSI escapes when colored");
        // ...but stripping it back reveals the same aligned layout as plain.
        let visible: Vec<String> = out.iter().map(|l| strip_ansi(l)).collect();
        assert_eq!(visible, vec!["OS    : Fedora", "Kernel: 6.6"]);
    }
}
