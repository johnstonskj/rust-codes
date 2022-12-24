/*!
This package contains an implementation of the [ISO
6166](https://www.iso.org/standard/78502.html) International securities
identification number (ISIN) specification.

ISO 6166 defines the structure of an International Securities Identification
Number (ISIN). An ISIN uniquely identifies a fungible security.

Securities with which ISINs can be used are:

* Equities (shares, units, depository receipts)
* Debt instruments (bonds and debt instruments other than international,
  international bonds and debt instruments, stripped coupons and principal,
  treasury bills, others)
* Entitlements (rights, warrants)
* Derivatives (options, futures)
* Others (commodities, currencies, indices, interest rates)

ISINs consist of two alphabetic characters, which are the ISO 3166-1 alpha-2
code for the issuing country, nine alpha-numeric characters (the National
Securities Identifying Number, or NSIN, which identifies the security, padded
as necessary with leading zeros), and one numerical check digit. They are thus
always 12 characters in length. When the NSIN changes due to corporate actions
or other reasons, the ISIN will also change. Issuance of ISINs is
decentralized to individual national numbering agencies (NNAs). Since existing
national numbering schemes administered by the various NNAs form the basis for
ISINs, the methodology for assignment is not consistent across agencies
globally.

An ISIN cannot specify a particular trading location. Another identifier,
typically a MIC (Market Identifier Code) or the three-letter exchange code,
will have to be specified in addition to the ISIN for this. The currency of
the trade will also be required to uniquely identify the instrument using this
method.

# Example

The following demonstrates the most common method for constructing an ISIN,
using the standard `FromStr` trait.

```rust
use codes_iso_3166::part_1::CountryCode;
use codes_iso_6166::InternationalSecuritiesId as Isin;
use std::str::FromStr;

let walmart = Isin::from_str("US9311421039").unwrap();
assert_eq!(walmart.country_code(), CountryCode::US);
assert_eq!(walmart.national_number(), "931142103");
assert_eq!(walmart.check_digit(), 9);
```

Alternatively, an ISIN can be constructed from a combination of ISO 3166
country code and an NSIN string. This will calculate and append the ISIN check
digit.

``` rust
use codes_iso_3166::part_1::CountryCode;
use codes_iso_6166::InternationalSecuritiesId as Isin;
use std::str::FromStr;

let bae_systems = Isin::new(CountryCode::GB, "263494").unwrap();
assert_eq!(&format!("{}", bae_systems), "GB0002634946");
assert_eq!(&format!("{:#}", bae_systems), "GB-000263494-6");
```

# Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [InternationalSecuritiesId] type.
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

use codes_agency::{standardized_type, Agency, Standard};
use codes_check_digits::{luhn, Calculator};
use codes_common::{fixed_length_code, invalid_format, invalid_length, Code};
use codes_iso_3166::part_1::CountryCode;
use std::{fmt::Display, fmt::Formatter, str::FromStr};
use tracing::warn;

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
/// The ISO 6166 International Securities Identification
/// Number ([ISIN](https://www.isin.org))
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct InternationalSecuritiesId {
    country: CountryCode,
    // National Securities Identifying Number (NSIN)
    nsin: String,
    check_digit: u8,
}

pub use codes_common::CodeParseError as InternationalSecuritiesIdError;

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
                format!("{}-{:0>9}-{}", self.country, &self.nsin, &self.check_digit)
            } else {
                format!("{}{:0>9}{}", self.country, self.nsin, self.check_digit)
            }
        )
    }
}

impl From<InternationalSecuritiesId> for String {
    fn from(v: InternationalSecuritiesId) -> String {
        v.to_string()
    }
}

impl FromStr for InternationalSecuritiesId {
    type Err = InternationalSecuritiesIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 14 {
            Self::from_str(&s.replace('-', ""))
        } else if s.len() != 12 {
            warn!("ISIN must be 12 characters long, not {}", s.len());
            Err(invalid_length(TYPE_NAME, s.len()))
        } else if let Ok(country_code) = CountryCode::from_str(&s[0..2]) {
            let cd_calc = luhn::get_algorithm_instance();
            cd_calc.validate(s)?;
            let nsid = &s[2..11];
            Ok(InternationalSecuritiesId {
                country: country_code,
                nsin: validate_nsin(&country_code, nsid)?,
                check_digit: u8::from_str(&s[11..]).map_err(|_| invalid_format(TYPE_NAME, s))?,
            })
        } else {
            warn!(
                "ISIN must have a valid ISO country code as first two characters, not {:?}",
                &s[0..2]
            );
            Err(invalid_format(TYPE_NAME, s))
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
                InternationalSecuritiesId::from_str(&path[4..])
            } else {
                warn!("URN authority is not ISIN");
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

impl Code<String> for InternationalSecuritiesId {}

fixed_length_code!(InternationalSecuritiesId, 12);

standardized_type!(InternationalSecuritiesId, ISO_6166);

impl InternationalSecuritiesId {
    ///
    /// Construct a new ISIN from country code and NSIN. This will
    /// check the NSIN for validity if possible, and calculate the
    /// ISIN check digit.
    ///
    pub fn new(country: CountryCode, nsin: &str) -> Result<Self, InternationalSecuritiesIdError> {
        let cd_calc = luhn::get_algorithm_instance();
        let check_digit = cd_calc.calculate(&format!("{}{:0>9}", country, nsin))?;
        Ok(Self {
            country,
            nsin: nsin.to_string(),
            check_digit,
        })
    }

    ///
    /// Return the country code of this ISIN.
    ///
    pub fn country_code(&self) -> CountryCode {
        self.country
    }

    ///
    /// Return the NSIN portion of this ISIN.
    ///
    pub fn national_number(&self) -> &String {
        &self.nsin
    }

    ///
    /// Return the check digit of this ISIN.
    ///
    pub fn check_digit(&self) -> u8 {
        self.check_digit
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

///
/// Wrapper to fetch an NSIN scheme for the provided country code
/// and, if one exists, validate it.
///
fn validate_nsin(country: &CountryCode, s: &str) -> Result<String, InternationalSecuritiesIdError> {
    if let Some(nsin) = nsin::national_number_scheme_for(country) {
        if nsin.is_valid(s) {
            Ok(s.to_string())
        } else {
            warn!("NSID value {:?} is not a valid {}", s, nsin.name());
            Err(invalid_format(nsin.name(), s))
        }
    } else {
        Ok(s.to_string())
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod nsin;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    /*
    use pretty_assertions::assert_eq;
     */
    use super::*;

    #[test]
    fn test_display_formatting() {
        // Walmart
        let isin = InternationalSecuritiesId::from_str("US9311421039").unwrap();
        assert_eq!(format!("{}", isin), "US9311421039".to_string());
        assert_eq!(format!("{:#}", isin), "US-931142103-9".to_string());
    }

    #[test]
    fn test_from_str() {
        let isin = InternationalSecuritiesId::from_str("US9311421039").unwrap();
        assert_eq!(format!("{}", isin), "US9311421039".to_string());

        let isin = InternationalSecuritiesId::from_str("US-931142103-9").unwrap();
        assert_eq!(format!("{}", isin), "US9311421039".to_string());
    }

    #[test]
    fn test_us_cusip() {
        // Apple
        let isin = InternationalSecuritiesId::new(CountryCode::US, "37833100").unwrap();
        assert_eq!(format!("{}", isin), "US0378331005".to_string());
        assert_eq!(format!("{:#}", isin), "US-037833100-5".to_string());
    }

    #[test]
    fn test_swiss_valor() {
        // Credit Suisse
        let isin = InternationalSecuritiesId::new(CountryCode::CH, "1213853").unwrap();
        assert_eq!(format!("{}", isin), "CH0012138530".to_string());
        assert_eq!(format!("{:#}", isin), "CH-001213853-0".to_string());
    }

    #[test]
    fn test_uk_sedol() {
        // BAE Systems
        let isin = InternationalSecuritiesId::new(CountryCode::GB, "263494").unwrap();
        assert_eq!(format!("{}", isin), "GB0002634946".to_string());
        assert_eq!(format!("{:#}", isin), "GB-000263494-6".to_string());
    }

    #[test]
    fn test_australia() {
        // Treasury Corporation
        let isin = InternationalSecuritiesId::new(CountryCode::AU, "XVGZA").unwrap();
        assert_eq!(format!("{}", isin), "AU0000XVGZA3".to_string());
        assert_eq!(format!("{:#}", isin), "AU-0000XVGZA-3".to_string());
    }

    #[test]
    fn test_japan() {
        let isin = InternationalSecuritiesId::new(CountryCode::JP, "K0VF05").unwrap();
        assert_eq!(format!("{}", isin), "JP000K0VF055".to_string());
        assert_eq!(format!("{:#}", isin), "JP-000K0VF05-5".to_string());
    }
}
