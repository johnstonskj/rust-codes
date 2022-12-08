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
 * `languages`  - Adds the `CountryCode::administrative_language` and `CountryCode::languages` methods (requires package `codes-iso-639`).
* `part_2` - Adds the corresponding module and `SubdivisionCode`.
  * `categories` - Adds the `SubdivisionCode::category_code` method and `SubdivisionCategoryCode` type.
  * `territories` - Adds the `TerritoryCode` type.
  * `languages` - Adds the `SubdivisionCode::name_language` method.

## Refresh

While ISO licenses the 3166 tables freely, access to the whole data tables is via subscription, and so there is not currently a way to download the tables periodically to keep up-to-date.

## Changes

**Version 0.1.1**

* Added part-2.

**Version 0.1.0**

* Initial release, part-1 only.

## TODO

1. Add names in administrative language to parts 1 and 2.
