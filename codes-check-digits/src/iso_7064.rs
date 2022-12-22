/*!
Provides an implementation of the
[ISO/IEC 7064:2003](https://www.iso.org/standard/31531.html)
*Information technology — Security techniques — Check character
systems* standard.

Also, see Code of Federal Regulations, Title 12 - Banks and Banking,
[Appendix C to Part 1003—Procedures for Generating a Check Digit and
Validating a ULI](https://www.govinfo.gov/content/pkg/CFR-2016-title12-vol8/xml/CFR-2016-title12-vol8-part1003-appC.xml).

# Example

YYYYY

*/

use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::{
    common::{
        ascii_alphanum_to_u8, calculate_mod, is_ascii_alpha_upper, is_ascii_alphanumeric_upper,
        is_ascii_numeric, string_to_numeric_string,
    },
    Calculator,
};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum IsoVariant {
    ///
    /// The data digits must be '0' - '9' but the check digit may
    /// also be 'X' (representing the value 10). Usage: unknown.
    /// This is not the ISBN check.
    ///
    Mod_11_2,
    ///
    /// The ISO 7064 standard defines a family of check digits in
    /// the form of "Mod N+1,N".
    ///
    /// It catches all single digit errors but does not catch
    /// transposition errors "01" → "10" (but not vice-versa) and
    /// "34" → "43" (but not vice-versa).
    ///
    Mod_11_10,
    ///
    /// ISO 7064 Mod_27_26 -- same as Mod_37_36 but restricted to
    /// 'A'-'Z'
    ///
    Mod_27_26,
    Mod_37_2,
    ///
    /// This scheme works similar to Mod_11_10, but is defined to
    /// use alphanumeric characters '0' -'9', 'A' - 'Z' for the
    /// data and check digit
    ///
    Mod_37_36,
    ///
    /// While this scheme is described in the standard as taking only
    /// a numeric alphabet it is used in a number of other standards
    /// with alpha-numeric input.
    ///
    /// 1. IF alpha-numeric:
    ///    1. replace any alphabetic character with it's numeric
    ///       equivalent where 'A' is 10, 'B' is 11, etc. The
    ///       input string '9A8C' therefore becomes '910812'.
    /// 2. Convert this string into an integer value. An arbitrary
    ///    precision integer is likely needed for longer input strings.
    /// 3. The check value is calculated as `98 - (n % 97)`.
    /// 4. If the value is less than 10 add a '0' pad character to the
    ///    left to produce the two character value.
    ///
    Mod_97_10,
    ///
    /// ISO 7064 Mod_661_26 -- restricted to 'A'-'Z' and produces 2
    /// check digits
    ///
    Mod_661_26,
    ///
    /// ISO 7064 Mod_1271_36 -- alphanumeric and produces 2 check
    /// digits
    ///
    Mod_1271_36,
}

#[derive(Debug)]
pub struct CheckDigitAlgorithm {
    variant: IsoVariant,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub const fn get_algorithm_instance(variant: IsoVariant) -> CheckDigitAlgorithm {
    CheckDigitAlgorithm::new(variant)
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for IsoVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl IsoVariant {
    const fn check_digits(&self) -> usize {
        match self {
            Self::Mod_11_2 => 1,
            Self::Mod_11_10 => 1,
            Self::Mod_27_26 => 1,
            Self::Mod_37_2 => 1,
            Self::Mod_37_36 => 1,
            Self::Mod_97_10 => 2,
            Self::Mod_661_26 => 2,
            Self::Mod_1271_36 => 2,
        }
    }

    const fn name(&self) -> &'static str {
        match self {
            Self::Mod_11_2 => "ISO 7064 - MOD 11-2",
            Self::Mod_11_10 => "ISO 7064 - MOD 11,10",
            Self::Mod_27_26 => "ISO 7064 - MOD 27,26",
            Self::Mod_37_2 => "ISO 7064 - MOD 37-2",
            Self::Mod_37_36 => "ISO 7064 - MOD 37,36",
            Self::Mod_97_10 => "ISO 7064 - MOD 97-10",
            Self::Mod_661_26 => "ISO 7064 - MOD 661-26",
            Self::Mod_1271_36 => "ISO 7064 - MOD 1271-36",
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl CheckDigitAlgorithm {
    pub const fn new(variant: IsoVariant) -> Self {
        Self { variant }
    }
}

impl Calculator<u8> for CheckDigitAlgorithm {
    fn name(&self) -> &'static str {
        self.variant.name()
    }

    fn number_of_check_digit_chars(&self) -> usize {
        self.variant.check_digits()
    }

    fn calculate(&self, s: &str) -> Result<u8, crate::error::CheckDigitError> {
        match self.variant {
            IsoVariant::Mod_11_2 => is_ascii_numeric(s)?,
            IsoVariant::Mod_27_26 => is_ascii_alpha_upper(s)?,
            IsoVariant::Mod_97_10 => {
                is_ascii_alphanumeric_upper(s)?;
                let s = format!("{}00", string_to_numeric_string(s, ascii_alphanum_to_u8));
                return Ok(98 - calculate_mod(&s, 97) as u8);
            }
            IsoVariant::Mod_661_26 => is_ascii_alpha_upper(s)?,
            _ => is_ascii_alphanumeric_upper(s)?,
        }
        todo!()
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
    fn test_string_to_string_a36() {
        assert_eq!(
            &string_to_numeric_string("10BX939C5543TQA1144M999143X", ascii_alphanum_to_u8),
            "10113393912554329261011442299914333"
        );
    }
}
