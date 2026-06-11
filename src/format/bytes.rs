//! Human-readable byte sizes using binary (IEC) units.

const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
const STEP: f64 = 1024.0;

/// Format a byte count as a human-readable string using binary units.
///
/// Values below 1024 are shown as a whole number of bytes (`"512 B"`); larger
/// values are scaled to the largest fitting unit with one decimal place
/// (`"1.5 KiB"`, `"31.2 GiB"`).
pub fn humanize_bytes(bytes: u64) -> String {
    if bytes < STEP as u64 {
        return format!("{bytes} B");
    }

    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= STEP && unit < UNITS.len() - 1 {
        value /= STEP;
        unit += 1;
    }
    // Guard the rounding boundary: a value like 1023.99 KiB would render as
    // "1024.0 KiB", a unit that should never appear — promote it to the next.
    if unit < UNITS.len() - 1 && (value * 10.0).round() / 10.0 >= STEP {
        value /= STEP;
        unit += 1;
    }
    format!("{value:.1} {}", UNITS[unit])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_bytes() {
        assert_eq!(humanize_bytes(0), "0 B");
    }

    #[test]
    fn bytes_below_one_kib_are_whole() {
        assert_eq!(humanize_bytes(512), "512 B");
        assert_eq!(humanize_bytes(1023), "1023 B");
    }

    #[test]
    fn exact_kib_boundary() {
        assert_eq!(humanize_bytes(1024), "1.0 KiB");
    }

    #[test]
    fn fractional_kib() {
        assert_eq!(humanize_bytes(1536), "1.5 KiB");
    }

    #[test]
    fn rounds_up_into_next_unit_at_boundary() {
        // 1 byte below 1 MiB rounds to a full MiB, not "1024.0 KiB".
        assert_eq!(humanize_bytes(1024 * 1024 - 1), "1.0 MiB");
    }

    #[test]
    fn gibibytes() {
        // 33,516,687,360 bytes ≈ 31.2 GiB (the test box's RAM).
        assert_eq!(humanize_bytes(33_516_687_360), "31.2 GiB");
    }

    #[test]
    fn tebibytes() {
        assert_eq!(humanize_bytes(2 * 1024 * 1024 * 1024 * 1024), "2.0 TiB");
    }
}
