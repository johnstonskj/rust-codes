/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::InternationalSecuritiesIdError;
use codes_check_digits::{sedol, Calculator, CodeWithCheckDigits};
use codes_common::error::invalid_length;
use codes_common::{code_as_str, code_impl, fixed_length_code, FixedLengthCode};
use std::str::FromStr;
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// See <https://en.wikipedia.org/wiki/SEDOL>
///
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Sedol(String);

pub const NATIONAL_ASSIGNMENT_AGENCY: &str = "London Stock Exchange";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const TYPE_NAME: &str = "SEDOL";

code_impl!(Sedol, as_str, str, String, to_string);

code_as_str!(Sedol);

fixed_length_code!(Sedol, 7);

impl CodeWithCheckDigits for Sedol {
    type CheckDigit = u8;
    type CheckDigitCalculator = sedol::CheckDigitAlgorithm;
    const CHECK_DIGIT_ALGORITHM: Self::CheckDigitCalculator = *sedol::get_algorithm_instance();
}

impl FromStr for Sedol {
    type Err = InternationalSecuritiesIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != Self::fixed_length() {
            warn!(
                "{} must be {} characters long, not {}",
                TYPE_NAME,
                Self::fixed_length(),
                s.len()
            );
            Err(invalid_length(TYPE_NAME, s.len()))
        } else {
            Self::CHECK_DIGIT_ALGORITHM.validate(s)?;
            Ok(Sedol(s.to_string()))
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::nsin::sedol::Sedol;
    use codes_common::Code;

    #[test]
    fn test_sedol_is_valid() {
        assert!(Sedol::is_valid("0263494"));
    }
}
