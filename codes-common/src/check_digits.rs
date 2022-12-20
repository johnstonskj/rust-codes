/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_format, CodeParseError};
use std::str::FromStr;
use tracing::warn;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Calculator {
    fn calculate(&self, s: &str) -> Result<u8, CodeParseError>;

    fn validate(&self, s: &str) -> Result<(), CodeParseError> {
        let check_digit_index = s.len() - 1;
        let check = self.calculate(&s[0..check_digit_index])?;
        if u8::from_str(&s[check_digit_index..]).unwrap() == check {
            Ok(())
        } else {
            warn!(
                "Check digit {:?} invalid, expecting {}",
                &s[check_digit_index..],
                check
            );
            Err(invalid_format("(check digit)", s))
        }
    }

    fn is_valid(&self, s: &str) -> bool {
        self.validate(s).is_ok()
    }
}

#[derive(Debug, Default)]
pub struct LuhnAlgorithm {}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Calculator for LuhnAlgorithm {
    fn calculate(&self, s: &str) -> Result<u8, CodeParseError> {
        let sum: u8 = s
            .chars()
            .flat_map(|c| {
                if c.is_ascii_uppercase() {
                    let n: u8 = (c as u8) - 55; // magic number, corresponds to b'7'
                    vec![n / 10, n % 10]
                } else if c.is_ascii_digit() {
                    vec![(c as u8) - b'0']
                } else {
                    unreachable!("Character {:?} not expected", c)
                }
            })
            .rev()
            .enumerate()
            .map(|(i, n)| {
                if i & 1 == 1 {
                    n
                } else if n > 4 {
                    1 + ((n * 2) - 10)
                } else {
                    n * 2
                }
            })
            .sum();

        Ok((10 - (sum % 10)) % 10)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_digits() {
        let calculator: LuhnAlgorithm = Default::default();
        assert!(calculator.validate("US0378331005").is_ok());
        assert!(calculator.validate("037833100").is_ok());
    }
}
