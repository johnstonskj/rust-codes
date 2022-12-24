/*!
Provides the [NsinScheme] trait that allows for validation of NSIN values.

# Example

```rust
# let test_code: &str = "263494";
use codes_iso_3166::part_1::CountryCode;
use codes_iso_6166::nsin::{NsinScheme, national_number_scheme_for};

if let Some(nsin) = national_number_scheme_for(&CountryCode::GB) {
    if !nsin.is_valid(test_code) {
        panic!("Not a valid {} (NSIN).", nsin.name());
    }
}
```

*/

use codes_check_digits::{luhn, sedol, Calculator};
use codes_iso_3166::part_1::CountryCode;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Debug};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A National Securities Identifying Number, or NSIN, is a generic
/// nine-digit alphanumeric code which identifies a fungible security.
/// The NSIN is issued by a national numbering agency (NNA) designated
/// for that country. Regional substitute NNAs have been allocated the
/// task of functioning as NNAs in those countries where NNAs have not
/// yet been established. NSINs are used as part of the makeup of a
/// product's ISIN.
///
pub trait NsinScheme: Debug + Sync {
    ///
    /// Return the name of the agency that acts as the National Numbering
    /// Agency (NNA) for this scheme.
    ///
    fn agency_name(&self) -> &'static str;

    ///
    /// Return the name of the NSIN scheme, or data type.
    ///
    fn name(&self) -> &'static str;

    ///
    /// Returns `true` if the provided string is valid for this NSIN
    /// scheme, else `false`.
    ///
    fn is_valid(&self, s: &str) -> bool;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Return an implementation of the [NsinScheme] trait for the provided
/// country code, if one has been registered.
///
#[allow(clippy::borrowed_box)]
pub fn national_number_scheme_for(country: &CountryCode) -> Option<&Box<dyn NsinScheme>> {
    KNOWN_NSID.get(country)
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref KNOWN_NSID: HashMap<CountryCode, Box<dyn NsinScheme>> = known_nsids();
}

///
/// [https://en.wikipedia.org/wiki/CUSIP]
///
#[derive(Debug, Default)]
struct Cusip {}

///
/// [https://en.wikipedia.org/wiki/SEDOL]
///
#[derive(Debug, Default)]
struct Sedol {}

///
/// [https://en.wikipedia.org/wiki/Valoren_number]
///
#[derive(Debug, Default)]
struct Valoren {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl NsinScheme for Cusip {
    fn agency_name(&self) -> &'static str {
        "CUSIP Services Bureau"
    }

    fn name(&self) -> &'static str {
        "CUSIP"
    }

    fn is_valid(&self, s: &str) -> bool {
        assert!(s.len() <= 10);
        luhn::get_algorithm_instance().is_valid(s)
    }
}

// ------------------------------------------------------------------------------------------------

impl NsinScheme for Sedol {
    fn agency_name(&self) -> &'static str {
        "London Stock Exchange"
    }

    fn name(&self) -> &'static str {
        "SEDOL"
    }

    fn is_valid(&self, s: &str) -> bool {
        assert!(s.len() == 7);
        sedol::get_algorithm_instance().is_valid(s)
    }
}

// ------------------------------------------------------------------------------------------------

impl NsinScheme for Valoren {
    fn agency_name(&self) -> &'static str {
        "SIX Financial"
    }

    fn name(&self) -> &'static str {
        "VALOR"
    }

    fn is_valid(&self, s: &str) -> bool {
        s.len() >= 5 && s.len() <= 9 && s.chars().all(|c| c.is_ascii_digit())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn known_nsids() -> HashMap<CountryCode, Box<dyn NsinScheme>> {
    let mut nsids: HashMap<CountryCode, Box<dyn NsinScheme>> = Default::default();
    nsids.insert(CountryCode::CA, Box::<Cusip>::default());
    nsids.insert(CountryCode::CH, Box::<Valoren>::default());
    nsids.insert(CountryCode::UK, Box::<Sedol>::default());
    nsids.insert(CountryCode::US, Box::<Cusip>::default());
    nsids
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sedol_check_digit() {
        let sedol = Sedol::default();
        assert!(NsinScheme::is_valid(&sedol, "0263494"));
    }

    #[test]
    fn test_valoren() {
        let valor = Valoren::default();
        assert!(NsinScheme::is_valid(&valor, "1213853"));
    }
}

#[cfg(feature = "nsin_cusip")]
pub mod cusip;

#[cfg(feature = "nsin_sedol")]
pub mod sedol;
