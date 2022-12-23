# Package codes-iso-4217

This package contains an implementation of the
[ISO 4217](https://www.iso.org/iso-4217-currency-codes.html)
Currency Codes specification.

[![crates.io](https://img.shields.io/crates/v/codes-iso-4217.svg)](https://crates.io/crates/codes-iso-4217)
[![docs.rs](https://docs.rs/codes-iso-4217/badge.svg)](https://docs.rs/codes-iso-4217)

This standard establishes internationally recognized codes for the
representation of currencies that enable clarity and reduce errors. Currencies
are represented both numerically and alphabetically, using either three digits
or three letters. Some of the alphabetic codes for major currencies are
familiar, such as "EUR" for Euros. Fortunately, ISO 4217 covers everything
from Afghanis to Zambian Kwacha as well.

This package extends the data model of the ISO specification by adding a
currency symbol string (and Unicode code points for the symbol) where possible
to all symbols.

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

## Example

```rust
use codes_iso_4217::{CurrencyCode, ISO_4217};

let code = CurrencyCode::BZD;

assert_eq!(code.alpha_code(), "BZD");
assert_eq!(code.numeric_code(), Some(84));

// feature = "currency_name"
assert_eq!(code.currency_name(), "Belize Dollar");

// feature = "country_name"
assert_eq!(code.country_name(), "BELIZE");

// feature = "monetary_units"
assert_eq!(code.monetary_units(), 2);

// feature = "is_fund"
assert_eq!(code.is_fund(), false);

// feature = "historical_codes"
assert_eq!(code.is_historical(), false);
assert_eq!(code.withdrawal_date(), None);

// feature = "symbols"
assert_eq!(code.currency_symbol_str(), Some("BZ$"));
assert_eq!(code.currency_symbol_code_points(), Some(&[0x42, 0x5a, 0x24]));

assert_eq!(ISO_4217.title(), "Currency codes");
```

## Features

By default only the `serde` feature is enabled, the `CurrencyCode::alpha_code` and
`CurrencyCode::numeric_code` methods cannot be excluded.

* `serde` - Enables serialization of the `CurrencyCode` type.
* `currency_name` - Adds the `CurrencyCode::currency_name` method.
* `country_name` - Adds the `CurrencyCode::country_name` method.
* `monetary_units` - Adds the `CurrencyCode::monetary_units` method.
* `is_fund` - Adds the `CurrencyCode::is_fund` method.
* `historical_codes` - Adds the `CurrencyCode::is_historical` and `CurrencyCode::withdrawal_date` methods.
* `symbols` - Adds the `CurrencyCode::currency_symbol_str` and `CurrencyCode::currency_symbol_code_points` methods.

## Changes

**Version 0.1.6**

* Implemented `Standardized` and `FixedLengthCode` traits.

**Version 0.1.5**

* Added symbols, scraped from xe.com
* Synced the examples, with commented out lines for feature-gated methods.

**Version 0.1.4**

* Added `ALL_CODES` constant.

**Version 0.1.3**

* Clean-up implementation based on newer `codes-common`
* Using new `Code` trait and macro-created implementation.

**Version 0.1.2**

* Added more documentation.
* Removed the implementation of the `Code` trait.
* Changed error type to enum from struct.

**Version 0.1.1**

* Completed the build-time work to generate the `CurrencyCode` type and
  methods. Took heavy inspiration from
  [penny](https://github.com/bb010g/penny) but wanted a different resulting style.

**Version 0.1.0**

* Initial place-holder release.

## TODO

1. Create a scheduled build that retrieves updated files from the ISO site.
