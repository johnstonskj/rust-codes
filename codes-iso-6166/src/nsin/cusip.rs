/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::InternationalSecuritiesIdError;
use codes_check_digits::{luhn, Calculator, CodeWithCheckDigits};
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
/// See <https://en.wikipedia.org/wiki/CUSIP>
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Cusip(String);

pub const NATIONAL_ASSIGNMENT_AGENCY: &str = "CUSIP Services Bureau";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const TYPE_NAME: &str = "CUSIP";

code_impl!(Cusip, as_str, str, String, to_string);

code_as_str!(Cusip);

fixed_length_code!(Cusip, 20);

impl CodeWithCheckDigits for Cusip {
    type CheckDigit = u8;
    type CheckDigitCalculator = luhn::CheckDigitAlgorithm;
    const CHECK_DIGIT_ALGORITHM: Self::CheckDigitCalculator = *luhn::get_algorithm_instance();
}

impl FromStr for Cusip {
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
            Ok(Cusip(s.to_string()))
        }
    }
}
