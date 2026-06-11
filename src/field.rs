//! The set of information fields rsfetch can display, and their identifiers.
//!
//! [`Field`] is the single source of truth for which fields exist and the order
//! they appear in by default. It parses from lowercase identifiers both from
//! the CLI (via [`FromStr`]) and from config files (via `serde`).

use std::fmt;
use std::str::FromStr;

use serde::Deserialize;

/// A single line of information in the output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Field {
    Host,
    Os,
    Kernel,
    Shell,
    Uptime,
    Cpu,
    Memory,
    Swap,
    Disks,
}

impl Field {
    /// The canonical lowercase identifier for this field.
    pub fn id(self) -> &'static str {
        match self {
            Field::Host => "host",
            Field::Os => "os",
            Field::Kernel => "kernel",
            Field::Shell => "shell",
            Field::Uptime => "uptime",
            Field::Cpu => "cpu",
            Field::Memory => "memory",
            Field::Swap => "swap",
            Field::Disks => "disks",
        }
    }

    /// The default fields to display, in order.
    pub fn default_order() -> Vec<Field> {
        vec![
            Field::Host,
            Field::Os,
            Field::Kernel,
            Field::Shell,
            Field::Uptime,
            Field::Cpu,
            Field::Memory,
            Field::Swap,
            Field::Disks,
        ]
    }
}

/// Error returned when a string does not name a known [`Field`].
#[derive(Debug, PartialEq, Eq)]
pub struct ParseFieldError(pub String);

impl fmt::Display for ParseFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown field '{}'", self.0)
    }
}

impl std::error::Error for ParseFieldError {}

impl FromStr for Field {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "host" => Ok(Field::Host),
            "os" => Ok(Field::Os),
            "kernel" => Ok(Field::Kernel),
            "shell" => Ok(Field::Shell),
            "uptime" => Ok(Field::Uptime),
            "cpu" => Ok(Field::Cpu),
            "memory" => Ok(Field::Memory),
            "swap" => Ok(Field::Swap),
            "disks" => Ok(Field::Disks),
            _ => Err(ParseFieldError(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_canonical_ids() {
        assert_eq!("host".parse(), Ok(Field::Host));
        assert_eq!("memory".parse(), Ok(Field::Memory));
        assert_eq!("disks".parse(), Ok(Field::Disks));
    }

    #[test]
    fn parsing_is_case_insensitive() {
        assert_eq!("CPU".parse(), Ok(Field::Cpu));
        assert_eq!("Os".parse(), Ok(Field::Os));
    }

    #[test]
    fn rejects_unknown_field() {
        assert_eq!("bogus".parse::<Field>(), Err(ParseFieldError("bogus".into())));
    }

    #[test]
    fn id_round_trips_through_from_str() {
        for field in Field::default_order() {
            assert_eq!(field.id().parse(), Ok(field));
        }
    }

    #[test]
    fn deserializes_from_toml_list() {
        #[derive(Deserialize)]
        struct Wrapper {
            fields: Vec<Field>,
        }
        let parsed: Wrapper = toml::from_str(r#"fields = ["host", "cpu", "disks"]"#).unwrap();
        assert_eq!(parsed.fields, vec![Field::Host, Field::Cpu, Field::Disks]);
    }
}
