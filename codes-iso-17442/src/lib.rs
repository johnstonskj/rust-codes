/*!
This package contains an implementation of the
[ISO 17442](https://www.iso.org/standard/78829.html)
Legal Entity Identifier (LEI) specification.

The Legal Entity Identifier (LEI) is a unique global identifier for legal
entities participating in financial transactions. Also known as an LEI code
or LEI number, its purpose is to help identify legal entities on a globally
accessible database. Legal entities are organisations such as companies or
government entities that participate in financial transactions. The identifier
is used in regulatory reporting to financial regulators and all financial
companies and funds are required to have an LEI.


The ISO Specification defines a format for describing identifiers and
including check digits to ensure validity. In turn
[The Global Legal Entity Identifier Foundation](https://www.gleif.org/en)
(GLEIF) is the top-level maintainer of the global registry of identifiers
and as such has further refined the LEI format to contain the following
components:

1. Characters 1-4: Prefix used to ensure the uniqueness among codes from LEI
   issuers (Local Operating Units or LOUs).
2. Characters 5-18: Entity-specific part of the code generated and assigned by
   LOUs according to transparent, sound and robust allocation policies. As
   required by ISO 17442, it contains no embedded intelligence.
3. Characters 19-20: Two check digits as described in the ISO 17442 standard.

GLIEF also provides daily [download files](https://www.gleif.org/en) for
**all** registered identifiers, and an LEI search API.

# Example

```rust
use codes_iso_17442::LegalEntityId;
use std::str::FromStr;

let lei = LegalEntityId::from_str("YZ83GD8L7GG84979J516").unwrap();

assert_eq!(lei.local_operating_unit(), "YZ83");
assert_eq!(lei.entity(), "GD8L7GG84979J5");
assert_eq!(lei.check_digits(), "16");
```

# Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [LegalEntityId] type.
* `url` - Enables the conversion between LEI and URL (URN) forms.

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
use codes_common::{invalid_format, invalid_length};
use std::{fmt::Display, fmt::Formatter, ops::Deref, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An instance of the `Standard` struct defined in the
/// [`codes_agency`](https://docs.rs/codes-agency/latest/codes_agency/)
/// package that describes the ISO-17442 specification.
///
pub const ISO_17442: Standard = Standard::new_with_long_ref(
    Agency::ISO,
    "17442",
    "ISO 17442-1:2020",
    "Financial services — Legal entity identifier (LEI) — Part 1: Assignment",
    "https://www.iso.org/standard/78829.html",
);

/// The formatted Legal Entity Identifier (LEI) is formatted as a
/// 20-character, alpha-numeric code based on the ISO 17442 standard developed
/// by the International Organization for Standardization (ISO). It connects
/// to key information that enables clear and unique identification of legal
/// entities participating in financial transactions. Each LEI database entry
/// contains information about an entity's ownership and thus answers the
/// questions of "who is who" and "who owns whom". Therefore the publicly
/// available LEI data pool can be regarded as a global directory of
/// non-individual participants in the financial market.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LegalEntityId(String);

pub use codes_common::CodeParseError as LegalEntityIdError;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for LegalEntityId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for LegalEntityId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for LegalEntityId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<LegalEntityId> for String {
    fn from(v: LegalEntityId) -> String {
        v.0
    }
}

impl FromStr for LegalEntityId {
    type Err = LegalEntityIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 20 {
            Err(invalid_length("LegalEntityId", s.len()))
        } else if !s[..18].chars().all(|c| c.is_ascii_alphanumeric())
            || !s[18..].chars().all(|c| c.is_ascii_digit())
        {
            Err(invalid_format("LegalEntityId", s))
        } else {
            let num_str: String = s[..18]
                .chars()
                .map(|c| {
                    if c.is_ascii_digit() {
                        (c as u32) - 48
                    } else {
                        (c as u32) - 55
                    }
                    .to_string()
                })
                .collect();
            let num = u128::from_str(&format!("{}00", num_str)).unwrap();
            let check = 98 - num % 97;
            let check_str = format!("{:02}", check);
            if check_str == s[18..] {
                Ok(LegalEntityId(s.to_string()))
            } else {
                Err(invalid_format("LegalEntityId", s))
            }
        }
    }
}

#[cfg(feature = "urn")]
impl TryFrom<url::Url> for LegalEntityId {
    type Error = LegalEntityIdError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        if !value.scheme().eq_ignore_ascii_case("urn") {
            Err(invalid_format("LegalEntityId", value.scheme()))
        } else {
            let path = value.path();
            if path[0..4].eq_ignore_ascii_case("lei:") {
                LegalEntityId::from_str(&path[4..])
            } else {
                Err(invalid_format("LegalEntityId", path))
            }
        }
    }
}

#[cfg(feature = "urn")]
impl From<LegalEntityId> for url::Url {
    fn from(v: LegalEntityId) -> url::Url {
        url::Url::parse(&format!("urn:lei:{}", v.0)).unwrap()
    }
}

impl LegalEntityId {
    ///
    /// Return the portion of the LEI that corresponds to the Local Operating
    /// Unit (LOU) accredited by GLEIF to issue entity identifiers.
    ///
    pub fn local_operating_unit(&self) -> &str {
        &self.0[..4]
    }

    ///
    /// Return the portion of the LEI that corresponds to the Entity Identifier
    /// assigned by a GLEIF accredited Local Operating Unit (LOU).
    ///
    pub fn entity(&self) -> &str {
        &self.0[4..18]
    }

    ///
    /// Return the final two check digits from LEI.
    ///
    pub fn check_digits(&self) -> &str {
        &self.0[18..]
    }

    ///
    /// Return a reference to this LEI as a string.
    ///
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::LegalEntityId;
    use std::str::FromStr;

    #[test]
    fn test_some_valid_lei_1() {
        assert!(LegalEntityId::from_str("54930084UKLVMY22DS16").is_ok());
    }

    #[test]
    fn test_some_valid_lei_2() {
        assert!(LegalEntityId::from_str("213800WSGIIZCXF1P572").is_ok());
    }

    #[test]
    fn test_some_valid_lei_3() {
        assert!(LegalEntityId::from_str("5493000IBP32UQZ0KL24").is_ok());
    }

    #[test]
    fn test_some_valid_lei_4() {
        assert!(LegalEntityId::from_str("YZ83GD8L7GG84979J516").is_ok());
    }

    #[test]
    fn test_some_valid_lei_components() {
        let lei = LegalEntityId::from_str("YZ83GD8L7GG84979J516").unwrap();
        assert_eq!(lei.local_operating_unit(), "YZ83");
        assert_eq!(lei.entity(), "GD8L7GG84979J5");
        assert_eq!(lei.check_digits(), "16");
    }

    #[cfg(feature = "url")]
    #[test]
    fn test_lei_to_url() {
        use url::Url;

        let lei = LegalEntityId::from_str("YZ83GD8L7GG84979J516").unwrap();
        let url: Url = lei.into();

        assert_eq!(url.as_str(), "urn:lei:YZ83GD8L7GG84979J516");
    }

    #[cfg(feature = "url")]
    #[test]
    fn test_url_to_lei() {
        use url::Url;

        let url = Url::parse("urn:lei:YZ83GD8L7GG84979J516").unwrap();
        let lei = LegalEntityId::try_from(url).unwrap();

        assert_eq!(lei.as_str(), "YZ83GD8L7GG84979J516");
    }
}
