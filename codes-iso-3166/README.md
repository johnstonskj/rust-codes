# Package codes-iso-3166

This package contains an implementation of the [ISO 3166](https://www.iso.org/iso-3166-country-codes.html) part 1 standard.

The purpose of ISO 3166 is to define internationally recognized codes of
letters and/or numbers that we can use when we refer to countries and their
subdivisions. However, it does not define the names of countries – this
information comes from United Nations sources (Terminology Bulletin Country
Names and the Country and Region Codes for Statistical Use maintained by the
United Nations Statistics Divisions).

The country codes can be represented either as a two-letter code (alpha-2)
which is recommended as the general-purpose code, a three-letter code
(alpha-3) which is more closely related to the country name and a three-digit
numeric code (numeric-3) which can be useful if you need to avoid using Latin
script.

This package does not include an implementation of part-3, *Code for formerly 
used names of countries*.

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

## Example

This example shows the use of the part-1 Country Code.

```rust
use codes_iso_3166::part_1::CountryCode;

let country = CountryCode::from_str("AG").unwrap();

assert_eq!(country.alpha_2_code(), "AG");
assert_eq!(country.short_name(), "Antigua and Barbuda");
```

## Features

By default only the `serde` feature is enabled, and [part_1] two-letter
language codes.

* `serde` - Enables serialization of the different Language Code types.
* `alpha_3_code` - Adds the `CountryCode::alpha_3_code` method.
* `numeric_code` - Adds the `CountryCode::numeric_code` method.
* `independent` - Adds the `CountryCode::independent` method.
* `status` - Adds the `CountryCode::status` method.
* `full_name` - Adds the `CountryCode::full_name` method.
* `local_names` - Adds the `CountryCode::local_short_name` and
  `CountryCode::local_full_name` methods.
* `languages`  - Adds the `CountryCode::administrative_language` and
  `CountryCode::languages` methods (requires package `codes-iso-639`).
* `formerly` - Adds the `CountryCode::former_short_name` and
  `CountryCode::former_alpha_3_code` methods.
* `part_2` - Adds the corresponding module and `SubdivisionCode`.
  * `categories` - Adds the `SubdivisionCode::category_code` method 
    and `SubdivisionCategoryCode` type.
  * `territories` - Adds the `TerritoryCode` type.
  * `languages` - Adds the `SubdivisionCode::name_language` method.

Note that the method `CountryCode::local_full_name` requires both
`local_names` and `full_name` features.

## Refresh

While ISO licenses the 3166 tables freely, access to the whole data tables is
via subscription, and so there is not currently a way to download the tables
periodically to keep up-to-date.

## Changes

**Version 0.1.2**

* Added `ALL_CODES` constant for all relevant types<35;12;24M.
* Added `indices` modules to part 1 and 2, partial implementations.

**Version 0.1.1**

* Added `formerly_alpha_3_code` and `formerly_short_name` methods to part_1.
* Added `local_short_name` and `local_full_name` methods to part_1.
* Added part-2.

**Version 0.1.0**

* Initial release, part-1 only.

## TODO

1. Add local_name to subdivision categories using administrative language.
1. Build true indices for secondary code lookups.
