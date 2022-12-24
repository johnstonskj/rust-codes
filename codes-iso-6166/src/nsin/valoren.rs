/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::InternationalSecuritiesIdError;
use codes_common::error::{invalid_format, invalid_length};
use codes_common::{code_as_str, code_impl, variable_length_code, VariableLengthCode};
use std::str::FromStr;
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// See <https://en.wikipedia.org/wiki/Valoren_number>
///
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
struct Valor(String);

pub const NATIONAL_ASSIGNMENT_AGENCY: &str = "SIX Financial";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const TYPE_NAME: &str = "VALOR";

code_impl!(Valor, as_str, str, String, to_string);

code_as_str!(Valor);

variable_length_code!(Valor, 5, 9);

impl FromStr for Valor {
    type Err = InternationalSecuritiesIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < Self::min_length() || s.len() > Self::max_length() {
            warn!(
                "{} must be {}..{} characters long, not {}",
                TYPE_NAME,
                Self::min_length(),
                Self::max_length(),
                s.len()
            );
            Err(invalid_length(TYPE_NAME, s.len()))
        } else if !s.chars().all(|c| c.is_ascii_digit()) {
            Err(invalid_format(TYPE_NAME, s))
        } else {
            Ok(Valor(s.to_string()))
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::nsin::valoren::Valor;
    use codes_common::Code;

    #[test]
    fn test_valoren_is_valid() {
        assert!(Valor::is_valid("1213853"));
    }
}
