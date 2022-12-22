# Package codes-common

Support capabilities for `codes-*` packages.

For notes on the design of the API, see the repository 
[README](https://github.com/johnstonskj/rust-codes/blob/main/README.md).

[![crates.io](https://img.shields.io/crates/v/codes-common.svg)](https://crates.io/crates/codes-common)
[![docs.rs](https://docs.rs/codes-common/badge.svg)](https://docs.rs/codes-common)

## Features

* `csv_tools` - Tools for reading and processing CSV files.
* `check_digits` - Algorithms for calculating check digits.

## Changes

**Version 0.1.8**

* Moved check digits to own crate.

**Version 0.1.7**

* Added the `check_digits` module with an implementation of `LuhnAlgorithm`.

**Version 0.1.6**

* Fixed a bug in the insert field macros

**Version 0.1.5**

* Updated CSV handlers
* Added CSV macros

**Version 0.1.4**

* Added `csv_tools` feature

**Version 0.1.3**

* Re-worked `code_impl` macro for non-string codes.

**Version 0.1.2**

* Added `retain` method to `SimpleData`.

**Version 0.1.1**

* Refactored the build process, added `SimpleData` type and `Data` trait.

**Version 0.1.0**

* Initial release, subject to much change.
