# Crate codes-iso-4217

This package contains an implementation of the
[ISO-4217](https://www.iso.org/iso-4217-currency-codes.html)
country code specification.

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

# Example

```rust
use codes_iso_4217::{CurrencyCode, ISO_4217};

let code = CurrencyCode::BZD;

assert_eq!(code.alpha_code(), "BZD");
assert_eq!(code.numeric_code(), Some(84));
assert_eq!(code.currency_name(), "Belize Dollar");
assert_eq!(code.country_name(), "BELIZE");
assert_eq!(code.is_fund(), false);
assert_eq!(code.is_historical(), false);
assert_eq!(code.withdrawal_date(), None);

assert_eq!(ISO_4217.title(), "Currency codes");
```

# Features

By default only the `serde` feature is enabled, the [CurrencyCode::alpha_code] and
[CurrencyCode::numeric_code] methods cannot be excluded.

* `serde` - Enables serialization of the [CurrencyCode] type.
* `currency_name` - Adds the [CurrencyCode::currency_name] method.
* `country_name` - Adds the [CurrencyCode::country_name] method.
* `monetary_units` - Adds the [CurrencyCode::monetary_units] method.
* `is_fund` - Adds the [CurrencyCode::is_fund] method.
* `historical_codes` - Adds the [CurrencyCode::is_historical] and [CurrencyCode::withdrawal_date] methods.

## Changes

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
