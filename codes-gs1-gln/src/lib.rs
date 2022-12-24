/*!
This package contains an implementation of the
[GS1 GLN](https://www.gs1.org/standards/id-keys/gln) Global Location Number (GLN) specification.

The Global Location Number (GLN) provides a global supply chain solution by uniquely identifying parties and locations that are involved in business transactions.

The GLN Allocation Rules Standard and contained GLN Management Rules is designed to help industry make consistent decisions about the unique identification of parties and locations in open supply chains. This standard has been developed in accordance with the GS1 Global Standards Management Process (GSMP) and is considered part of the GS1 system of standards. Overall, costs are minimised when all partners in the supply chain adhere to the GLN Allocation Rules Standard.

Unique identification is critical to maintaining operational efficiencies that business partners rely on to exchange information in consistent ways, as well as, ensuring the smooth operations of global supply and value chains. More specifically, the unique identification of parties and locations is critical for efficient logistic operations, traceability programs, recall readiness, and more. It is essential that accurate and up-to-date information on parties and locations is able to be readily shared between business partners to allow the “who” and “where” of business to be reliably answered no matter the use case.

# Example

```rust
use codes_check_digits::CodeWithCheckDigits;
use codes_common::Code;
use codes_gs1_gln::GlobalLocationNumber;
use std::str::FromStr;

let gln = GlobalLocationNumber::from_str("9436465792104").unwrap();

assert_eq!(gln.data_no_check_digit(), "943646579210");
assert_eq!(gln.check_digit_as_str(), "4");

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

use codes_agency::{standardized_type, Agency, Standard};
use codes_check_digits::{check_digits_impl, gs1, CodeWithCheckDigits};
use codes_common::{code_as_str, code_impl, fixed_length_code};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

code_impl!(GlobalLocationNumber, as_str, str, String, to_string);

code_as_str!(GlobalLocationNumber);

check_digits_impl!(
    GlobalLocationNumber,
    GlobalLocationNumberError,
    gs1::CheckDigitAlgorithm,
    u8,
    gs1::get_algorithm_instance(gs1::CodeFormat::Gln)
);

fixed_length_code!(GlobalLocationNumber, 13);

standardized_type!(GlobalLocationNumber, GS1_GLN);

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use codes_common::Code;

    #[test]
    fn test_is_valid() {
        assert!(GlobalLocationNumber::is_valid("1234567890128"));

        assert!(!GlobalLocationNumber::is_valid("9436465792103"));
    }
}
