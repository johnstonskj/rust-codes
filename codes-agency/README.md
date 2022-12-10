# Crate codes-agency

This package provides a common code representing standards agencies.

[![crates.io](https://img.shields.io/crates/v/codes-agency.svg)](https://crates.io/crates/codes-agency)
[![docs.rs](https://docs.rs/codes-agency/badge.svg)](https://docs.rs/codes-agency)

The two core types, [Agency] and [Standard] work together to provide reporting
capabilities to other *codes* project packages. Specifically a package that
provides types corresponding to a standard definition can have an instance of
the [Standard] struct that describes the standard. This in turn references the
[Agency] that controls the standard.

Consider the following example from the ISO 4217 package.

```rust
use codes_iso_4217::ISO_4217;

assert_eq!(ISO_4217.short_ref(), "4217");
assert_eq!(ISO_4217.title(), "Currency codes");
assert_eq!(ISO_4217.agency().to_string(), String::from("ISO"));
```

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

# Features

By default only the `serde` feature is enabled.

* `serde` - Enables serialization of the [CurrencyCode] type.
  
## Changes

**Version 0.1.6**

* Add the UN as a new Agency.

**Version 0.1.5**

* Added `ALL_CODES` constant.

**Version 0.1.4**

* Clean-up implementation based on newer `codes-common`
* Using new `Code` trait and macro-created implementation.

**Version 0.1.3**

* Added documentation to `Agency` variants.
* Renamed `Error` to be inline with other package naming conventions.

**Version 0.1.2**

* Remove the trait `Code`.
* Remove some feature flags.
* Added documentation.
* Added IANA, IEEE, IETF as agencies.

**Version 0.1.1**

* Initial documentation.

**Version 0.1.0**

* Initial place-holder release.

## TODO

1. Create a build system from agencies.xml
1. Add a `parent_agency` method (ITU is a part of UN)
