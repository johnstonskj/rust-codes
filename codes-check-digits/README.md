# Package codes-check-digits

This package contains implementations of various check digit specifications,
including [ISO/IEC 7064:2003](https://www.iso.org/standard/31531.html)
Information technology — Security techniques — Check character systems.

[![crates.io](https://img.shields.io/crates/v/codes-check-digits.svg)](https://crates.io/crates/codes-check-digits)
[![docs.rs](https://docs.rs/codes-check-digits/badge.svg)](https://docs.rs/codes-check-digits)

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

## Example

```rust
use codes_check_digits::{luhn, Calculator};

let calculator = luhn::get_algorithm_instance();
assert!(calculator.is_valid("US0378331005"));
assert!(calculator.validate("US0378331005").is_ok());
assert_eq!(calculator.calculate("US037833100"), Ok(5));
```

## Features

* `gs1` - Adds the `gs1` module containing algorithms for various codes such as
  EAN, GTIN, GLN, and UPC.
* `iso_7064` - Adds the `iso_7064` module containing implementations of the
  variants defined in ISO/IEC 7064:2003.
* `luhn` - Adds the `luhn` module containing an implementation of the Luhn Algorithm.
* `sedol` - Adds the `sedol` module containing an implementation of the algorithm
  used in SEDOL numbers.

## Changes

**Version 0.1.2**

* Made all NSIN types also Code implementations

**Version 0.1.1**

* rug integers not building on Windows, gated the feature for now.

**Version 0.1.0**

* Initial release, copied from packages `codes-common`, `codes-iso-6166`,
  `codes-iso-17442`, and `codes-gs1-gln`.

## TODO

TBD
