/*!


# Example

```rust
```

# Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [LegalEntityId] type.
* `url` - Enables the conversion between ISIN and URL (URN) forms.

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
/// package that describes the ISO-6166 specification.
///
pub const ISO_6166: Standard = Standard::new_with_long_ref(
    Agency::ISO,
    "6166",
    "ISO 6166-1:2021",
    "Financial services — International securities identification number (ISIN)",
    "https://www.iso.org/standard/78502.html",
);

///
/// International Securities Identification Number (ISIN)
///
/// (ISO-6166)[https://www.isin.org]
///
/// * A two-letter country code, drawn from a list (ISO 6166) prepared by the
///   International Organization for Standardization (ISO). This code is
///   assigned according to the location of a company's head office. A special
///   code, 'XS' is used for international securities cleared through
///   pan-European clearing systems like Euroclear and CEDEL. Depository receipt
///   ISIN usage is unique in that the country code for the security is that of
///   the receipt issuer, not that of the underlying security.
/// * A nine-digit numeric identifier, called the National Securities
///   Identifying Number (NSIN), and assigned by each country's or region's . If
///   a national number is composed of less than nine digits, it is padded with
///   leading zeros to become a NSIN. The numeric identifier has no intrinsic
///   meaning it is essentially a serial number.
/// * A single check-digit. The digit is calculated based upon the preceding
///   11 characters/digits and uses a sum modulo 10 algorithm and helps ensure
///   against counterfeit numbers.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct InternationalSecuritiesId {
    country: CountryCode,
    // National Securities Identifying Number (NSIN)
    nsid: String,
}
pub use codes_common::CodeParseError as InternationalSecuritiesError;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const TYPE_NAME: &str = "InternationalSecuritiesId";

impl Display for InternationalSecuritiesId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if f.alternate() {
                format!("{}-{}-{}", self.country, &self.nsid[0..9], &self.nsid[9..])
            } else {
                format!("{}{}", self.country, self.nsid)
            }
        )
    }
}

impl From<InternationalSecuritiesId> for String {
    fn from(v: InternationalSecuritiesId) -> String {
        v.0
    }
}

impl FromStr for InternationalSecuritiesId {
    type Err = InternationalSecuritiesIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 14 {
            Self::from_str(s.replace('-', ""))
        } else if s.len() != 12 {
            warn!("ISIN must be 12 characters long, not {}", s.len());
            Err(invalid_identifier_value((TYPE_NAME, s))
        } else if let Ok(country_code) = CountryCode::from_str(&s[0..2]) {
            validate_check_digit(s)?;
            let nsid = &s[2..];
            if nsid.chars().all(|c| c.is_ascii_alphanumeric()) {
                Ok(InternationalSecuritiesId {
                    country: country_code,
                    nsid: validate_nsid(nsid)?,
                })
            } else {
                warn!("NSID must be alphanumeric only, not {:?}", nsid);
                Err(invalid_identifier_value((TYPE_NAME, s))
            }
        } else {
            warn!(
                "ISIN must have a valid ISO country code as first two characters, not {:?}",
                &s[0..2]
            );
            Err(invalid_identifier_value((TYPE_NAME, s))
        }
    }
}

#[cfg(feature = "urn")]
impl TryFrom<url::Url> for InternationalSecuritiesId {
    type Error = InternationalSecuritiesIdError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        if !value.scheme().eq_ignore_ascii_case("urn") {
            Err(invalid_format(TYPE_NAME, value.scheme()))
        } else {
            let path = value.path();
            if path[0..5].eq_ignore_ascii_case("isin:") {
                LegalEntityId::from_str(&path[4..])
            } else {
                Err(invalid_format(TYPE_NAME, path))
            }
        }
    }
}

#[cfg(feature = "urn")]
impl From<InternationalSecuritiesId> for url::Url {
    fn from(id: InternationalSecuritiesId) -> url::Url {
        url::Url::parse(&format!("urn:isin:{:#}", id)).unwrap()
    }
}

impl InternationalSecuritiesId {
    pub fn country_code(&self) -> &CountryCode {
        &self.country
    }

    pub fn nsid(&self) -> &String {
        &self.nsid
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

fn validate_check_digit(s: &str) -> Result<(), Error> {
    let sum = letters_to_digits(&s[0..2])
        .chain(digits_to_digits(&s[2..11]))
        .enumerate()
        .map(|(i, n)| {
            if i & 1 != 0 {
                n
            } else if n > 4 {
                1 + ((n * 2) - 10)
            } else {
                n
            }
        })
        .sum();
    let check: u8 = (10 - (sum % 10)) % 10;
    if u8::parse(&s[12..]).unwrap() == check {
        Ok(())
    } else {
        warn!("Check digit {:?} invalid, expecting {}", &s[12..], check);
        Err(invalid_identifier_value(TYPE_NAME, s))
    }
}    

#[inline(always)]
fn letters_to_digits(s: &str) -> impl Iterator<Item = u8> {
    const BASE: u8 = 'A' as u8;

    s.chars().flat_map(|c| {
        let n: u8 = (c as u8) - BASE;
        vec![n / 10, n % 10]
    })
}

#[inline(always)]
fn digits_to_digits(s: &str) -> impl Iterator<Item = u8> {
    const BASE: u8 = '0' as u8;

    s.chars().map(|c| (c as u8) - BASE)
}

fn validate_nsid(country: &CountryCode, s: &str) -> Result<String, Error> {
    match nsid::nsid_validate(country, s) {
        Some(false) => {
            warn!(
                "NSID value {:?} is not a valid {}",
                s,
                nsid::nsid_name(country).unwrap()
            );
            Err(invalid_identifier_value((TYPE_NAME, s))
        }
        _ => Ok(s.to_string()),
    }
}

// -----------------------------------------------------------------------