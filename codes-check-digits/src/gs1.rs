/*!
The type [CheckDigitAlgorithm] provides an implementation of the
check digits specified for GS1 data types.

# Example

```rust
use codes_check_digits::{gs1, Calculator};

let calculator = gs1::get_algorithm_instance(gs1::CodeFormat::Gln);
assert!(calculator.is_valid("9436465792104"));
assert!(calculator.validate("9436465792104").is_ok());
assert_eq!(calculator.calculate("943646579210"), Ok(4));
```

*/

use crate::{
    common::{is_ascii_numeric, is_length_eq},
    error::CheckDigitError,
    Calculator,
};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This enumeration denotes the type of code to be validated.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CodeFormat {
    /// Global Trade Item Number GTIN-8
    Gtin8,
    /// Global Trade Item Number GTIN-12
    Gtin12,
    /// Global Trade Item Number GTIN-13
    Gtin13,
    /// Global Trade Item Number GTIN-14
    Gtin14,
    /// Global Location Number (GLN)
    Gln,
    /// Serial Shipping Container Code (SSCC)
    Sscc,
    /// European Article Number EAN-8
    LegacyEan8,
    /// European Article Number EAN-13
    LegacyEan13,
    /// Universal Product Code UPC-A
    LegacyUpcA,
}

///
/// Validate the different identifiers defined by GS1.
///
#[derive(Clone, Copy, Debug)]
pub struct CheckDigitAlgorithm {
    code_type: CodeFormat,
    length: usize,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub const fn get_algorithm_instance(code_format: CodeFormat) -> CheckDigitAlgorithm {
    CheckDigitAlgorithm::new(code_format)
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for CodeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl CodeFormat {
    const fn length(&self) -> usize {
        match self {
            Self::Gtin8 => 8,
            Self::Gtin12 => 12,
            Self::Gtin13 => 13,
            Self::Gtin14 => 14,
            Self::Gln => 13,
            Self::Sscc => 18,
            Self::LegacyEan8 => 8,
            Self::LegacyEan13 => 13,
            Self::LegacyUpcA => 12,
        }
    }
    const fn name(&self) -> &'static str {
        match self {
            Self::Gtin8 => "GS1 GTIN-8",
            Self::Gtin12 => "GS1 GTIN-12",
            Self::Gtin13 => "GS1 GTIN-13",
            Self::Gtin14 => "GS1 GTIN-14",
            Self::Gln => "GS1 GLN",
            Self::Sscc => "GS1 SSCC",
            Self::LegacyEan8 => "GS1 EAN-8",
            Self::LegacyEan13 => "GS1 EAN-13",
            Self::LegacyUpcA => "GS1 UPC-A",
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Calculator<u8> for CheckDigitAlgorithm {
    fn name(&self) -> &'static str {
        self.code_type.name()
    }

    fn calculate(&self, s: &str) -> Result<u8, CheckDigitError> {
        is_length_eq(s, self.length - self.number_of_check_digit_chars())?;
        is_ascii_numeric(s)?;
        let remainder: u16 = s
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| gs1_multiply_even_character(i, c) as u16)
            .sum::<u16>()
            % 10;
        if remainder == 0 {
            Ok(0)
        } else {
            Ok(10 - remainder as u8)
        }
    }
}

impl From<CodeFormat> for CheckDigitAlgorithm {
    fn from(code_type: CodeFormat) -> Self {
        Self::new(code_type)
    }
}

impl CheckDigitAlgorithm {
    const fn new(code_type: CodeFormat) -> Self {
        Self {
            code_type,
            length: code_type.length(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn gs1_multiply_even_character(i: usize, c: char) -> u8 {
    let n = (c as u8) - b'0';
    if i & 1 == 0 {
        n * 3
    } else {
        n
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::gs1::{CheckDigitAlgorithm, CodeFormat};
    use crate::Calculator;

    #[test]
    fn test_check_digits() {
        assert_eq!(
            CheckDigitAlgorithm::new(CodeFormat::Gln).calculate("123456789012"),
            Ok(8)
        );
        assert_eq!(
            CheckDigitAlgorithm::new(CodeFormat::Gln).calculate("210987654321"),
            Ok(0)
        );
        assert_eq!(
            CheckDigitAlgorithm::new(CodeFormat::Gln).calculate("943646579210"),
            Ok(4)
        );
    }
}
