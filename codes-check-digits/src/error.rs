/*!
Provides the crate's Error and Result types as well as helper
functions.

 */

use codes_common::CodeParseError;
use std::{
    fmt::{Debug, Display},
    ops::RangeInclusive,
};
use tracing::warn;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Errors resulting from the Check Digit [Calculator]
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckDigitError {
    /// Input string length is invalid.
    InvalidLength { min: usize, max: usize, got: usize },
    /// Input string does not conform to the expected alphabet.
    InvalidAlphabet { alphabet: String },
    /// The calculated check digit does not match the input.
    InvalidCheckDigit { expecting: String, got: String },
}

///
/// A Result type that specifically uses this crate's Error.
///
pub type Result<T> = std::result::Result<T, CheckDigitError>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn invalid_length(expecting: RangeInclusive<usize>, got: usize) -> CheckDigitError {
    warn!("Invalid input length, {} not in {:?}", got, expecting);
    CheckDigitError::InvalidLength {
        min: *expecting.start(),
        max: *expecting.end(),
        got,
    }
}

pub fn invalid_alphabet<S>(alphabet: S) -> CheckDigitError
where
    S: Into<String>,
{
    let alphabet = alphabet.into();
    warn!(
        "One or more input characters not in the alphabet {}",
        alphabet
    );
    CheckDigitError::InvalidAlphabet { alphabet }
}

pub fn invalid_check_digit<T1, T2>(expecting: T1, got: T2) -> CheckDigitError
where
    T1: Display,
    T2: Display,
{
    warn!("Invalid check digit expecting {}, got {}", expecting, got);
    CheckDigitError::InvalidCheckDigit {
        expecting: expecting.to_string(),
        got: got.to_string(),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for CheckDigitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CheckDigitError::InvalidLength { min, max, got } => format!(
                    "Expecting input length in the range {}..={}, not {}",
                    min, max, got
                ),
                CheckDigitError::InvalidAlphabet { alphabet } =>
                    format!("Expecting characters from the alphabet {:?}", alphabet),
                CheckDigitError::InvalidCheckDigit { expecting, got } => format!(
                    "Invalid check digit in string, expecting {}, got {}",
                    expecting, got
                ),
            }
        )
    }
}

impl std::error::Error for CheckDigitError {}

impl From<CheckDigitError> for CodeParseError {
    fn from(e: CheckDigitError) -> Self {
        Self::CheckDigit(Box::new(e))
    }
}
