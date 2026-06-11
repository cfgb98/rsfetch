//! Two-column layout: the logo on the left, the info block on the right.

use owo_colors::Style;

use crate::logo::Logo;

/// Place `info_lines` to the right of `logo`, separated by `gutter` spaces.
///
/// The info column is aligned to a fixed left offset of `logo.width() + gutter`.
/// Padding is computed from the logo's *visible* width, so coloring the logo
/// never shifts the info column. When one column is taller, the other is padded
/// (logo side) or its lines emitted alone (info exhausted) so no trailing
/// whitespace is produced past the logo.
pub fn combine_columns(
    logo: &Logo,
    info_lines: &[String],
    gutter: usize,
    logo_style: &Style,
    use_color: bool,
) -> Vec<String> {
    let logo_width = logo.width();
    let rows = logo.height().max(info_lines.len());
    let mut out = Vec::with_capacity(rows);

    for i in 0..rows {
        let left = logo.lines().get(i).map(String::as_str).unwrap_or("");
        let right = info_lines.get(i).map(String::as_str).unwrap_or("");

        let styled_left = if use_color && !left.is_empty() {
            logo_style.style(left).to_string()
        } else {
            left.to_string()
        };

        if right.is_empty() {
            // Info column exhausted: emit the logo line alone, no trailing pad.
            out.push(styled_left);
        } else {
            let pad = " ".repeat(logo_width - left.chars().count() + gutter);
            out.push(format!("{styled_left}{pad}{right}"));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn style() -> Style {
        Style::new().green()
    }

    fn lines(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

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
    fn info_taller_than_logo_pads_logo_column() {
        let logo = Logo::from_art("ab\nc");
        let out = combine_columns(&logo, &lines(&["X", "Y", "Z"]), 1, &style(), false);
        assert_eq!(out, lines(&["ab X", "c  Y", "   Z"]));
    }

    #[test]
    fn logo_taller_than_info_emits_remaining_logo_without_trailing_space() {
        let logo = Logo::from_art("ab\nc\nd");
        let out = combine_columns(&logo, &lines(&["X"]), 1, &style(), false);
        assert_eq!(out, lines(&["ab X", "c", "d"]));
    }

    #[test]
    fn colored_logo_keeps_info_column_aligned() {
        let logo = Logo::from_art("ab\nc");
        let out = combine_columns(&logo, &lines(&["X", "Y"]), 1, &style(), true);
        assert!(out[0].contains('\u{1b}'), "expected color on the logo");
        let visible: Vec<String> = out.iter().map(|l| strip_ansi(l)).collect();
        assert_eq!(visible, lines(&["ab X", "c  Y"]));
    }
}
