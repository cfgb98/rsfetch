//! ASCII logos and the logic that picks one for the running system.

pub mod art;

/// A parsed ASCII logo: a list of lines with surrounding blank lines trimmed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Logo {
    lines: Vec<String>,
}

impl Logo {
    /// Parse raw art, dropping leading and trailing blank lines.
    pub fn from_art(art: &str) -> Logo {
        let mut lines: Vec<String> = art.lines().map(|line| line.to_string()).collect();
        while lines.first().is_some_and(|line| line.trim().is_empty()) {
            lines.remove(0);
        }
        while lines.last().is_some_and(|line| line.trim().is_empty()) {
            lines.pop();
        }
        Logo { lines }
    }

    /// The logo's lines, top to bottom.
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Number of lines.
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Visible width: the widest line, in characters.
    pub fn width(&self) -> usize {
        self.lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0)
    }
}

/// Pick the art for the running system.
///
/// WSL is detected from the kernel string and takes precedence (it is the most
/// specific signal); otherwise the distribution is matched from the OS name,
/// falling back to a generic logo.
pub fn detect(os_name: Option<&str>, kernel: Option<&str>) -> &'static str {
    let kernel_lc = kernel.unwrap_or_default().to_ascii_lowercase();
    if kernel_lc.contains("microsoft") || kernel_lc.contains("wsl") {
        return art::WSL;
    }

    let os_lc = os_name.unwrap_or_default().to_ascii_lowercase();
    if os_lc.contains("arch") {
        art::ARCH
    } else if os_lc.contains("debian") {
        art::DEBIAN
    } else if os_lc.contains("fedora") {
        art::FEDORA
    } else if os_lc.contains("ubuntu") {
        art::UBUNTU
    } else if os_lc.contains("darwin") {
        art::MAC
    } else {
        art::GENERIC
    }
}

/// Error returned when an explicit logo name is not recognized.
#[derive(Debug, PartialEq, Eq)]
pub struct UnknownLogo(pub String);

impl std::fmt::Display for UnknownLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown logo '{}'", self.0)
    }
}

impl std::error::Error for UnknownLogo {}

/// Resolve a configured logo name into a [`Logo`], if any.
///
/// `"none"`/`"off"` disables the logo (`Ok(None)`); `"auto"` detects from the
/// system; any other name is looked up explicitly and errors if unknown.
pub fn resolve(
    name: &str,
    os_name: Option<&str>,
    kernel: Option<&str>,
) -> Result<Option<Logo>, UnknownLogo> {
    match name.to_ascii_lowercase().as_str() {
        "none" | "off" => Ok(None),
        "auto" => Ok(Some(Logo::from_art(detect(os_name, kernel)))),
        other => by_name(other)
            .map(|art| Some(Logo::from_art(art)))
            .ok_or_else(|| UnknownLogo(name.to_string())),
    }
}

/// Look up art by explicit logo name.
pub fn by_name(name: &str) -> Option<&'static str> {
    match name.to_ascii_lowercase().as_str() {
        "arch" => Some(art::ARCH),
        "debian" => Some(art::DEBIAN),
        "fedora" => Some(art::FEDORA),
        "ubuntu" => Some(art::UBUNTU),
        "wsl" => Some(art::WSL),
        "generic" => Some(art::GENERIC),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_distros_from_os_name() {
        assert_eq!(detect(Some("Arch Linux"), Some("6.6")), art::ARCH);
        assert_eq!(detect(Some("Ubuntu"), Some("5.15")), art::UBUNTU);
        assert_eq!(detect(Some("Debian GNU/Linux"), Some("6.1")), art::DEBIAN);
        assert_eq!(detect(Some("Fedora Linux"), Some("6.6")), art::FEDORA);
    }

    #[test]
    fn wsl_kernel_takes_precedence_over_distro() {
        assert_eq!(
            detect(Some("Fedora Linux"), Some("6.6.114.1-microsoft-standard-WSL2")),
            art::WSL
        );
    }

    #[test]
    fn unknown_os_falls_back_to_generic() {
        assert_eq!(detect(Some("Plan 9"), Some("4")), art::GENERIC);
        assert_eq!(detect(None, None), art::GENERIC);
    }

    #[test]
    fn from_art_trims_blank_lines_and_measures() {
        let logo = Logo::from_art("\n\nabc\nde\n\n");
        assert_eq!(logo.lines(), &["abc".to_string(), "de".to_string()]);
        assert_eq!(logo.height(), 2);
        assert_eq!(logo.width(), 3);
    }

    #[test]
    fn resolve_none_disables_logo() {
        assert_eq!(resolve("none", Some("Arch"), None), Ok(None));
        assert_eq!(resolve("off", Some("Arch"), None), Ok(None));
    }

    #[test]
    fn resolve_auto_detects_from_system() {
        assert_eq!(
            resolve("auto", Some("Arch Linux"), Some("6.6")),
            Ok(Some(Logo::from_art(art::ARCH)))
        );
    }

    #[test]
    fn resolve_named_logo() {
        assert_eq!(
            resolve("fedora", None, None),
            Ok(Some(Logo::from_art(art::FEDORA)))
        );
    }

    #[test]
    fn resolve_unknown_logo_errors() {
        assert_eq!(resolve("bogus", None, None), Err(UnknownLogo("bogus".into())));
    }
}
