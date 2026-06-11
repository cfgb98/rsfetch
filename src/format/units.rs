//! Human-readable durations (uptime).

const SECS_PER_MIN: u64 = 60;
const SECS_PER_HOUR: u64 = 60 * SECS_PER_MIN;
const SECS_PER_DAY: u64 = 24 * SECS_PER_HOUR;

/// Format a duration in seconds as a compact uptime string.
///
/// Shows the most significant units only: seconds are dropped once the
/// duration reaches an hour (`"1h 1m"`), and zero-valued trailing units are
/// omitted (`3600` → `"1h"`, `90` → `"1m 30s"`).
pub fn humanize_uptime(secs: u64) -> String {
    let days = secs / SECS_PER_DAY;
    let hours = (secs % SECS_PER_DAY) / SECS_PER_HOUR;
    let mins = (secs % SECS_PER_HOUR) / SECS_PER_MIN;
    let seconds = secs % SECS_PER_MIN;

    // Pick the parts to consider based on the most significant non-zero unit,
    // dropping seconds once we reach an hour.
    let parts: &[(u64, &str)] = if days > 0 {
        &[(days, "d"), (hours, "h"), (mins, "m")]
    } else if hours > 0 {
        &[(hours, "h"), (mins, "m")]
    } else if mins > 0 {
        &[(mins, "m"), (seconds, "s")]
    } else {
        &[(seconds, "s")]
    };

    let rendered: Vec<String> = parts
        .iter()
        .filter(|(value, _)| *value > 0)
        .map(|(value, unit)| format!("{value}{unit}"))
        .collect();

    if rendered.is_empty() {
        "0s".to_string()
    } else {
        rendered.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(humanize_uptime(0), "0s");
    }

    #[test]
    fn seconds_only() {
        assert_eq!(humanize_uptime(30), "30s");
    }

    #[test]
    fn minutes_and_seconds() {
        assert_eq!(humanize_uptime(90), "1m 30s");
    }

    #[test]
    fn whole_hour_drops_zero_minutes() {
        assert_eq!(humanize_uptime(3600), "1h");
    }

    #[test]
    fn hours_drop_seconds() {
        assert_eq!(humanize_uptime(3661), "1h 1m");
    }

    #[test]
    fn days_and_hours() {
        // 90000s = 25h = 1d 1h 0m
        assert_eq!(humanize_uptime(90_000), "1d 1h");
    }

    #[test]
    fn exact_day_drops_zero_hours_and_minutes() {
        assert_eq!(humanize_uptime(86_400), "1d");
    }

    #[test]
    fn multiple_days_with_hours_and_minutes_drops_seconds() {
        // 90061s = 1d 1h 1m 1s — seconds dropped once days/hours are present.
        assert_eq!(humanize_uptime(90_061), "1d 1h 1m");
    }
}
