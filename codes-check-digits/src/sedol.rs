/*!
The type [CheckDigitAlgorithm] provides an implementation of the check
digit calculation as part of the SEDOL specification.

# Example

```rust
use codes_check_digits::{sedol, Calculator};

let calculator = sedol::get_algorithm_instance();
assert!(calculator.is_valid("0540528"));
assert!(calculator.validate("0540528").is_ok());
assert_eq!(calculator.calculate("054052"), Ok(8));
 ```

*/

use crate::{
    common::{ascii_alphanum_to_u8, is_ascii_alphanumeric_upper_no_vowels, is_length_eq},
    error::CheckDigitError,
    Calculator,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Validate the SEDOL numbers defined by the London Stock Exchange
///
#[derive(Debug, Default)]
pub struct CheckDigitAlgorithm {}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const SHARED_INSTANCE: CheckDigitAlgorithm = CheckDigitAlgorithm {};

pub const fn get_algorithm_instance() -> &'static CheckDigitAlgorithm {
    &SHARED_INSTANCE
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const WEIGHTS: [u16; 6] = [1, 3, 1, 7, 3, 9];

impl Calculator<u8> for CheckDigitAlgorithm {
    fn name(&self) -> &'static str {
        "Stock Exchange Daily Official List (SEDOL)"
    }

    fn calculate(&self, s: &str) -> Result<u8, CheckDigitError> {
        is_length_eq(s, 6)?;
        is_ascii_alphanumeric_upper_no_vowels(s)?;
        let sum: u16 = s
            .chars()
            .enumerate()
            .map(|(i, c)| (ascii_alphanum_to_u8(c) as u16) * WEIGHTS[i])
            .sum();

        Ok(((10 - (sum % 10)) % 10) as u8)
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::sedol::CheckDigitAlgorithm;
    use crate::Calculator;

    #[test]
    fn test_validate_hsbc() {
        let sedol = CheckDigitAlgorithm::default();
        assert!(sedol.is_valid("0540528"));
        assert_eq!(sedol.calculate("054052"), Ok(8));
    }
}
