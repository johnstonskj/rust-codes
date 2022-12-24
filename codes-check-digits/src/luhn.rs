/*!
The type [CheckDigitAlgorithm] provides an implementation of the
[Luhn Algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm).

# Example

```rust
use codes_check_digits::{luhn, Calculator};

let calculator = luhn::get_algorithm_instance();
assert!(calculator.is_valid("US0378331005"));
assert!(calculator.validate("US0378331005").is_ok());
assert_eq!(calculator.calculate("US037833100"), Ok(5));
```

*/

use crate::{common::is_ascii_alphanumeric_upper, error::CheckDigitError, Calculator};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------
///
/// This algorithm is known by many names: the "Luhn Formula", "The IBM Check",
/// "Mod 10", and is officially specified in "Annex B to ISO/IEC 7812, Part 1"
/// and in "ANSI X4.13". It is used as the check scheme on credit cards such
/// as Visa, Master Card, and American Express.
///
/// # Issues
///
/// It catches all single digit errors, but does not catch transposition errors
/// with "0" and "9" (meaning "09" → "90" and "90" → "09" are not caught).
///
#[derive(Clone, Copy, Debug, Default)]
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

impl Calculator<u8> for CheckDigitAlgorithm {
    fn name(&self) -> &'static str {
        "Luhn Algorithm (ISO/IEC 7812, Part 1, Annex B)"
    }

    fn calculate(&self, s: &str) -> Result<u8, CheckDigitError> {
        is_ascii_alphanumeric_upper(s)?;
        let sum: u8 = s
            .chars()
            .flat_map(luhn_alphanum_to_vec)
            .rev()
            .enumerate()
            .map(|(i, n)| luhn_double_odd(i & 1 == 1, n))
            .sum();

        Ok((10 - (sum % 10)) % 10)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn luhn_double_odd(is_odd: bool, d: u8) -> u8 {
    if is_odd {
        d
    } else {
        match d {
            0..=4 => d * 2,
            5 => 1,
            6 => 3,
            7 => 5,
            8 => 7,
            9 => 9,
            _ => panic!(),
        }
    }
}

#[inline(always)]
fn luhn_alphanum_to_vec(c: char) -> Vec<u8> {
    let d = c as u8;
    match d {
        b'0'..=b'9' => vec![d - b'0'],
        b'A'..=b'Z' => {
            let d = d - 55;
            vec![d / 10, d % 10]
        }
        _ => panic!(),
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::luhn::CheckDigitAlgorithm;
    use crate::Calculator;

    #[test]
    fn test_check_digits() {
        let calculator: CheckDigitAlgorithm = Default::default();
        assert!(calculator.validate("US0378331005").is_ok());
        assert!(calculator.validate("037833100").is_ok());
    }
}
