//! Configuration errors shared by pitch processors.

use std::fmt;

/// An invalid pitch processor configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    /// A value must be a power of two.
    NotPowerOfTwo { what: &'static str, value: usize },
    /// A value must lie in the inclusive range.
    OutOfRange {
        what: &'static str,
        value: usize,
        min: usize,
        max: usize,
    },
    /// A named value is invalid.
    InvalidValue { what: &'static str },
    /// The number of pitch factors exceeds the limit.
    TooManyFactors { max: usize, got: usize },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotPowerOfTwo { what, value } => {
                write!(f, "{what} must be a power of two (got {value})")
            }
            Self::OutOfRange {
                what,
                value,
                min,
                max,
            } => write!(f, "{what} must be between {min} and {max} (got {value})"),
            Self::InvalidValue { what } => write!(f, "invalid {what}"),
            Self::TooManyFactors { max, got } => {
                write!(f, "too many factors (maximum {max}, got {got})")
            }
        }
    }
}

impl std::error::Error for ConfigError {}
