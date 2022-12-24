/*!
Provides tools for building different check-digit algorithms.

This package contains implementations of various check digit specifications,
including [ISO/IEC 7064:2003](https://www.iso.org/standard/31531.html)
Information technology — Security techniques — Check character systems.

# Example

```rust,ignore
use codes_check_digits::{luhn, Calculator};

let calculator = luhn::get_algorithm_instance();
assert!(calculator.is_valid("US0378331005"));
assert!(calculator.validate("US0378331005").is_ok());
assert_eq!(calculator.calculate("US037833100"), Ok(5));
```

# Features

* `gs1` - Adds the `gs1` module containing algorithms for various codes such as
  EAN, GTIN, GLN, and UPC.
* `iso_7064` - Adds the `iso_7064` module containing implementations of the
  variants defined in ISO/IEC 7064:2003.
* `luhn` - Adds the `luhn` module containing an implementation of the Luhn Algorithm.
* `sedol` - Adds the `sedol` module containing an implementation of the algorithm
  used in SEDOL numbers.

*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    private_in_public,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

use codes_common::Code;
use error::CheckDigitError;
use std::{borrow::Cow, fmt::Display};
use tracing::trace;

use crate::error::invalid_check_digit;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait CodeWithCheckDigits: Code<String> + AsRef<str> {
    type CheckDigit: Display + PartialEq;
    type CheckDigitCalculator: Calculator<Self::CheckDigit>;
    const CHECK_DIGIT_ALGORITHM: Self::CheckDigitCalculator;

    fn data_no_check_digit(&self) -> Cow<'_, str> {
        let s = self.as_ref();
        s[..(s.len() - Self::CHECK_DIGIT_ALGORITHM.number_of_check_digit_chars())].into()
    }

    fn check_digit_as_str(&self) -> Cow<'_, str> {
        let s = self.as_ref();
        s[(s.len() - Self::CHECK_DIGIT_ALGORITHM.number_of_check_digit_chars())..].into()
    }
}

///
/// Trait for types that implement check digit algorithms.
///
pub trait Calculator<T>
where
    T: Display + PartialEq,
{
    ///
    /// Return the number of characters used as the check digit.
    /// Currently it is assumed that these are the *n* right-most
    /// characters in the input string.
    ///
    fn number_of_check_digit_chars(&self) -> usize {
        1
    }

    ///
    /// Return the name of this algorithm.
    ///
    fn name(&self) -> &'static str;

    ///
    /// Calculate a check digit for the provided string.
    ///
    fn calculate(&self, s: &str) -> Result<T, CheckDigitError>;

    ///
    /// Create a new string with the original data plus check digit.
    ///
    fn create(&self, s: &str) -> Result<String, CheckDigitError> {
        Ok(format!(
            "{}{:0>width$}",
            s,
            self.calculate(s)?,
            width = self.number_of_check_digit_chars()
        ))
    }

    ///
    /// Validate that the string is valid and that it contains a valid
    /// check digit.
    ///
    fn validate<S>(&self, s: S) -> Result<(), CheckDigitError>
    where
        Self: Sized,
        S: AsRef<str>,
    {
        let s = s.as_ref();
        trace!(
            algorithm_name = self.name(),
            num_check_digits = self.number_of_check_digit_chars(),
            "Validating check digits for input {:?}",
            s
        );
        let check_digit_index = s.len() - self.number_of_check_digit_chars();
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
        Self: Sized,
        S: AsRef<str>,
    {
        self.validate(s).is_ok()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! check_digits_impl {
    ($type_name:ty, $error_type:ty, $algorithm_type:ty, $check_digit_type:ty, $algorithm_init:expr) => {
        impl CodeWithCheckDigits for $type_name {
            type CheckDigit = $check_digit_type;
            type CheckDigitCalculator = $algorithm_type;
            const CHECK_DIGIT_ALGORITHM: Self::CheckDigitCalculator = $algorithm_init;
        }

        impl ::std::str::FromStr for $type_name {
            type Err = GlobalLocationNumberError;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                use codes_check_digits::Calculator;
                Self::CHECK_DIGIT_ALGORITHM.validate(s)?;
                Ok(Self(s.to_string()))
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
mod common;

pub mod error;

#[cfg(feature = "gs1")]
pub mod gs1;

#[cfg(feature = "iso_7064")]
pub mod iso_7064;

#[cfg(feature = "luhn")]
pub mod luhn;

#[cfg(feature = "sedol")]
pub mod sedol;
