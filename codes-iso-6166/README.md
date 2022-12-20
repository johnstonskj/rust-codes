# Package codes-iso-17442

This package contains an implementation of the [ISO
6166](https://www.iso.org/standard/78502.html) International securities
identification number (ISIN) specification.

[![crates.io](https://img.shields.io/crates/v/codes-iso-6166.svg)](https://crates.io/crates/codes-iso-6166)
[![docs.rs](https://docs.rs/codes-iso-6166/badge.svg)](https://docs.rs/codes-iso-6166

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

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

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
assert!(&format!("{}", bae_systems), "GB0002634946");
assert!(&format!("{:#}", bae_systems), "GB-000263494-6");
```

The following demonstrates the functionality in the `nsin` module which allows
for the creation of validation tools for specific national codes.

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

# Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the `InternationalSecuritiesId` type.
* `url` - Enables the conversion between ISIN and URL (URN) forms.

## Changes

**Version 0.1.0**

* Initial release

## TODO

TBD
