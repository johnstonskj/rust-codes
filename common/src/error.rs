use std::fmt;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Common `Error` type, mainly used for `FromStr` failures.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CodeParseError {
    /// The string to parse was either too short or too long.
    InvalidLength { type_name: String, length: usize },
    /// The value is incorrectly formatted
    InvalidFormat { type_name: String, value: String },
    /// The string value did not represent a known value.
    UnknownValue { type_name: String, value: String },
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn invalid_length<S>(type_name: S, length: usize) -> CodeParseError
where
    S: Into<String>,
{
    CodeParseError::InvalidLength {
        type_name: type_name.into(),
        length,
    }
}

pub fn invalid_format<S1, S2>(type_name: S1, value: S2) -> CodeParseError
where
    S1: Into<String>,
    S2: Into<String>,
{
    CodeParseError::InvalidFormat {
        type_name: type_name.into(),
        value: value.into(),
    }
}

pub fn unknown_value<S1, S2>(type_name: S1, value: S2) -> CodeParseError
where
    S1: Into<String>,
    S2: Into<String>,
{
    CodeParseError::UnknownValue {
        type_name: type_name.into(),
        value: value.into(),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl fmt::Display for CodeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidLength { type_name, length } => format!(
                    "The string passed is an invalid length for the type `{}`; length: {}",
                    type_name, length
                ),
                Self::InvalidFormat { type_name, value } => format!(
                    "The string passed is incorrectly formatted for type `{}`; value: {:?}",
                    type_name, value
                ),
                 Self::UnknownValue { type_name, value } => format!(
                    "The string passed is an invalid length for the not a known value of type `{}`; value: {:?}",
                    type_name, value
                ),
            }
        )
    }
}

impl std::error::Error for CodeParseError {}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
