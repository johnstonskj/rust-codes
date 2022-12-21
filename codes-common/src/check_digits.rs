/*!
Provides tools for building different check-digit algorithms.

# Example

```rust
use codes_common::check_digits::{LuhnAlgorithm, Calculator};

let calculator: LuhnAlgorithm = Default::default();
assert!(calculator.validate("US0378331005").is_ok());
```

*/

use crate::error::CodeParseError;
use std::{fmt::Display, ops::RangeInclusive};
use tracing::{trace, warn};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

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
/// Trait for types that implement check digit algorithms.
///
pub trait Calculator<T>
where
    T: Display + PartialEq,
{
    const NUM_CHECK_DIGITS: usize = 1;

    fn name(&self) -> &'static str;

    ///
    /// Calculate a check digit for the provided string.
    ///
    fn calculate(&self, s: &str) -> Result<T, CheckDigitError>;

    ///
    /// Create a new string with the original data plus check digit.
    ///
    fn create(&self, s: &str) -> Result<String, CheckDigitError> {
        Ok(format!("{}{}", s, self.calculate(s)?))
    }

    ///
    /// Validate that the string is valid and that it contains a valid
    /// check digit.
    ///
    fn validate<S>(&self, s: S) -> Result<(), CheckDigitError>
    where
        S: AsRef<str>,
    {
        let s = s.as_ref();
        trace!(
            algorithm_name = self.name(),
            num_check_digits = Self::NUM_CHECK_DIGITS,
            "Validating check digits for input {:?}",
            s
        );
        let check_digit_index = s.len() - Self::NUM_CHECK_DIGITS;
        let check = self.calculate(&s[0..check_digit_index])?;
        if s[check_digit_index..] == check.to_string() {
            Ok(())
        } else {
            Err(invalid_check_digit(&s[check_digit_index..], check))
        }
    }

    ///
    /// Returns `true` if the provided string includes a valid check digit,
    /// else `false`. The default implementation relies on the `validate`
    /// method.
    ///
    fn is_valid<S>(&self, s: S) -> bool
    where
        S: AsRef<str>,
    {
        self.validate(s).is_ok()
    }
}

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
#[derive(Debug, Default)]
pub struct LuhnAlgorithm {}

#[derive(Debug, Default)]
pub struct IsoAlgorithm {}

///
/// This enumeration denotes the type of code to be validated.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GsOneCode {
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
#[derive(Debug)]
pub struct GsOne {
    code_type: GsOneCode,
    length: usize,
}

///
/// Validate the SEDOL numbers defined by the London Stock Exchange
///
#[derive(Debug, Default)]
pub struct Sedol {}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

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

impl From<CheckDigitError> for CodeParseError {
    fn from(e: CheckDigitError) -> Self {
        Self::CheckDigit(e)
    }
}

impl std::error::Error for CheckDigitError {}

// ------------------------------------------------------------------------------------------------

impl Calculator<u8> for LuhnAlgorithm {
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

impl IsoAlgorithm {
    ///
    /// The data digits must be '0' - '9' but the check digit may
    /// also be 'X' (representing the value 10). Usage: unknown.
    /// This is not the ISBN check.
    ///
    pub fn mod_11_2() -> Self {
        todo!()
    }

    ///
    /// The ISO 7064 standard defines a family of check digits in
    /// the form of "Mod N+1,N".
    ///
    /// It catches all single digit errors but does not catch
    /// transposition errors "01" → "10" (but not vice-versa) and
    /// "34" → "43" (but not vice-versa).
    ///
    pub fn mod_11_10() -> Self {
        todo!()
    }

    ///
    /// ISO 7064 Mod 27,26 -- same as Mod37,36 but restricted to
    /// 'A'-'Z'
    ///
    pub fn mod_27_26() -> Self {
        todo!()
    }

    pub fn mod_37_2() -> Self {
        todo!()
    }

    ///
    /// This scheme works similar to Mod 11,10, but is defined to
    /// use alphanumeric characters '0' -'9', 'A' - 'Z' for the
    /// data and check digit
    ///
    pub fn mod_37_36() -> Self {
        todo!()
    }

    ///
    /// This scheme produces two digits as a check. And as a result,
    /// it catches just about every possible error. If you can afford
    /// the extra digit, this system is superior to the dihedral check.
    ///
    /// It also has an especially compact formula. The check digits are
    /// given by mod(98 - mod(data * 100, 97), 97) and the verification
    /// is just mod(data_check,97) == 1. In practice an alternate
    /// algorithm (based on Horner's Rule) is used to avoid overflow
    /// issues.
    ///
    pub fn mod_97_10() -> Self {
        todo!()
    }

    ///
    /// ISO 7064 Mod 661-26 -- restricted to 'A'-'Z' and produces 2
    /// check digits
    ///
    pub fn mod_661_26() -> Self {
        todo!()
    }

    ///
    /// ISO 7064 Mod 1271-36 -- alphanumeric and produces 2 check
    /// digits
    ///
    pub fn mod_1271_36() -> Self {
        todo!()
    }
}

// ISO 7064 Mod 17,16
// This scheme works similar to Mod 11,10, but is defined but uses hexadecimal digits ('0' - '9', 'A' - 'F') for the data and check digit. Oddly it's not actually in the ISO 7064 spec, but some other ISO spec Usage ISAN, V-ISAN

// ------------------------------------------------------------------------------------------------

impl Display for GsOneCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl GsOneCode {
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
    const fn as_ref(&self) -> &'static str {
        match self {
            GsOneCode::Gtin8 => "GTIN-8",
            GsOneCode::Gtin12 => "GTIN-12",
            GsOneCode::Gtin13 => "GTIN-13",
            GsOneCode::Gtin14 => "GTIN-14",
            GsOneCode::Gln => "GLN",
            GsOneCode::Sscc => "SSCC",
            GsOneCode::LegacyEan8 => "EAN-8",
            GsOneCode::LegacyEan13 => "EAN-13",
            GsOneCode::LegacyUpcA => "UPC-A",
        }
    }
}

impl Calculator<u8> for GsOne {
    fn name(&self) -> &'static str {
        self.code_type.as_ref()
    }

    fn calculate(&self, s: &str) -> Result<u8, CheckDigitError> {
        is_length_eq(s, self.length - GsOne::NUM_CHECK_DIGITS)?;
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

impl From<GsOneCode> for GsOne {
    fn from(code_type: GsOneCode) -> Self {
        Self::new(code_type)
    }
}

impl GsOne {
    pub const fn new(code_type: GsOneCode) -> Self {
        Self {
            code_type,
            length: code_type.length(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Calculator<u8> for Sedol {
    fn name(&self) -> &'static str {
        "Stock Exchange Daily Official List (SEDOL)"
    }

    fn calculate(&self, s: &str) -> Result<u8, CheckDigitError> {
        const WEIGHTS: [u16; 6] = [1, 3, 1, 7, 3, 9];
        is_length_eq(s, 6)?;
        is_ascii_alphanumeric_upper_no_vowels(s)?;
        let sum: u16 = s
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let n = if c.is_ascii_uppercase() {
                    (c as u16) - 55 // magic number, corresponds to b'7'
                } else if c.is_ascii_digit() {
                    (c as u16) - (b'0' as u16)
                } else {
                    unreachable!()
                };
                n * WEIGHTS[i]
            })
            .sum();

        Ok(((10 - (sum % 10)) % 10) as u8)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ========== Error functions

fn invalid_length(expecting: RangeInclusive<usize>, got: usize) -> CheckDigitError {
    warn!("Invalid input length, {} not in {:?}", got, expecting);
    CheckDigitError::InvalidLength {
        min: *expecting.start(),
        max: *expecting.end(),
        got,
    }
}

fn invalid_alphabet<S>(alphabet: S) -> CheckDigitError
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

fn invalid_check_digit<T1, T2>(expecting: T1, got: T2) -> CheckDigitError
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

// ========== Input validation functions

#[allow(dead_code)]
#[inline(always)]
fn is_in_length_range(s: &str, range: RangeInclusive<usize>) -> Result<(), CheckDigitError> {
    if range.contains(&s.len()) {
        Ok(())
    } else {
        Err(invalid_length(range, s.len()))
    }
}

#[allow(dead_code)]
#[inline(always)]
fn is_length_eq(s: &str, expected: usize) -> Result<(), CheckDigitError> {
    if s.len() == expected {
        Ok(())
    } else {
        Err(invalid_length(expected..=expected, s.len()))
    }
}

#[allow(dead_code)]
#[inline(always)]
fn is_ascii_numeric(s: &str) -> Result<(), CheckDigitError> {
    if s.chars().all(|c| matches!(c, '0'..='9')) {
        Ok(())
    } else {
        Err(invalid_alphabet("ascii-numeric"))
    }
}

#[allow(dead_code)]
#[inline(always)]
fn is_ascii_alpha_upper(s: &str) -> Result<(), CheckDigitError> {
    if s.chars().all(|c| matches!(c, 'A'..='Z')) {
        Ok(())
    } else {
        Err(invalid_alphabet("ascii-alpha-upper"))
    }
}

#[inline(always)]
fn is_ascii_alphanumeric_upper(s: &str) -> Result<(), CheckDigitError> {
    if s.chars().all(|c| matches!(c, '0'..='9' | 'A'..='Z')) {
        Ok(())
    } else {
        Err(invalid_alphabet("ascii-alphanumeric-upper"))
    }
}

#[inline(always)]
fn is_ascii_alphanumeric_upper_no_vowels(s: &str) -> Result<(), CheckDigitError> {
    if s.chars()
        .all(|c| matches!(c, '0'..='9' | 'B'..='D' | 'F'..='H' | 'J'..='N' | 'P'..='T' | 'V'..='Z'))
    {
        Ok(())
    } else {
        Err(invalid_alphabet("ascii-alphanumeric-upper"))
    }
}

// ========== String transform functions

#[allow(dead_code)]
#[inline(always)]
fn string_to_byte_vecs(s: &str, f: fn(char) -> Vec<u8>) -> impl Iterator<Item = u8> + '_ {
    s.chars().flat_map(f)
}

#[allow(dead_code)]
#[inline(always)]
fn string_to_bytes(s: &str, f: fn(char) -> u8) -> impl Iterator<Item = u8> + '_ {
    s.chars().map(f)
}

// ========== Character transform functions

#[allow(dead_code)]
#[inline(always)]
fn ascii_numeric_to_u8(c: char) -> u8 {
    let d = c as u8;
    match d {
        b'0'..=b'9' => d - b'0',
        _ => panic!(),
    }
}

#[allow(dead_code)]
#[inline(always)]
fn ascii_numeric_or_x_to_u8(c: char) -> u8 {
    let d = c as u8;
    match d {
        b'0'..=b'9' => d - b'0',
        b'X' => 10,
        _ => panic!(),
    }
}

#[allow(dead_code)]
#[inline(always)]
fn ascii_alpha_to_u8(c: char) -> u8 {
    let d = c as u8;
    match d {
        b'A'..=b'Z' => d - b'A',
        _ => panic!(),
    }
}

#[allow(dead_code)]
#[inline(always)]
fn ascii_alphanum_to_u8(c: char) -> u8 {
    let d = c as u8;
    match d {
        b'0'..=b'9' => d - b'0',
        b'A'..=b'Z' => d - b'A' + 10,
        _ => panic!(),
    }
}

#[allow(dead_code)]
#[inline(always)]
fn ascii_alphanum_or_star_to_u8(c: char) -> u8 {
    let d = c as u8;
    match d {
        b'0'..=b'9' => d - b'0',
        b'A'..=b'Z' => d - b'A' + 10,
        b'*' => 36,
        _ => panic!(),
    }
}

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
