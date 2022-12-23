# Package codes-iso-17442

This package contains an implementation of the [ISO
17442](https://www.iso.org/standard/78829.html) Legal Entity Identifier (LEI)
specification.

[![crates.io](https://img.shields.io/crates/v/codes-iso-17442.svg)](https://crates.io/crates/codes-iso-17442)
[![docs.rs](https://docs.rs/codes-iso-17442/badge.svg)](https://docs.rs/codes-iso-17442)

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

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

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

* `serde` - Enables serialization of the `LegalEntityId` type.
* `url` - Enables the conversion between LEI and URL (URN) forms.

## Changes

**Version 0.1.2**

* Moved check digit code to common package
* Implemented `Standardized` and `FixedLengthCode` traits.

**Version 0.1.0**

* Initial release

## TODO

TBD
