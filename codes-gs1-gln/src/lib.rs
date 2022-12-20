/*!
This package contains an implementation of the
[GS1 GLN](https://www.gs1.org/standards/id-keys/gln) Global Location Number (GLN) specification.

The Global Location Number (GLN) provides a global supply chain solution by uniquely identifying parties and locations that are involved in business transactions.

The GLN Allocation Rules Standard and contained GLN Management Rules is designed to help industry make consistent decisions about the unique identification of parties and locations in open supply chains. This standard has been developed in accordance with the GS1 Global Standards Management Process (GSMP) and is considered part of the GS1 system of standards. Overall, costs are minimised when all partners in the supply chain adhere to the GLN Allocation Rules Standard.

Unique identification is critical to maintaining operational efficiencies that business partners rely on to exchange information in consistent ways, as well as, ensuring the smooth operations of global supply and value chains. More specifically, the unique identification of parties and locations is critical for efficient logistic operations, traceability programs, recall readiness, and more. It is essential that accurate and up-to-date information on parties and locations is able to be readily shared between business partners to allow the “who” and “where” of business to be reliably answered no matter the use case.

# Example

```rust
use codes_gs1_gln::GlobalLocationNumber;
use std::str::FromStr;

let gln = GlobalLocationNumber::from_str("9436465792104").unwrap();

assert_eq!(gln.data(), "943646579210");
assert_eq!(gln.check_digit(), 4);

assert!(!GlobalLocationNumber::is_valid("9436465792109"));
```

```rust
use codes_gs1_gln::GS1_GLN;

assert_eq!(GS1_GLN.title(), "Global Location Number (GLN)");
```

# Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [GlobalLocationNumber] type.

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

use codes_agency::{Agency, Standard};
use codes_common::Code;
use std::{fmt::Display, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An instance of the `Standard` struct defined in the
/// [`codes_agency`](https://docs.rs/codes-agency/latest/codes_agency/)
/// package that describes the ISO-10383 specification.
///
pub const GS1_GLN: Standard = Standard::new(
    Agency::GS1,
    "GLN",
    "Global Location Number (GLN)",
    "https://www.gs1.org/standards/id-keys/gln",
);

///
/// Encapsulates the complete Global Location Number (GLN) including
/// check digit as a string.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GlobalLocationNumber(String);

pub use codes_common::CodeParseError as GlobalLocationNumberError;

///
/// GS1 codes such as the GLN but also GTIN-8, GTIN-12 (U.P.C.),
/// GTIN-13, and GTIN-14 all share the same mechanism for check digit
/// calculate, differing only in the length of the code in digits.
///
const CODE_LENGTH: usize = 13;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const TYPE_NAME: &str = "GlobalLocationNumber";

impl Display for GlobalLocationNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for GlobalLocationNumber {
    type Err = GlobalLocationNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 13 {
            Err(codes_common::invalid_length(TYPE_NAME, s.len()))
        } else if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(codes_common::invalid_format(TYPE_NAME, s))
        }
    }
}

impl From<GlobalLocationNumber> for String {
    fn from(v: GlobalLocationNumber) -> Self {
        v.0
    }
}

impl AsRef<str> for GlobalLocationNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Code<String> for GlobalLocationNumber {}

impl GlobalLocationNumber {
    ///
    /// Returns `true` if the passed string is a valid GLN
    /// including check digit.
    ///
    pub fn is_valid(s: &str) -> bool {
        if s.len() == 13 && s.chars().all(|c| c.is_ascii_digit()) {
            let calculated = Self::calculate_check_digit(&s[0..(CODE_LENGTH - 1)]);
            calculated.to_string().as_str() == &s[(CODE_LENGTH - 1)..]
        } else {
            false
        }
    }

    ///
    /// Return the data portion of the GLN, that is the string of digits
    /// excluding the check digit.
    ///
    pub fn data(&self) -> &str {
        &self.0[0..(CODE_LENGTH - 1)]
    }

    ///
    /// Return the check digit portion of the GLN only.
    ///
    pub fn check_digit(&self) -> u8 {
        u8::from_str(&self.0[(CODE_LENGTH - 1)..]).unwrap()
    }

    fn calculate_check_digit(s: &str) -> u32 {
        const BASE: u32 = '0' as u32;
        const FACTORS: [u32; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
        let remainder = s
            .chars()
            .enumerate()
            .map(|(i, c)| ((c as u32) - BASE) * FACTORS[i])
            .sum::<u32>()
            % 10;
        if remainder == 0 {
            0
        } else {
            10 - remainder
        }
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
        assert_eq!(
            GlobalLocationNumber::calculate_check_digit("123456789012"),
            8
        );
        assert_eq!(
            GlobalLocationNumber::calculate_check_digit("210987654321"),
            0
        );
        assert_eq!(
            GlobalLocationNumber::calculate_check_digit("943646579210"),
            4
        );
    }
}
