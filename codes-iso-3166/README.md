#Package codes-iso-3166

This package contains an implementation of the
[ISO 3166](...) specification.


For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

## Example

TBD

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

## Refresh

While ISO licenses the 3166 tables freely, access to the whole data tables is via subscription, and so there is not currently a way to download the tables periodically to keep up-to-date.

## Changes

**Version 0.1.0**

* Initial release, part-1 only.

## TODO

TBD
