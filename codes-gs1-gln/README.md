# Package codes-gs1-gln

This package contains an implementation of the
[GS1 GLN](https://www.gs1.org/standards/id-keys/gln) Global Location Number (GLN) specification.

[![crates.io](https://img.shields.io/crates/v/codes-gs1-gln.svg)](https://crates.io/crates/codes-gs1-gln)
[![docs.rs](https://docs.rs/codes-gs1-gln/badge.svg)](https://docs.rs/codes-gs1-gln)

The Global Location Number (GLN) provides a global supply chain solution by uniquely identifying parties and locations that are involved in business transactions.

The GLN Allocation Rules Standard and contained GLN Management Rules is designed to help industry make consistent decisions about the unique identification of parties and locations in open supply chains. This standard has been developed in accordance with the GS1 Global Standards Management Process (GSMP) and is considered part of the GS1 system of standards. Overall, costs are minimised when all partners in the supply chain adhere to the GLN Allocation Rules Standard.

Unique identification is critical to maintaining operational efficiencies that business partners rely on to exchange information in consistent ways, as well as, ensuring the smooth operations of global supply and value chains. More specifically, the unique identification of parties and locations is critical for efficient logistic operations, traceability programs, recall readiness, and more. It is essential that accurate and up-to-date information on parties and locations is able to be readily shared between business partners to allow the “who” and “where” of business to be reliably answered no matter the use case.

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

## Example

```rust
use codes_gs1_gln::GlobalLocationNumber;
use std::str::FromStr;

let gln = GlobalLocationNumber::from_str("9436465792104").unwrap();

assert_eq!(gln.data(), "943646579210");
assert_eq!(gln.check_digit(), 4);

assert!(!GlobalLocationNumber::is_valid("9436465792109"));
```

```rust
use codes_gs1_gln::GS1_GLN;

assert_eq!(GS1_GLN.title(), "Global Location Number (GLN)");
```

## Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [GlobalLocationNumber] type.

## Changes

**Version 0.1.1**

* Moved check digit code to common package.
* Implemented `Standardized` and `FixedLengthCode` traits.

**Version 0.1.0**

* Initial version

## TODO

TBD
