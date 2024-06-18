use crate::error::{invalid_alphabet, invalid_length, CheckDigitError};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[cfg(all(feature = "big_integer", not(target_family = "windows")))]
use rug::Integer;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ========== Input validation functions

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn is_in_length_range(
    s: &str,
    range: RangeInclusive<usize>,
) -> Result<(), CheckDigitError> {
    if range.contains(&s.len()) {
        Ok(())
    } else {
        Err(invalid_length(range, s.len()))
    }
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn is_length_eq(s: &str, expected: usize) -> Result<(), CheckDigitError> {
    if s.len() == expected {
        Ok(())
    } else {
        Err(invalid_length(expected..=expected, s.len()))
    }
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn is_ascii_numeric(s: &str) -> Result<(), CheckDigitError> {
    if s.chars().all(|c| c.is_ascii_digit()) {
        Ok(())
    } else {
        Err(invalid_alphabet("ascii-numeric"))
    }
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn is_ascii_alpha_upper(s: &str) -> Result<(), CheckDigitError> {
    if s.chars().all(|c| c.is_ascii_uppercase()) {
        Ok(())
    } else {
        Err(invalid_alphabet("ascii-alpha-upper"))
    }
}

#[inline(always)]
pub(crate) fn is_ascii_alphanumeric_upper(s: &str) -> Result<(), CheckDigitError> {
    if s.chars().all(|c| matches!(c, '0'..='9' | 'A'..='Z')) {
        Ok(())
    } else {
        Err(invalid_alphabet("ascii-alphanumeric-upper"))
    }
}

#[inline(always)]
pub(crate) fn is_ascii_alphanumeric_upper_no_vowels(s: &str) -> Result<(), CheckDigitError> {
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
pub(crate) fn string_to_byte_vecs(
    s: &str,
    f: fn(char) -> Vec<u8>,
) -> impl Iterator<Item = u8> + '_ {
    s.chars().flat_map(f)
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn string_to_bytes(s: &str, f: fn(char) -> u8) -> impl Iterator<Item = u8> + '_ {
    s.chars().map(f)
}

#[inline(always)]
pub(crate) fn string_to_numeric_string(s: &str, f: fn(char) -> u8) -> String {
    s.chars().map(f).map(|b| b.to_string()).collect()
}

// ========== Character transform functions

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn ascii_numeric_to_u8(c: char) -> u8 {
    let d = c as u8;
    match d {
        b'0'..=b'9' => d - b'0',
        _ => panic!(),
    }
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn ascii_numeric_or_x_to_u8(c: char) -> u8 {
    let d = c as u8;
    match d {
        b'0'..=b'9' => d - b'0',
        b'X' => 10,
        _ => panic!(),
    }
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn ascii_alpha_to_u8(c: char) -> u8 {
    let d = c as u8;
    match d {
        b'A'..=b'Z' => d - b'A',
        _ => panic!(),
    }
}

#[inline(always)]
pub(crate) fn ascii_alphanum_to_u8(c: char) -> u8 {
    let d = c as u8;
    match d {
        b'0'..=b'9' => d - b'0',
        b'A'..=b'Z' => d - b'A' + 10,
        _ => panic!(),
    }
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn ascii_alphanum_or_star_to_u8(c: char) -> u8 {
    let d = c as u8;
    match d {
        b'0'..=b'9' => d - b'0',
        b'A'..=b'Z' => d - b'A' + 10,
        b'*' => 36,
        _ => panic!(),
    }
}

// ========== Math operations

// The string length of u64::MAX
const MAX_STRING_LEN: usize = "18446744073709551615".len() - 4;

#[cfg(any(not(feature = "big_integer"), target_family = "windows"))]
#[inline(always)]
pub(crate) fn calculate_mod(s: &str, modulus: u16) -> u16 {
    if s.len() <= MAX_STRING_LEN {
        (u64::from_str(s).unwrap() % modulus as u64) as u16
    } else {
        calculate_mod(
            &format!(
                "{}{}",
                calculate_mod(&s[..MAX_STRING_LEN], modulus),
                &s[MAX_STRING_LEN..]
            ),
            modulus,
        )
    }
}

#[cfg(all(feature = "big_integer", not(target_family = "windows")))]
pub(crate) fn calculate_mod(s: &str, modulus: u16) -> u16 {
    if s.len() <= MAX_STRING_LEN {
        (u64::from_str(s).unwrap() % modulus as u64) as u16
    } else {
        (Integer::from_str(s).unwrap() % modulus).to_u16().unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_number_mod() {
        assert_eq!(calculate_mod("6735", 97), 42);
    }

    #[test]
    fn test_medium_number_mod() {
        assert_eq!(calculate_mod("2356482816436572726364746826735", 97), 10);
    }

    #[test]
    fn test_large_number_mod() {
        assert_eq!(
            calculate_mod(
                "2356482816436572726364746826735876349762450197396751623576456124",
                97
            ),
            23
        );
    }
}
